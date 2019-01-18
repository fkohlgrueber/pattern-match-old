#![allow(dead_code)]
use crate::repeat::Repeat;
use std::ops::Deref;

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
    Ray(Seq<Expr>),
    Array(Seq<Expr>),
    Block(Block),
}


pub enum Lit {
    Char(Alt<char>),
    Bool(Alt<bool>),
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
impl PatternTreeNode for Block {}

impl PatternTreeNode for char {}
impl PatternTreeNode for u128 {}
impl PatternTreeNode for bool {}

// --------------------------------------------

impl<T, U> IsMatch<U> for Alternative<T> 
where T: IsMatch<U> {
    fn is_match(&self, other: &U) -> bool {
        self.0.iter().any(|x| x.is_match(other))
    }
}

impl<T, U> IsMatch<Vec<U>> for Sequence<T> 
where T: IsMatch<U> {
    fn is_match(&self, other: &Vec<U>) -> bool {
        self.0.len() == other.len() && 
        self.0.iter().zip(other.iter()).all(|(x, y)| x.elmt.is_match(y))
    }
}

impl<T, U> IsMatch<U> for Alt<T>
where T: PatternTreeNode + IsMatch<U> {
    fn is_match(&self, other: &U) -> bool {
        self.0.is_match(other)
    }
}


impl<T, U> IsMatch<Vec<U>> for Seq<T>
where T: PatternTreeNode + IsMatch<U> {
    fn is_match(&self, other: &Vec<U>) -> bool {
        self.0.is_match(other)
    }
}

// --------------------------------------------

pub trait IsMatch<T> {
    fn is_match(&self, other: &T) -> bool;
}

impl IsMatch<u128> for u128 {
    fn is_match(&self, other: &u128) -> bool {
        self == other
    }
}

impl IsMatch<char> for char {
    fn is_match(&self, other: &char) -> bool {
        self == other
    }
}

impl IsMatch<bool> for bool {
    fn is_match(&self, other: &bool) -> bool {
        self == other
    }
}

// --------------------------------------------

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
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => i.is_match(j),
            (Expr::Array(i), ast::ExprKind::Array(j)) => i.is_match(j),
            (Expr::Block(i), ast::ExprKind::Block(j, _label)) => i.is_match(j),
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

impl IsMatch<ast::Block> for Seq<Stmt> {
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

/*
impl<T> IsMatch<ast::Block> for T 
where T: IsMatch<Vec<ast::Stmt>> + PatternTreeNode {
    fn is_match(&self, other: &ast::Block) -> bool {
        self.is_match(&other.stmts)
    }
}
*/

// --------------------------------------------


// --------------------------------------------

// --------------------------------------------



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        'c'.is_match(&'c');
        Alt(Alternative(vec!(1, 2, 3))).is_match(&2);
        Seq::<u128>(Alternative(vec!())).is_match(&vec!(1, 23));


    }
}