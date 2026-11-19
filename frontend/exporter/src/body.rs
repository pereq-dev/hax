pub use module::*;

#[cfg(not(feature = "rustc"))]
mod module {
    pub trait IsBody: Sized + Clone + 'static {}
    impl<T: Sized + Clone + 'static> IsBody for T {}
}

#[cfg(feature = "rustc")]
mod module {
    pub use crate::prelude::*;
    pub use rustc_hir::{
        def_id::{DefId as RDefId, LocalDefId as RLocalDefId},
        hir_id::OwnerId as ROwnerId,
    };
    use rustc_middle::ty;

    mod store {
        //! This module helps at store bodies to avoid stealing.
        //! `rustc_data_structures::steal::Steal` is a box for which the content can be stolen, for performance reasons.
        //! The query system of Rust creates and steal such boxes, resulting in hax trying to borrow the value of a Steal while some query stole it already.
        //! This module provides an ad-hoc global cache and query overrides to deal with this issue.
        use rustc_hir::def_id::LocalDefId;
        use rustc_middle::mir::Body;
        use rustc_middle::query::plumbing::IntoQueryParam;
        use rustc_middle::thir::{ExprId, Thir};
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::rc::Rc;

        thread_local! {
            static THIR_BODY: RefCell<HashMap<LocalDefId, (Rc<Thir<'static>>, ExprId)>> = RefCell::new(HashMap::new());
            static MIR_BUILT: RefCell<HashMap<LocalDefId, Rc<Body<'static>>>> = RefCell::new(HashMap::new());
        }

        /// Register overrides for rustc queries.
        /// This will clone and store bodies for THIR and MIR (built) in an ad-hoc global cache.
        pub fn override_queries_store_body(providers: &mut rustc_middle::query::Providers) {
            providers.thir_body = |tcx, def_id| {
                let (steal, expr_id) =
                    (rustc_interface::DEFAULT_QUERY_PROVIDERS.thir_body)(tcx, def_id)?;
                let body = steal.borrow().clone();
                let body: Thir<'static> = unsafe { std::mem::transmute(body) };
                THIR_BODY.with(|map| map.borrow_mut().insert(def_id, (Rc::new(body), expr_id)));
                Ok((steal, expr_id))
            };
            providers.mir_built = |tcx, def_id| {
                let steal = (rustc_interface::DEFAULT_QUERY_PROVIDERS.mir_built)(tcx, def_id);
                let body = steal.borrow().clone();
                let body: Body<'static> = unsafe { std::mem::transmute(body) };
                MIR_BUILT.with(|map| map.borrow_mut().insert(def_id, Rc::new(body)));
                steal
            };
        }

        /// Extension trait that provides non-stealing variants of `thir_body` and `mir_built`.
        /// Those methods requires rustc queries to be overridden with the helper function `register` above.
        #[extension_traits::extension(pub trait SafeTyCtxtBodies)]
        impl<'tcx> rustc_middle::ty::TyCtxt<'tcx> {
            fn thir_body_safe(
                &self,
                key: impl IntoQueryParam<rustc_span::def_id::LocalDefId>,
            ) -> Result<(Rc<Thir<'tcx>>, ExprId), rustc_span::ErrorGuaranteed> {
                let key = key.into_query_param();
                if !THIR_BODY.with(|map| map.borrow().contains_key(&key)) {
                    // Compute a body, which will insert a body in `THIR_BODIES`.
                    let _ = self.thir_body(key);
                }
                THIR_BODY.with(|map| {
                    let (body, expr) = map
                        .borrow_mut()
                        .get(&key)
                        .expect("Did we forgot to call `register`?")
                        .clone();
                    let body: Rc<Thir<'tcx>> = unsafe { std::mem::transmute(body) };
                    Ok((body, expr))
                })
            }
            fn mir_built_safe(
                &self,
                key: impl IntoQueryParam<rustc_span::def_id::LocalDefId>,
            ) -> Rc<Body<'tcx>> {
                let key = key.into_query_param();
                if !MIR_BUILT.with(|map| map.borrow().contains_key(&key)) {
                    // Compute a body, which will insert a body in `MIR_BODIES`.
                    let _ = self.mir_built(key);
                }
                MIR_BUILT.with(|map| {
                    let body = map
                        .borrow_mut()
                        .get(&key)
                        .expect("Did we forgot to call `register`?")
                        .clone();
                    unsafe { std::mem::transmute(body) }
                })
            }
        }
    }
    pub use store::*;

    pub fn get_thir<'tcx, S: BaseState<'tcx>>(
        did: RLocalDefId,
        s: &S,
    ) -> (
        Rc<rustc_middle::thir::Thir<'tcx>>,
        rustc_middle::thir::ExprId,
    ) {
        let tcx = s.base().tcx;

        // The `type_of` anon constants isn't available directly, it needs to be fed by some
        // other query. This hack ensures this happens, otherwise `thir_body` returns an error.
        // See https://rust-lang.zulipchat.com/#narrow/channel/182449-t-compiler.2Fhelp/topic/Change.20in.20THIR.20of.20anonymous.20constants.3F/near/509764021 .
        let hir_id = tcx.local_def_id_to_hir_id(did);
        for (parent_id, parent) in tcx.hir_parent_iter(hir_id) {
            if let rustc_hir::Node::Item(..) = parent {
                let _ = tcx.check_well_formed(parent_id.owner.def_id);
                break;
            }
        }

        let msg = |_| fatal!(s[tcx.def_span(did)], "THIR not found for {:?}", did);
        tcx.thir_body_safe(did).as_ref().unwrap_or_else(msg).clone()
    }

