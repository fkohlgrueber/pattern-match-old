use crate::matchers::{MatchSequences, MatchValues};

pub enum Ty {
    Ptr(MatchValues<Ty>, MatchValues<syntax::ast::Mutability>),
    Path(MatchSequences<MatchValues<String>>),
}

pub enum Expr {
    Lit(MatchValues<Lit>),
    Array(MatchSequences<MatchValues<Expr>>),
    Cast(MatchValues<Expr>, MatchValues<Ty>),
}

pub enum Lit {
    Char(MatchValues<char>),
    Bool(MatchValues<bool>),
    Int(MatchValues<u128>),
}