#![allow(dead_code)]
use crate::repeat::Repeat;
use std::ops::Deref;


// --------------------------------------------
//         Alternatives / Sequences
// --------------------------------------------

struct Alternative<T>(Vec<T>);  // Empty Vec matches everything
struct Sequence<T>(Vec<Repeat<T>>);

pub struct Alt<T>(Alternative<T>);
pub struct Seq<T>(Alternative<Sequence<Alternative<T>>>);

// --------------------------------------------
//                Pattern Tree
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
//                  Reductions
// --------------------------------------------

pub trait Reduce {
    type Target;

    fn reduce(&self) -> &Self::Target;
}

impl<T> Reduce for syntax::ptr::P<T> {
    type Target = T;

    fn reduce(&self) -> &Self::Target {
        &*self
    }
}

impl<T> Reduce for syntax::source_map::Spanned<T> {
    type Target = T;

    fn reduce(&self) -> &Self::Target {
        &self.node
    }
}

impl Reduce for syntax::ast::Stmt {
    type Target = syntax::ast::StmtKind;

    fn reduce(&self) -> &Self::Target {
        &self.node
    }
}

impl Reduce for syntax::ast::Expr {
    type Target = syntax::ast::ExprKind;

    fn reduce(&self) -> &Self::Target {
        &self.node
    }
}

impl Reduce for syntax::ast::Block {
    type Target = Vec<syntax::ast::Stmt>;

    fn reduce(&self) -> &Self::Target {
        &self.stmts
    }
}

// --------------------------------------------
//            Is Match Equality
// --------------------------------------------

trait IsMatchEquality {
    fn is_match_equality(&self, other: &Self) -> bool;
}

impl<T> IsMatchEquality for T 
where T: PartialEq {
    fn is_match_equality(&self, other: &Self) -> bool {
        self == other
    }
}

// --------------------------------------------
//            Is Match Node
// --------------------------------------------
//  left: PatternTreeNode
//  right: ast node (probably reduced)

trait IsMatchNode<T> {
    fn is_match_node(&self, other: &T) -> bool;
}

use syntax::ast;

impl IsMatchNode<ast::LitKind> for Lit {
    fn is_match_node(&self, other: &ast::LitKind) -> bool {
        match (self, other) {
            (Lit::Char(i), ast::LitKind::Char(j)) => i.is_match_alt_seq(j),
            (Lit::Bool(i), ast::LitKind::Bool(j)) => i.is_match_alt_seq(j),
            (Lit::Int(i), ast::LitKind::Int(j, _)) => i.is_match_alt_seq(j),
            _ => false,
        }
    }
}

impl IsMatchNode<ast::ExprKind> for Expr {
    fn is_match_node(&self, other: &ast::ExprKind) -> bool {
        match (self, other) {
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => i.is_match_alt_seq(j),
            //(Expr::Array(i), ast::ExprKind::Array(j)) => i.is_match(j.reduce()),
            // (Expr::Block(i), ast::ExprKind::Block(j, _label)) => i.is_match(j),
            _ => false,
        }
    }
}

// --------------------------------------------
//            Is Match AltSeq
// --------------------------------------------
//  left: Alt or Seq
//  right: ast node (probably reduced)

trait IsMatchAltSeq<T> {
    fn is_match_alt_seq(&self, other: &T) -> bool;
}

impl<T> IsMatchAltSeq<T> for Alternative<T> 
where T: IsMatchEquality {
    fn is_match_alt_seq(&self, other: &T) -> bool {
        self.0.iter().any(|x| x.is_match_equality(other))
    }
}

impl<T, U> IsMatchAltSeq<U> for Alternative<T> 
where T: IsMatchNode<U> {
    fn is_match_alt_seq(&self, other: &U) -> bool {
        self.0.iter().any(|x| x.is_match_equality(other))
    }
}

impl<T> IsMatchAltSeq<T> for Alt<T>
where T: IsMatchEquality {
    fn is_match_alt_seq(&self, other: &T) -> bool {
        self.0.is_match_alt_seq(other)
    }
}