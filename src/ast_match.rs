use crate::matchers::{IsMatch, MatchResult, Join, MatchSequences, MatchValues};
use syntax::ast;
use crate::pattern_tree::*;
use itertools::Itertools;

impl IsMatch<ast::TyKind> for Ty {
    fn is_match(&self, other: &ast::TyKind) -> Option<MatchResult> {
        match (self, other) {
            (Ty::Path(i), ast::TyKind::Path(_, j)) => {
                let vals = j.segments.iter().map(|x| x.ident.to_string()).collect::<Vec<_>>();
                i.is_match(&vals.iter().collect::<Vec<_>>().as_slice())
            },
            (Ty::Ptr(ity, imut), ast::TyKind::Ptr(jmutty)) => {
                ity.is_match(&jmutty.ty.node).join(imut.is_match(&jmutty.mutbl))
            }
            _ => None,
        }
    }
}


impl IsMatch<ast::ExprKind> for Expr {
    fn is_match(&self, other: &ast::ExprKind) -> Option<MatchResult> {
        match (self, other) {
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => i.is_match(&j.node),
            (Expr::Array(i), ast::ExprKind::Array(j)) => i.is_match(j),
            (Expr::Cast(ie, ity), ast::ExprKind::Cast(je, jty)) => ie.is_match_2(je).join(ity.is_match(&jty.node)),
            (Expr::If(i_check, i_then, i_else), ast::ExprKind::If(j_check, j_then, j_else)) => 
                i_check.is_match(&j_check.node)
                    .join(i_then.is_match(&j_then.stmts.iter().map(|x| x).collect::<Vec<_>>().as_slice()))
                    .join(i_else.is_match(j_else)),
            (Expr::Block(i), ast::ExprKind::Block(j, _label)) => i.is_match(&j.stmts.iter().map(|x| x).collect::<Vec<_>>().as_slice()),
            (Expr::IfLet(i_block, i_else), ast::ExprKind::IfLet(_pattern, _check, j_block, j_else)) => // TODO: also check pattern and expr
                i_block.is_match(&j_block.stmts.iter().map(|x| x).collect::<Vec<_>>().as_slice())
                    .join(i_else.is_match(j_else)),
            _ => None,
        }
    }
}


impl IsMatch<ast::Expr> for Expr {
    fn is_match(&self, other: &ast::Expr) -> Option<MatchResult> {
        self.is_match(&other.node)
    }
}


impl IsMatch<ast::LitKind> for Lit {
    fn is_match(&self, other: &ast::LitKind) -> Option<MatchResult> {
        match (self, other) {
            (Lit::Char(i), ast::LitKind::Char(j)) => i.is_match(j),
            (Lit::Bool(i), ast::LitKind::Bool(j)) => i.is_match(j),
            (Lit::Int(i), ast::LitKind::Int(j, _)) => i.is_match(j),
            _ => None,
        }
    }
}


impl IsMatch<ast::StmtKind> for Stmt {
    fn is_match(&self, other: &ast::StmtKind) -> Option<MatchResult> {
        match (self, other) {
            (Stmt::Expr(i), ast::StmtKind::Expr(j)) => i.is_match_2(j),
            (Stmt::Semi(i), ast::StmtKind::Semi(j)) => i.is_match_2(j),
            _ => None,
        }
    }
}

impl IsMatch<ast::Stmt> for Stmt {
    fn is_match(&self, other: &ast::Stmt) -> Option<MatchResult> {
        self.is_match(&other.node)
    }
}

pub trait IsMatch2<Rhs> {
    fn is_match_2(&self, other: &Rhs) -> Option<MatchResult>;
}

impl<T, U> IsMatch2<syntax::source_map::Spanned<U>> for T
where T: IsMatch<U> {
    fn is_match_2(&self, other: &syntax::source_map::Spanned<U>) -> Option<MatchResult> {
        self.is_match(&other.node)
    }
}

impl<T, U> IsMatch2<syntax::ptr::P<U>> for T
where T: IsMatch<U> {
    fn is_match_2(&self, other: &syntax::ptr::P<U>) -> Option<MatchResult> {
        self.is_match(&*other)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lit() {
        assert!(Lit::Bool(any!()).is_match(&ast::LitKind::Bool(false)));
        assert!(Lit::Bool(any!(true)).is_match(&ast::LitKind::Bool(true)));
        
        assert!(!Lit::Bool(any!(true)).is_match(&ast::LitKind::Bool(false)));
    }

    #[test]
    fn test_expr() {
        //assert!(Expr::Array(seq!()).is_match(&ast::ExprKind::Array(vec!())));
    }
}
*/