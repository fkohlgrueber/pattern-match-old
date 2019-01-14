use crate::matchers::IsMatch;
use syntax::ast;
use crate::pattern_tree::*;


impl IsMatch<ast::TyKind> for Ty {
    fn is_match(&self, other: &ast::TyKind) -> bool {
        match (self, other) {
            (Ty::Path(i), ast::TyKind::Path(_, j)) => {
                let vals = j.segments.iter().map(|x| x.ident.to_string()).collect::<Vec<_>>();
                i.is_match(&vals.iter().collect::<Vec<_>>().as_slice())
            },
            (Ty::Ptr(ity, imut), ast::TyKind::Ptr(jmutty)) => {
                ity.is_match(&jmutty.ty.node) && imut.is_match(&jmutty.mutbl)
            }
            _ => false,
        }
    }
}


impl IsMatch<ast::ExprKind> for Expr {
    fn is_match(&self, other: &ast::ExprKind) -> bool {
        match (self, other) {
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => i.is_match(&j.node),
            (Expr::Array(i), ast::ExprKind::Array(j)) => i.is_match(&j.iter().map(|x| &**x).collect::<Vec<_>>().as_slice()),
            (Expr::Cast(ie, ity), ast::ExprKind::Cast(je, jty)) => ie.is_match(&je.node) && ity.is_match(&jty.node),
            _ => false,
        }
    }
}


impl IsMatch<ast::Expr> for Expr {
    fn is_match(&self, other: &ast::Expr) -> bool {
        self.is_match(&other.node)
    }
}


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