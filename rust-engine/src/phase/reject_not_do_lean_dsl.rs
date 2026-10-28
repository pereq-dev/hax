use crate::ast::*;
use crate::ast::{diagnostics::*, visitors::*};
use crate::phase::Phase;

/// Rejection Phase for patterns unsupported by Lean's do-notation DSL
///
/// This phase rejects unsupported interleavings of expressions and statements.
/// It is built as a visitor.
#[derive(Default)]
pub struct RejectNotDoLeanDSL;

/// Expressions are either do-statements or do-expressions. The former can be downgraded into the
/// latter.
#[derive(Clone, Copy, Debug)]
enum DoDSLExprKind {
    Statement,
    Expression,
}

/// Gives the "kind" of an expression in the do-notation DSL
fn dsl_expr_kind(expr_kind: &ExprKind) -> DoDSLExprKind {
    match expr_kind {
        ExprKind::If { .. } | ExprKind::Match { .. } | ExprKind::Let { .. } => {
            DoDSLExprKind::Statement
        }
        _ => DoDSLExprKind::Expression,
    }
}

/// The default value for entry points of expression (function items, function impl items)
impl Default for DoDSLExprKind {
    fn default() -> Self {
        Self::Statement
    }
}

/// Visitor internal state
#[setup_error_handling_struct]
#[derive(Default)]
pub struct RejectNotDoLeanDSLVisitor {
    /// Expected kind for the visited expression. Used by `visit_expr`, ignored by other methods
    dsl_expr_kind: DoDSLExprKind,
}

impl VisitorWithContext for RejectNotDoLeanDSLVisitor {
    fn context(&self) -> Context {
        Context::Phase(stringify!(RejectNotDoLeanDSL).to_string())
    }
}

impl AstVisitorMut for RejectNotDoLeanDSLVisitor {
    setup_error_handling_impl!();

    fn visit_expr(&mut self, expr: &mut Expr) {
        use DoDSLExprKind::*;
        let parent_dsl_expr_kind = self.dsl_expr_kind;
        self.dsl_expr_kind = match (self.dsl_expr_kind, dsl_expr_kind(&expr.kind)) {
            // A do-expression cannot be upgraded to a do-statement, we throw an error
            (Expression, Statement) => {
                self.error(
                    expr.clone(),
                    DiagnosticInfoKind::ExplicitRejection {
                        reason: "This interleaving of expression and statements does not fit in Lean's do-notation DSL.\
                                 \nYou may try hoisting out let-bindings and control-flow.".to_string(),
                        issue_id: Some(1741),
                    },
                );
                Statement
            }
            // Closures body are do-statement, as a `do` keyword is introduced
            (_, _) if matches!(&*expr.kind, ExprKind::Closure { .. }) => Statement,
            // In other cases, we keep the computed kind
            (_, kind) => kind,
        };
        self.visit_inner(expr);
        self.dsl_expr_kind = parent_dsl_expr_kind;
    }

    /// Visitor for types. Array lengths can be any (const) expression, so they are checked for dsl
    /// patterns (as DoDSL-expressions)
    fn visit_ty(&mut self, ty: &mut Ty) {
        if let TyKind::Array { length, .. } = ty.kind_mut() {
            // The Lean Backend does not support computation in array lengths yet.  It should be
            // possible to have do-blocks, and treat them like constants. See
            // https://github.com/pereq-dev/hax/issues/1713
            let parent_dsl_expr_kind = self.dsl_expr_kind;
            self.dsl_expr_kind = DoDSLExprKind::Expression;
            self.visit_inner(&mut *length);
            self.dsl_expr_kind = parent_dsl_expr_kind;
        }
    }
}

impl Phase for RejectNotDoLeanDSL {
    fn apply(&self, items: &mut Vec<Item>) {
        // Entry points are statements
        RejectNotDoLeanDSLVisitor::default().visit(items)
    }
}
