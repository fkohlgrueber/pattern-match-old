use crate::matchers::{MatchSequences, MatchValues};

type Alt<T> = MatchValues<T>;
type Seq<T> = MatchValues<MatchSequences<MatchValues<T>>>;

#[derive(Clone)]
pub enum Ty {
    Ptr(Alt<Ty>, Alt<syntax::ast::Mutability>),
    Path(Seq<String>),
}

#[derive(Clone)]
pub enum Expr {
    Lit(Alt<Lit>),
    Array(Seq<Expr>),
    Cast(Alt<Expr>, Alt<Ty>),
    If(Alt<Expr>, Block, Alt<Option<Alt<Expr>>>),
    Block(Block),
    IfLet(Block, Alt<Option<Alt<Expr>>>)
}

#[derive(Clone)]
pub enum Lit {
    Char(Alt<char>),
    Bool(Alt<bool>),
    Int(Alt<u128>),
}

type Block = Seq<Stmt>;

#[derive(Clone)]
pub enum Stmt {
    Expr(Alt<Expr>),
    Semi(Alt<Expr>)
}