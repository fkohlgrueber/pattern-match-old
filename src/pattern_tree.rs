use crate::matchers::{MatchSequences, MatchValues};

#[derive(Clone)]
pub enum Ty {
    Ptr(MatchValues<Ty>, MatchValues<syntax::ast::Mutability>),
    Path(MatchSequences<MatchValues<String>>),
}

#[derive(Clone)]
pub enum Expr {
    Lit(MatchValues<Lit>),
    Array(MatchSequences<MatchValues<Expr>>),
    Cast(MatchValues<Expr>, MatchValues<Ty>),
    If(MatchValues<Expr>, Block, MatchValues<Option<MatchValues<Expr>>>),
    Block(Block),
    IfLet(Block, MatchValues<Option<MatchValues<Expr>>>)
}

#[derive(Clone)]
pub enum Lit {
    Char(MatchValues<char>),
    Bool(MatchValues<bool>),
    Int(MatchValues<u128>),
}

type Block = MatchSequences<MatchValues<Stmt>>;

#[derive(Clone)]
pub enum Stmt {
    Expr(MatchValues<Expr>),
    Semi(MatchValues<Expr>)
}