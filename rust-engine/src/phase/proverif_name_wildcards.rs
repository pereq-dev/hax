//! ProVerif-backend phase: name every wildcard pattern.
//!
//! ProVerif binders must be distinct: two `_`s in one `letfun` header are the
//! hard error `variable wildcard already defined`, and two in one pattern
//! silently rebind (the second wins). So every `_` becomes a binder `wildcard`,
//! `wildcard_1`, …, numbered per function. So does a field-less struct pattern
//! such as `()`, since a constructor declared as a `const` cannot appear in a
//! ProVerif pattern. A wildcard arm keeps its `_`: the printer renders it as
//! the fallback of the arm chain.

use crate::ast::visitors::*;
use crate::ast::*;
use crate::phase::Phase;

/// Phase naming wildcard patterns.
#[derive(Default, Debug)]
pub struct ProverifNameWildcards;

struct NameWildcards {
    /// Counter numbering the wildcards of the current function.
    next: usize,
}

impl AstVisitorMut for NameWildcards {
    fn visit_arm(&mut self, arm: &mut Arm) {
        if matches!(&*arm.pat.kind, PatKind::Wild) {
            self.visit_expr(&mut arm.body)
        } else {
            self.visit_inner(arm)
        }
    }

    fn visit_pat(&mut self, pat: &mut Pat) {
        let is_unit = matches!(
            &*pat.kind,
            PatKind::Construct { is_struct: true, fields, .. } if fields.is_empty()
        );
        if !(is_unit || matches!(&*pat.kind, PatKind::Wild)) {
            return self.visit_inner(pat);
        }
        let name = match self.next {
            0 => "wildcard".to_string(),
            n => format!("wildcard_{n}"),
        };
        self.next += 1;
        *pat.kind = PatKind::var_pat(name.as_str().into());
    }
}

impl Phase for ProverifNameWildcards {
    fn apply(&self, items: &mut Vec<Item>) {
        let mut visitor = NameWildcards { next: 0 };
        let mut name = |params: &mut [Param], body: &mut Expr| {
            visitor.next = 0;
            for param in params {
                visitor.visit_pat(&mut param.pat);
            }
            visitor.visit_expr(body);
        };
        for item in items.iter_mut() {
            match &mut item.kind {
                ItemKind::Fn { params, body, .. } => name(params, body),
                ItemKind::Impl { items, .. } => {
                    for impl_item in items {
                        if let ImplItemKind::Fn { params, body } = &mut impl_item.kind {
                            name(params, body)
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
