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
    pub COLLAPSIBLE_IF,
    Forbid,
    "`if`s that can be collapsed (e.g. `if x { if y { ... } }` and `else { if x { ... } }`)"
}

pub struct CollapsibleIf;

impl LintPass for CollapsibleIf {
    fn get_lints(&self) -> LintArray {
        lint_array!(COLLAPSIBLE_IF)
    }
}

impl EarlyLintPass for CollapsibleIf {
    fn check_expr(&mut self, cx: &EarlyContext, expr: &syntax::ast::Expr) {
        use crate::pattern_tree::Expr::*;
        use crate::pattern_tree::Stmt::*;
        use crate::matchers::IsMatch;
        
        let if_or_if_let: matchers::Alt<pattern_tree::Expr> = any!(
            If(any!(), any!(), any!()),
            IfLet(any!(), any!())
        );

        let if_or_if_let_block: matchers::Opt<pattern_tree::Expr> = any!(opt!(any!(
            Block(any!(seq!(
                any!(
                    Expr(if_or_if_let.clone()),
                    Semi(if_or_if_let)
                ); 1
            )))))
        );

        let pattern: matchers::Alt<pattern_tree::Expr> = any!(
            // If without else clause
            If(any!(), any!(seq!(
                any!(
                    Expr(any!(If(any!(), any!(), any!(opt!())))),
                    Semi(any!(If(any!(), any!(), any!(opt!()))))
                ); 1
            )), any!(opt!()))
        );

        let pattern2: matchers::Alt<pattern_tree::Expr> = any!(
            // If with else clause
            If(any!(), any!(), if_or_if_let_block.clone()),
            // IfLet with else clause
            IfLet(any!(), if_or_if_let_block)
        );

        if pattern.is_match(expr) {
            cx.span_lint(
                SIMPLE_PATTERN,
                expr.span,
                "this if statement can be collapsed",
            );
        }
        if pattern2.is_match(expr) {
            match &expr.node {
                syntax::ast::ExprKind::If(_, _, Some(else_)) | syntax::ast::ExprKind::IfLet(_, _, _, Some(else_)) => {
                    cx.span_lint(
                        SIMPLE_PATTERN,
                        else_.span,
                        "this `else { if .. }` block can be collapsed",
                    );
                },
                _ => ()
            }
        }
    }
}


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
        use crate::pattern_tree::Stmt::*;
        use crate::matchers::IsMatch;
        
        let pattern: matchers::Alt<pattern_tree::Expr> = any!(
            Lit(
                any!(
                    Bool(any!(false))
                )
            ),
            Array(
                any!(seq!(
                    any!(Lit(any!(Char(any!('a'))))); ..,
                    any!(Lit(any!(Char(any!('b'))))); 1..=3,
                    any!(Lit(any!(Char(any!('c'))))); 1
                ))
            ),
            If(any!(Lit(any!(Bool(any!(true))))), any!(seq!(
                any!(Expr(any!(Lit(any!(Int(any!())))))); ..,
                any!(Semi(any!(Lit(any!(Bool(any!())))))); ..
            )), any!())
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
            ls.register_early_pass(None, false, box CollapsibleIf);
        });
        rustc_driver::run_compiler(&args, Box::new(compiler), None, None)
    });
}