    pub trait IsBody:
        Sized + std::fmt::Debug + Clone + std::any::Any + Send + Sync + 'static
    {
        fn body<'tcx, S: UnderOwnerState<'tcx>>(
            s: &S,
            did: RDefId,
            instantiate: Option<ty::GenericArgsRef<'tcx>>,
        ) -> Option<Self>;

        /// Reuse a MIR body we already got. Panic if that's impossible.
        fn from_mir<'tcx, S: UnderOwnerState<'tcx>>(
            _s: &S,
            _body: rustc_middle::mir::Body<'tcx>,
        ) -> Option<Self> {
            None
        }
    }

    pub fn make_fn_def<'tcx, Body: IsBody, S: BaseState<'tcx>>(
        fn_sig: &rustc_hir::FnSig,
        body_id: &rustc_hir::BodyId,
        s: &S,
    ) -> FnDef<Body> {
        let hir_id = body_id.hir_id;
        let ldid = hir_id.owner.def_id;

        let (thir, expr_entrypoint) = get_thir(ldid, s);
        let s = &s.with_owner_id(ldid.to_def_id()).with_thir(thir.clone());
        FnDef {
            params: thir.params.raw.sinto(s),
            ret: thir.exprs[expr_entrypoint].ty.sinto(s),
            body: Body::body(s, ldid.to_def_id(), None).s_unwrap(s),
            sig_span: fn_sig.span.sinto(s),
            header: fn_sig.header.sinto(s),
        }
    }

    pub fn body_from_id<'tcx, Body: IsBody, S: UnderOwnerState<'tcx>>(
        id: rustc_hir::BodyId,
        s: &S,
    ) -> Body {
        // **Important:**
        // We need a local id here, and we get it from the owner id, which must
        // be local. It is safe to do so, because if we have access to HIR objects,
        // it necessarily means we are exploring a local item (we don't have
        // access to the HIR of external objects, only their MIR).
        Body::body(s, s.base().tcx.hir_body_owner_def_id(id).to_def_id(), None).s_unwrap(s)
    }

    mod implementations {
        use super::*;
        impl IsBody for () {
            fn body<'tcx, S: UnderOwnerState<'tcx>>(
                _s: &S,
                _did: RDefId,
                _instantiate: Option<ty::GenericArgsRef<'tcx>>,
            ) -> Option<Self> {
                Some(())
            }
            fn from_mir<'tcx, S: UnderOwnerState<'tcx>>(
                _s: &S,
                _body: rustc_middle::mir::Body<'tcx>,
            ) -> Option<Self> {
                Some(())
            }
        }
        impl IsBody for ThirBody {
            fn body<'tcx, S: BaseState<'tcx>>(
                s: &S,
                did: RDefId,
                instantiate: Option<ty::GenericArgsRef<'tcx>>,
            ) -> Option<Self> {
                let did = did.as_local()?;
                // The following returns `None` if did refers to something that has no body (avoids a crash in the call to `thir_body`)
                s.base().tcx.hir_maybe_body_owned_by(did)?;
                let (thir, expr) = get_thir(did, s);
                assert!(instantiate.is_none(), "monomorphized thir isn't supported");
                let s = &s.with_owner_id(did.to_def_id()).with_thir(thir.clone());
                let params = thir.params.raw.sinto(s);
                let expr = if *CORE_EXTRACTION_MODE {
                    let expr = &thir.exprs[expr];
                    Decorated {
                        contents: Box::new(ExprKind::Tuple { fields: vec![] }),
                        hir_id: None,
                        attributes: vec![],
                        ty: expr.ty.sinto(s),
                        span: expr.span.sinto(s),
                    }
                } else {
                    expr.sinto(&s.with_thir(thir))
                };
                Some(Self { expr, params })
            }
        }

        impl<A: IsBody, B: IsBody> IsBody for (A, B) {
            fn body<'tcx, S: UnderOwnerState<'tcx>>(
                s: &S,
                did: RDefId,
                instantiate: Option<ty::GenericArgsRef<'tcx>>,
            ) -> Option<Self> {
                Some((A::body(s, did, instantiate)?, B::body(s, did, instantiate)?))
            }
        }

        impl<MirKind: IsMirKind + Clone + 'static> IsBody for MirBody<MirKind> {
            fn body<'tcx, S: UnderOwnerState<'tcx>>(
                s: &S,
                did: RDefId,
                instantiate: Option<ty::GenericArgsRef<'tcx>>,
            ) -> Option<Self> {
                let tcx = s.base().tcx;
                let typing_env = s.typing_env();
                MirKind::get_mir(tcx, did, |body| {
                    let body = substitute(tcx, typing_env, instantiate, body.clone());
                    let body = Rc::new(body);
                    body.sinto(&s.with_mir(body.clone()))
                })
            }
            fn from_mir<'tcx, S: UnderOwnerState<'tcx>>(
                s: &S,
                body: rustc_middle::mir::Body<'tcx>,
            ) -> Option<Self> {
                let body = Rc::new(body.clone());
                let s = &s.with_mir(body.clone());
                Some(body.sinto(s))
            }
        }
    }

    impl<'tcx, S: UnderOwnerState<'tcx>, Body: IsBody> SInto<S, Body> for rustc_hir::BodyId {
        fn sinto(&self, s: &S) -> Body {
            body_from_id::<Body, _>(*self, s)
        }
    }
}
