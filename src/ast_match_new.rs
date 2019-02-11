use pattern_tree::*;
use crate::matchers::IsMatch;
use crate::pattern_tree_old::PatternTreeNode;
use syntax::ast;

impl IsMatch<ast::LitKind> for Lit {
    fn is_match(&self, other: &ast::LitKind) -> bool {
        match (self, other) {
            (Lit::Char(i), ast::LitKind::Char(j)) => i.is_match(j),
            (Lit::Bool(i), ast::LitKind::Bool(j)) => i.is_match(j),
            (Lit::Int(i), ast::LitKind::Int(j, _)) => i.is_match(j),
            _ => false,
        }
    }
}

impl IsMatch<ast::ExprKind> for Expr {
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

impl IsMatch<ast::Expr> for Expr {
    fn is_match(&self, other: &ast::Expr) -> bool {
        self.is_match(&other.node)
    }
}

impl IsMatch<ast::StmtKind> for Stmt {
    fn is_match(&self, other: &ast::StmtKind) -> bool {
        match (self, other) {
            (Stmt::Expr(i), ast::StmtKind::Expr(j)) => i.is_match(j),
            (Stmt::Semi(i), ast::StmtKind::Semi(j)) => i.is_match(j),
            _ => false,
        }
    }
}

impl IsMatch<ast::Stmt> for Stmt {
    fn is_match(&self, other: &ast::Stmt) -> bool {
        self.is_match(&other.node)
    }
}

impl IsMatch<ast::Block> for Block {
    fn is_match(&self, other: &ast::Block) -> bool {
        self.is_match(&other.stmts)
    }
}

use pattern_tree::matchers::Seq;
type Block = Seq<Stmt>;

impl PatternTreeNode for Lit {}
impl PatternTreeNode for Expr {}
impl PatternTreeNode for Stmt {}
impl PatternTreeNode for Block {}