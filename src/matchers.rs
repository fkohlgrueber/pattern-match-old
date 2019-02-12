
use itertools::Itertools;
use itertools::repeat_n;

use pattern_tree::matchers::*;

// Trait that has to be implemented on all types that can be used in a pattern tree
pub trait PatternTreeNode {}

impl PatternTreeNode for char {}
impl PatternTreeNode for u128 {}
impl PatternTreeNode for bool {}

impl IsMatchEquality for u128 {}
impl IsMatchEquality for char {}
impl IsMatchEquality for bool {}

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


impl<T, U> IsMatch<U> for Alt<T>
where T: PatternTreeNode + IsMatch<U> {
    fn is_match(&self, other: &U) -> bool {
        match self {
            Alt::Any => true,
            Alt::Elmt(e) => e.is_match(other),
            Alt::Named(e, _) => e.is_match(other),
            Alt::Alt(a, b) => a.is_match(other) || b.is_match(other)
        }
    }
}

impl<T, U> IsMatch<&[U]> for Seq<T> 
where T: PatternTreeNode + IsMatch<U> {
    fn is_match(&self, other: &&[U]) -> bool {
        
        match self {
            Seq::Any => other.len() == 1,
            Seq::Elmt(e) => other.len() == 1 && e.is_match(&other[0]),
            Seq::Named(e, _) => e.is_match(other),
            Seq::Alt(a, b) => a.is_match(other) || b.is_match(other),
            Seq::Empty => other.is_empty(),
            Seq::Repeat(e, r) => {
                let e_range = e.num_elmts_range();
                let e_range = e_range.start..e_range.end.unwrap_or(other.len()+1);

                if r.start == 0 && other.is_empty() {
                    return true;
                }

                for i in r.start..r.end.unwrap_or(other.len()+1) {
                    
                    let iterators = repeat_n(e_range.clone(), i)
                        .multi_cartesian_product()
                        .filter(|x| x.iter().sum::<usize>() == other.len());

                    'outer: for vals in iterators {
                        let mut skip = 0;
                        for v in vals.iter() {
                            
                            if !e.is_match(&&other[skip..skip+v]) {
                                continue 'outer;
                            }
                            skip += v;
                        }
                        return true;
                    }
                }

                false
            },
            Seq::Seq(a, b) => {
                let range = a.num_elmts_range();
                for i in range.start..range.end.unwrap_or(other.len()+1) {
                    if i > other.len() {
                        break;
                    }
                    let (l, r) = other.split_at(i);
                    if a.is_match(&l) && b.is_match(&r) {
                        return true;
                    }
                }
                false
            },
            
        }
    }
}

impl<T, U> IsMatch<Vec<U>> for Seq<T> 
where T: PatternTreeNode + IsMatch<U> {
    fn is_match(&self, other: &Vec<U>) -> bool {
        self.is_match(&other.as_slice())
    }
}


impl<T, U> IsMatch<Option<U>> for Opt<T> 
where T: PatternTreeNode + IsMatch<U> {
    fn is_match(&self, other: &Option<U>) -> bool {
        
        match self {
            Opt::Any => other.is_some(),
            Opt::Elmt(e) => match other {
                Some(other) => e.is_match(other),
                None => false
            },
            Opt::Named(e, _) => e.is_match(other),
            Opt::Alt(a, b) => a.is_match(other) || b.is_match(other),
            Opt::None => other.is_none(),
        }
    }
}