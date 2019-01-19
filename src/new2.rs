#![allow(dead_code)]
use crate::repeat::Repeat;
use itertools::Itertools;

struct Alternative<T>(Vec<T>);  // Empty Vec matches everything
struct Sequence<T>(Vec<Repeat<T>>);
struct Optional<T>(Option<T>);

// --------------------------------------------

pub struct Alt<T>(Alternative<T>)
where T: PatternTreeNode;


pub struct Seq<T>(Alternative<Sequence<Alternative<T>>>)
where T: PatternTreeNode;

pub struct Opt<T>(Alternative<Optional<Alternative<T>>>)
where T: PatternTreeNode;


// --------------------------------------------

pub enum Expr {
    Lit(Alt<Lit>),
    Array(Seq<Expr>),
    Block(Block),
    If(Alt<Expr>, Block, Opt<Expr>),
    IfLet(Block, Opt<Expr>)
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
        self.0.is_empty() || self.0.iter().any(|x| x.is_match(other))
    }
}

impl<T, U> IsMatch<Vec<U>> for Sequence<T> 
where T: IsMatch<U> {
    fn is_match(&self, other: &Vec<U>) -> bool {
        let iterators: Vec<_> = self.0.iter().map(
            |x| x.range.start..x.range.end.unwrap_or_else(|| other.len()+1)
        ).multi_cartesian_product()
         .filter(|x| x.iter().sum::<usize>() == other.len())
         .collect();

        'outer: for vals in iterators {
            let mut skip = 0;
            for (i, v) in vals.iter().enumerate() {
                if !other.iter().skip(skip).take(*v).all(|x| self.0[i].elmt.is_match(x)) {
                    continue 'outer;
                }
                skip += v;
            }
            return true;
        }
        
        false
    }
}

impl<T, U> IsMatch<Option<U>> for Optional<T> 
where T: IsMatch<U> {
    fn is_match(&self, other: &Option<U>) -> bool {
        if let Some(i) = &self.0 {
            if let Some(j) = &other {
                return i.is_match(j);
            }
        }
        false
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

impl<T, U> IsMatch<Option<U>> for Opt<T>
where T: PatternTreeNode + IsMatch<U> {
    fn is_match(&self, other: &Option<U>) -> bool {
        self.0.is_match(other)
    }
}

// --------------------------------------------

pub trait IsMatch<T> {
    fn is_match(&self, other: &T) -> bool;
}

pub trait IsMatchEquality: PartialEq {}

impl IsMatchEquality for u128 {}
impl IsMatchEquality for char {}
impl IsMatchEquality for bool {}


impl<T> IsMatch<T> for T 
where T: IsMatchEquality {
    fn is_match(&self, other: &T) -> bool {
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
            (Expr::Lit(i), ast::ExprKind::Lit(j)) => 
                i.is_match(j),
            (Expr::Array(i), ast::ExprKind::Array(j)) => 
                i.is_match(j),
            (Expr::Block(i), ast::ExprKind::Block(j, _label)) => 
                i.is_match(j),
            (Expr::If(i_check, i_then, i_else), ast::ExprKind::If(j_check, j_then, j_else)) =>
                i_check.is_match(j_check) && 
                i_then.is_match(j_then) && 
                i_else.is_match(j_else),
            (Expr::IfLet(i_block, i_else), ast::ExprKind::IfLet(_pattern, _check, j_block, j_else)) => // TODO: also check pattern and expr
                i_block.is_match(j_block) && 
                i_else.is_match(j_else),
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

impl IsMatch<ast::Block> for Block {
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
