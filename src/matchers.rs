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
pub struct Alternative<T>(Vec<T>);  // Empty Vec matches everything
pub struct Sequence<T>(Vec<Repeat<T>>);
pub struct Optional<T>(Option<T>);

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

// structs that may actually be used in the pattern tree
pub struct Alt<T>(Alternative<T>) where T: PatternTreeNode;
pub struct Seq<T>(Alternative<Sequence<Alternative<T>>>) where T: PatternTreeNode;
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

