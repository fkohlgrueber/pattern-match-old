#![allow(dead_code)]
use crate::repeat::Repeat;
use std::ops::Deref;
use syntax::ast;

struct Alternative<T>(Vec<T>);  // Empty Vec matches everything
struct Sequence<T>(Vec<Repeat<T>>);

// --------------------------------------------

pub struct Alt<T>(Alternative<T>)
where T: PatternTreeNode;


pub struct Seq<T>(Alternative<Sequence<Alternative<T>>>)
where T: PatternTreeNode;

// --------------------------------------------

pub enum Expr {
    Lit(Alt<Lit>),
    Array(Seq<Expr>),
    Block(Block),
}


pub enum Lit {
    Int(Alt<u128>),
}

type Block = Seq<Stmt>;

pub enum Stmt {
    Expr(Alt<Expr>),
    Semi(Alt<Expr>)
}

// --------------------------------------------

pub trait PatternTreeNode {}

impl PatternTreeNode for Expr {}
impl PatternTreeNode for Lit {}
impl PatternTreeNode for Stmt {}
impl PatternTreeNode for u128 {}

// --------------------------------------------

pub trait IsMatch<T> {
    fn is_match(&self, other: &T) -> bool;
}

// --------------------------------------------

impl IsMatch<Self> for u128 {
    fn is_match(&self, other: &Self) -> bool {
        self == other
    }
}

impl IsMatch<ast::Expr> for Expr {
    fn is_match(&self, other: &ast::Expr) -> bool {
        match (self, &other.node) {
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => i.is_match(j),
            (Expr::Array(i), ast::ExprKind::Array(j)) => i.is_match(j),
            //(Expr::Block(i), ast::ExprKind::Block(j, _label)) => i.is_match(j),
            _ => false,
        }
    }
}

impl IsMatch<ast::ExprKind> for Expr {
    fn is_match(&self, other: &ast::ExprKind) -> bool {
        match (self, other) {
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => i.is_match(j),
            (Expr::Array(i), ast::ExprKind::Array(j)) => i.is_match(j),
            //(Expr::Block(i), ast::ExprKind::Block(j, _label)) => i.is_match(j),
            _ => false,
        }
    }
}

impl IsMatch<syntax::ptr::P<ast::Expr>> for Expr {
    fn is_match(&self, other: &syntax::ptr::P<ast::Expr>) -> bool {
        match (self, &other.node) {
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => i.is_match(j),
            (Expr::Array(i), ast::ExprKind::Array(j)) => i.is_match(j),
            //(Expr::Block(i), ast::ExprKind::Block(j, _label)) => i.is_match(j),
            _ => false,
        }
    }
}

impl IsMatch<syntax::source_map::Spanned<ast::LitKind>> for Lit {
    fn is_match(&self, other: &syntax::source_map::Spanned<ast::LitKind>) -> bool {
        match (self, &other.node) {
            (Lit::Int(i), ast::LitKind::Int(j, _)) => i.is_match(j),
            _ => false,
        }
    }
}

impl IsMatch<ast::Stmt> for Stmt {
    fn is_match(&self, other: &ast::Stmt) -> bool {
        match (self, &other.node) {
            (Stmt::Expr(i), ast::StmtKind::Expr(j)) => i.is_match(j),
            (Stmt::Semi(i), ast::StmtKind::Semi(j)) => i.is_match(j),
            _ => false,
        }
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



// --------------------------------------------

impl IsMatch<u128> for Alt<u128> {
    fn is_match(&self, other: &u128) -> bool {
        (self.0).0.iter().any(|x| x.is_match(other))
    }
}

impl IsMatch<ast::Expr> for Alt<Expr> {
    fn is_match(&self, other: &ast::Expr) -> bool {
        (self.0).0.iter().any(|x| x.is_match(other))
    }
}

impl IsMatch<ast::ExprKind> for Alt<Expr> {
    fn is_match(&self, other: &ast::ExprKind) -> bool {
        (self.0).0.iter().any(|x| x.is_match(other))
    }
}

impl IsMatch<syntax::ptr::P<ast::Expr>> for Alt<Expr> {
    fn is_match(&self, other: &syntax::ptr::P<ast::Expr>) -> bool {
        (self.0).0.iter().any(|x| x.is_match(other))
    }
}

impl IsMatch<syntax::source_map::Spanned<ast::LitKind>> for Alt<Lit> {
    fn is_match(&self, other: &syntax::source_map::Spanned<ast::LitKind>) -> bool {
        (self.0).0.iter().any(|x| x.is_match(other))
    }
}

impl IsMatch<ast::Stmt> for Alt<Stmt> {
    fn is_match(&self, other: &ast::Stmt) -> bool {
        (self.0).0.iter().any(|x| x.is_match(other))
    }
}

impl IsMatch<ast::StmtKind> for Alt<Stmt> {
    fn is_match(&self, other: &ast::StmtKind) -> bool {
        (self.0).0.iter().any(|x| x.is_match(other))
    }
}

// --------------------------------------------

impl IsMatch<Vec<syntax::ptr::P<ast::Expr>>> for Seq<Expr> {
    fn is_match(&self, other: &Vec<syntax::ptr::P<ast::Expr>>) -> bool {
        unimplemented!()
        //(self.0).0.len() == other.len() && 
        //(self.0).0.iter().zip(other.iter()).all(|(x, y)| x.elmt.is_match(y))
    }
}

impl IsMatch<ast::Block> for Seq<Stmt> {
    fn is_match(&self, other: &ast::Block) -> bool {
        unimplemented!()
    }
}

impl IsMatch<Vec<ast::Stmt>> for Seq<Stmt> {
    fn is_match(&self, other: &Vec<ast::Stmt>) -> bool {
        unimplemented!()
    }
}

// --------------------------------------------
