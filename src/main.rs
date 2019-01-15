#![feature(box_syntax)]
#![feature(rustc_private)]

extern crate rustc;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate syntax;

use rustc::{declare_lint, lint_array};
use rustc::lint::*;
use rustc_driver::driver;

#[macro_use]
mod macros;
mod matchers;
mod pattern_tree;
mod repeat;
mod ast_match;


declare_lint! {
    pub SIMPLE_PATTERN,
    Forbid,
    "simple pattern lint"
}

pub struct SimplePattern;

impl LintPass for SimplePattern {
    fn get_lints(&self) -> LintArray {
        lint_array!(SIMPLE_PATTERN)
    }
}

impl EarlyLintPass for SimplePattern {
    fn check_expr(&mut self, cx: &EarlyContext, expr: &syntax::ast::Expr) {
        
        use crate::pattern_tree::Expr::*;
        use crate::pattern_tree::Lit::*;
        use crate::pattern_tree::Ty::*;
        use crate::pattern_tree::Stmt::*;
        use crate::matchers::IsMatch;
        
        let pattern = any!(
            Lit(
                any!(
                    Bool(any!(false))
                )
            ),
            Array(
                seq!(
                    any!(Lit(any!(Char(any!('a'))))); ..,
                    any!(Lit(any!(Char(any!('b'))))); 1..=3,
                    any!(Lit(any!(Char(any!('c'))))); 1
                )
            ),
            Cast(
                any!(
                    Lit(any!(Int(any!(0))))
                ),
                any!(
                    Ptr(any!(Path(seq!(any!("i32".to_string()); 1))), any!(syntax::ast::Mutability::Immutable))
                )
            ),
            If(any!(Lit(any!(Bool(any!(true))))), seq!(
                any!(Expr(any!(Lit(any!(Int(any!())))))); ..,
                any!(Semi(any!(Lit(any!(Bool(any!())))))); ..
            ), any!())
        );

        if pattern.is_match(expr) {
            cx.span_lint(
                SIMPLE_PATTERN,
                expr.span,
                "This is a match for a simple pattern. Well Done!",
            );
        }
    }
}

pub fn main() {
    let args: Vec<_> = std::env::args().collect();
    rustc_driver::run(move || {
        let mut compiler = driver::CompileController::basic();
        compiler.after_parse.callback = Box::new(move |state| {
            let mut ls = state.session.lint_store.borrow_mut();
            ls.register_early_pass(None, false, box SimplePattern);
        });
        rustc_driver::run_compiler(&args, Box::new(compiler), None, None)
    });
}
