use crate::matchers::{MatchSequences, MatchValues};

pub enum Ty {
    Ptr(MatchValues<Ty>, MatchValues<syntax::ast::Mutability>),
    Path(MatchSequences<MatchValues<String>>),
}

pub enum Expr {
    Lit(MatchValues<Lit>),
    Array(MatchSequences<MatchValues<Expr>>),
    Cast(MatchValues<Expr>, MatchValues<Ty>),
    If(MatchValues<Expr>, Block, MatchValues<Option<Expr>>)
}

pub enum Lit {
    Char(MatchValues<char>),
    Bool(MatchValues<bool>),
    Int(MatchValues<u128>),
}


type Block = MatchSequences<MatchValues<Stmt>>;

pub enum Stmt {
    Expr(MatchValues<Expr>),
    Semi(MatchValues<Expr>)
}