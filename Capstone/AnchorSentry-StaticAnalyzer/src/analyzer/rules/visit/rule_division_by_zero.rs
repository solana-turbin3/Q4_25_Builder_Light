use syn::visit::{self, Visit};
use syn::spanned::Spanned;
use syn::{Expr, ExprBinary, ExprPath};
use crate::state::{PossibleDivisionByZeroFindingCheckerInstance};
use crate::report::report::*;
use crate::report::knowledge_base::{Finding, PossibleDivisionByZeroFinding, RULE_POSSIBLE_DIVISION_BY_ZERO};

pub struct PossibleDivisionByZeroChecker {
    pub instance: Vec<PossibleDivisionByZeroFindingCheckerInstance>,
    pub current_fn: String
}

impl<'ast> Visit<'ast> for PossibleDivisionByZeroChecker {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.current_fn = node.sig.ident.to_string();

        syn::visit::visit_item_fn(self, node);
    }

    fn visit_expr_binary(&mut self, expr: &'ast ExprBinary) {
        // Check if this is a division: left / right
        if matches!(expr.op, syn::BinOp::Div(_)) {
            // expr.right is Box<Expr> -> deref before matching
            let divisor_expr: &Expr = &*expr.right;

            // We only care about the simple case: "some_var" (Expr::Path with single ident)
            if let Expr::Path(ExprPath { path, .. }) = divisor_expr {
                if let Some(ident) = path.get_ident() {
                    // don't attempt to get line/col yet; just include span debug for context
                    let line_number = expr.span().start().line;
                    self.instance.push(PossibleDivisionByZeroFindingCheckerInstance {
                        function_name: self.current_fn.clone(),
                        divisor: ident.to_string(),
                        line: line_number
                    });
                }
            }
        }

        // continue walking inside this binary expression
        visit::visit_expr_binary(self, expr);
    }
}

pub fn rules_division_by_variable(ast: &syn::File, r: &mut Report) {
    let mut checker = PossibleDivisionByZeroChecker { instance: Vec::new(), current_fn: String::new(), };
    checker.visit_file(ast);
    for p in checker.instance {
        // println!("Possible division by 0 : {:?}", p);
        r.add(Finding::PossibleDivisionByZero(PossibleDivisionByZeroFinding {
            rule: &RULE_POSSIBLE_DIVISION_BY_ZERO,
            fn_name : p.function_name,
            line: p.line,
            divisor: p.divisor       
        }))
    }
}