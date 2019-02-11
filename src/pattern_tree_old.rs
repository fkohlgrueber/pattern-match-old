use crate::matchers::*;

// Trait that has to be implemented on all types that can be used in a pattern tree
pub trait PatternTreeNode {}

impl PatternTreeNode for Expr {}
impl PatternTreeNode for Lit {}
impl PatternTreeNode for Block {}
impl PatternTreeNode for Stmt {}

impl PatternTreeNode for char {}
impl PatternTreeNode for u128 {}
impl PatternTreeNode for bool {}

#[derive(Clone)]
pub enum Expr {
    Lit(Alt<Lit>),
    Array(Seq<Expr>),
    Block(Block),
    If(Alt<Expr>, Block, Opt<Expr>),
    IfLet(Block, Opt<Expr>)
}

#[derive(Clone)]
pub enum Lit {
    Char(Alt<char>),
    Bool(Alt<bool>),
    Int(Alt<u128>),
}

pub type Block = Seq<Stmt>;

#[derive(Clone)]
pub enum Stmt {
    Expr(Alt<Expr>),
    Semi(Alt<Expr>)
}

impl IsMatchEquality for u128 {}
impl IsMatchEquality for char {}
impl IsMatchEquality for bool {}

