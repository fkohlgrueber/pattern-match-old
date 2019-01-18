
use crate::repeat::Repeat;


pub enum Expr {
    Lit(Alt<Lit>),
    Ray(Seq<Expr>),
    Array(Seq<Expr>)
}


pub enum Lit {
    Char(Alt<char>),
    Bool(Alt<bool>),
    Int(Alt<u128>),
}

// --------------------------------------------



pub struct Alt<T> 
where T: Descendant {
    pub values: Option<Vec<T>>
}


pub struct Seq<T> 
where T: Descendant {
    pub values: Option<Vec<Vec<Repeat<Alt<T>>>>>
}


pub trait PatternTreeNode {}
pub trait Descendant {}

trait MatchesEquality: PartialEq {}

// --------------------------------------------

impl PatternTreeNode for Lit {}
impl PatternTreeNode for Expr {}

impl<T> Descendant for T
where T: PatternTreeNode {}



// Types that implement PartialEq
impl Descendant for char {}
impl Descendant for bool {}
impl Descendant for u128 {}

// --------------------------------------------

pub trait Matches<U> {
    //fn matches(&self, other: &U) -> bool;
}

impl<T> Matches<T> for T 
where T: PartialEq {}
