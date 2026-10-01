//! This module exposes the legacy phases written in OCaml in the OCaml engine.

use crate::{
    ast::Item,
    phase::{Phase, PhaseKind},
};

/// Group consecutive ocaml phases as one monolithic phase, so that we avoid extra roundtrips to the OCaml engine.
pub fn group_consecutive_ocaml_phases(phases: Vec<PhaseKind>) -> Vec<Box<dyn Phase>> {
    let mut output: Vec<Box<dyn Phase>> = vec![];
    let mut ocaml_phases = vec![];
    let mut phases = phases.into_iter();

    struct LegacyOCamlPhases {
        phases: Vec<LegacyOCamlPhase>,
    }

    impl Phase for LegacyOCamlPhases {
        fn apply(&self, items: &mut Vec<Item>) {
            apply_legacy_phases(&self.phases, items);
        }
    }

    loop {
        let phase = phases.next();
        if let Some(PhaseKind::Legacy(ocaml_phase)) = phase {
            ocaml_phases.push(ocaml_phase)
        } else {
            if !ocaml_phases.is_empty() {
                output.push(Box::new(LegacyOCamlPhases {
                    phases: std::mem::take(&mut ocaml_phases),
                }));
            }
            if let Some(phase) = phase {
                output.push(Box::new(phase));
            } else {
                break;
            }
        }
    }

    output
}

fn apply_legacy_phases(phases: &[LegacyOCamlPhase], items: &mut Vec<Item>) {
    use crate::ocaml_engine::Response;
    let query = crate::ocaml_engine::QueryKind::ApplyPhases {
        input: std::mem::take(items),
        phases: phases.iter().map(ToString::to_string).collect(),
    };
    let Some(Response::ApplyPhases {
        output,
        diagnostics,
    }) = query.execute(None)
    else {
        panic!()
    };
    for diagnostic in diagnostics {
        crate::hax_io::report_diagnostic(diagnostic);
    }
    *items = output;
}

macro_rules! make_ocaml_legacy_phase {
    ($($name:ident),*) => {

        pastey::paste!{
            /// The list of exposed OCaml phases.
            #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
            pub enum LegacyOCamlPhase {
                $(
                    #[doc = concat!("The phase ", stringify!($name), " from the OCaml engine.")]
                    [< $name:camel >]
                ),*
            }


            impl std::fmt::Display for LegacyOCamlPhase {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(Self::[< $name:camel >] => stringify!($name).fmt(f)),*
                    }
                }
            }

            impl Phase for LegacyOCamlPhase {
                fn apply(&self, items: &mut Vec<Item>) {
                    apply_legacy_phases(&[*self], items);
                }
            }
        }
    };
}

impl From<LegacyOCamlPhase> for PhaseKind {
    fn from(legacy_phase: LegacyOCamlPhase) -> Self {
        Self::Legacy(legacy_phase)
    }
}

make_ocaml_legacy_phase!(
    and_mut_defsite,
    bundle_cycles,
    cf_into_monads,
    direct_and_mut,
    drop_blocks,
    drop_match_guards,
    drop_references,
    drop_return_break_continue,
    drop_sized_trait,
    explicit_conversions,
    functionalize_loops,
    hoist_disjunctive_patterns,
    local_mutation,
    newtype_as_refinement,
    reconstruct_asserts,
    reconstruct_for_index_loops,
    reconstruct_for_loops,
    reconstruct_question_marks,
    reconstruct_while_loops,
    reorder_fields,
    rewrite_control_flow,
    rewrite_local_self,
    simplify_hoisting,
    simplify_match_return,
    simplify_question_marks,
    sort_items,
    sort_items_namespace_wise,
    specialize,
    traits_specs,
    transform_hax_lib_inline,
    trivialize_assign_lhs,
    reject_arbitrary_lhs,
    reject_continue,
    reject_question_mark,
    reject_raw_or_mut_pointer,
    reject_early_exit,
    reject_as_pattern,
    reject_dyn,
    reject_trait_item_default,
    reject_unsafe,
    reject_impl_type_method,
    hoist_side_effects
);
