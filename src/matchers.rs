use crate::repeat::Repeat;
use crate::pattern_tree::PatternTreeNode;
use itertools::Itertools;


// Main trait for matching
pub trait IsMatch<T> {
    fn is_match(&self, other: &T) -> bool;
}

// Trait for types that can be matched by their equality
pub trait IsMatchEquality: PartialEq {}

impl<T> IsMatch<T> for T 
where T: IsMatchEquality {
    fn is_match(&self, other: &T) -> bool {
        self == other
    }
}

// Basic pattern building blocks 
#[derive(Clone)]
pub struct Alternative<T>(pub Vec<T>);  // Empty Vec matches everything
#[derive(Clone)]
pub struct Sequence<T>(pub Vec<Repeat<T>>);
#[derive(Clone)]
pub struct Optional<T>(pub Option<T>);

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
        match (&self.0, &other) {
            (Some(i), Some(j)) => i.is_match(j),
            (None, None) => true,
            _ => false
        }
    }
}

// structs that may actually be used in the pattern tree
#[derive(Clone)]
pub struct Alt<T>(Alternative<T>) where T: PatternTreeNode;
#[derive(Clone)]
pub struct Seq<T>(Alternative<Sequence<Alternative<T>>>) where T: PatternTreeNode;
#[derive(Clone)]
pub struct Opt<T>(Alternative<Optional<Alternative<T>>>) where T: PatternTreeNode;

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

impl<T> From<Alternative<T>> for Alt<T> 
where T: PatternTreeNode {
    fn from(other: Alternative<T>) -> Self {
        Alt(other)
    }
}

impl<T> From<Alternative<Optional<Alternative<T>>>> for Opt<T> 
where T: PatternTreeNode {
    fn from(other: Alternative<Optional<Alternative<T>>>) -> Self {
        Opt(other)
    }
}

impl<T> From<Alternative<Sequence<Alternative<T>>>> for Seq<T> 
where T: PatternTreeNode {
    fn from(other: Alternative<Sequence<Alternative<T>>>) -> Self {
        Seq(other)
    }
}
