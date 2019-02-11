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
            (Expr::Array(i), ast::ExprKind::Array(j)) => 
                i.is_match(j),
            _ => false,
        }
    }
}

impl IsMatch<ast::Expr> for Expr {
    fn is_match(&self, other: &ast::Expr) -> bool {
        self.is_match(&other.node)
    }
}


impl PatternTreeNode for Lit {}
impl PatternTreeNode for Expr {}