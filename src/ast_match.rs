use pattern_tree::*;
use pattern_tree::matchers::*;
use crate::matchers::IsMatch;
use crate::matchers::PatternTreeNode;
use syntax::ast;

#[derive(Debug)]
pub struct Ast {}

impl pattern_tree::MatchAssociations for Ast {
    type Expr = ast::Expr;
    type Lit = ast::Lit;
    type Bool = bool;
    type Char = char;
    type Int = u128;
    type Stmt = ast::Stmt;
}

impl<'cx, 'o, Cx> IsMatch<ast::LitKind> for Lit<'cx, 'o, Cx, Ast> {
    fn is_match(&self, other: &ast::LitKind) -> bool {
        match (self, other) {
            (Lit::Char(i), ast::LitKind::Char(j)) => i.is_match(j),
            (Lit::Bool(i), ast::LitKind::Bool(j)) => i.is_match(j),
            (Lit::Int(i), ast::LitKind::Int(j, _)) => i.is_match(j),
            _ => false,
        }
    }
}

impl<'cx, 'o, Cx> IsMatch<ast::ExprKind> for Expr<'cx, 'o, Cx, Ast> {
    fn is_match(&self, other: &ast::ExprKind) -> bool {
        match (self, other) {
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => 
                i.is_match(j),
            (Expr::Block(i), ast::ExprKind::Block(j, _label)) => 
                i.is_match(j),
            (Expr::Array(i), ast::ExprKind::Array(j)) => 
                i.is_match(j),
            (Expr::If(i_check, i_then, i_else), ast::ExprKind::If(j_check, j_then, j_else)) =>
                i_check.is_match(j_check) && 
                i_then.is_match(j_then) && 
                i_else.is_match(j_else),
            (Expr::IfLet(i_block, i_else), ast::ExprKind::IfLet(_pattern, _check, j_block, j_else)) => 
                // TODO: also check pattern and expr
                i_block.is_match(j_block) && 
                i_else.is_match(j_else),
            _ => false,
        }
    }
}

impl<'cx, 'o, Cx> IsMatch<ast::Expr> for Expr<'cx, 'o, Cx, Ast> {
    fn is_match(&self, other: &ast::Expr) -> bool {
        self.is_match(&other.node)
    }
}

impl<'cx, 'o, Cx> IsMatch<ast::StmtKind> for Stmt<'cx, 'o, Cx, Ast> {
    fn is_match(&self, other: &ast::StmtKind) -> bool {
        match (self, other) {
            (Stmt::Expr(i), ast::StmtKind::Expr(j)) => i.is_match(j),
            (Stmt::Semi(i), ast::StmtKind::Semi(j)) => i.is_match(j),
            _ => false,
        }
    }
}

impl<'cx, 'o, Cx> IsMatch<ast::Stmt> for Stmt<'cx, 'o, Cx, Ast> {
    fn is_match(&self, other: &ast::Stmt) -> bool {
        self.is_match(&other.node)
    }
}

impl<'cx, 'o, Cx, O> IsMatch<ast::Block> for Seq<'cx, 'o, Stmt<'cx, 'o, Cx, Ast>, Cx, O> {
    fn is_match(&self, other: &ast::Block) -> bool {
        self.is_match(&other.stmts)
    }
}

impl<T, U> IsMatch<syntax::ptr::P<U>> for T 
where T: PatternTreeNode, T: IsMatch<U> {
    fn is_match(&self, other: &syntax::ptr::P<U>) -> bool {
        self.is_match(&*other)
    }
}

impl<T, U> IsMatch<syntax::source_map::Spanned<U>> for T 
where T: PatternTreeNode, T: IsMatch<U> {
    fn is_match(&self, other: &syntax::source_map::Spanned<U>) -> bool {
        self.is_match(&other.node)
    }
}

impl<'cx, 'o, Cx, A> PatternTreeNode for Lit<'cx, 'o, Cx, A> where A: pattern_tree::MatchAssociations {}
impl<'cx, 'o, Cx, A> PatternTreeNode for Expr<'cx, 'o, Cx, A> where A: pattern_tree::MatchAssociations {}
impl<'cx, 'o, Cx, A> PatternTreeNode for Stmt<'cx, 'o, Cx, A> where A: pattern_tree::MatchAssociations {}
impl<'cx, 'o, Cx, A, O> PatternTreeNode for Seq<'cx, 'o, Stmt<'cx, 'o, Cx, A>, Cx, O> where A: pattern_tree::MatchAssociations {}