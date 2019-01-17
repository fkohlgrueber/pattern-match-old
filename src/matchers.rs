use crate::repeat::Repeat;
use itertools::Itertools;
use std::ops::Deref;
use std::collections::HashMap;

#[derive(Default)]
pub struct MatchResult {
    pub names: HashMap<String, String>
}

impl MatchResult {
    fn update(&mut self, other: MatchResult) {
        self.names.extend(other.names);
    }
}

pub trait Join {
    fn join(self, other: Option<MatchResult>) -> Option<MatchResult>;
}

impl Join for Option<MatchResult> {
    fn join(self, other: Option<MatchResult>) -> Option<MatchResult> {
        if let Some(mut i) = self {
            if let Some(j) = other {
                i.update(j);
                return Some(i);
            }
        }
        None
    }
}

pub trait IsMatch<Rhs = Self> {
    fn is_match(&self, other: &Rhs) -> Option<MatchResult>;
}

pub trait IsMatchEquality: PartialEq {}

impl IsMatchEquality for char {}
impl IsMatchEquality for String {}
impl IsMatchEquality for bool {}
impl IsMatchEquality for u128 {}
impl IsMatchEquality for syntax::ast::Mutability {}

#[derive(Clone)]
pub struct MatchValues<T> {
    pub values: Option<Vec<T>>,
    pub name: Option<String>
}


impl<T> IsMatch<T> for T
where T: IsMatchEquality {
    fn is_match(&self, other: &T) -> Option<MatchResult> {
        if self == other {
            Some(MatchResult::default())
        } else {
            None
        }
    }
}

impl<T, U, V> IsMatch<Option<V>> for Option<T> 
where T: IsMatch<U>, V: Deref<Target=U> {
    fn is_match(&self, other: &Option<V>) -> Option<MatchResult> {
        match (self, other) {
            (Some(i), Some(j)) => i.is_match(j),
            (None, None) => Some(MatchResult::default()),
            _ => None
        }
    }
}

impl<T, U> IsMatch<U> for MatchValues<T>
where T: IsMatch<U> {
    fn is_match(&self, other: &U) -> Option<MatchResult> {
        let mut res = match &self.values {
            Some(v) => v.iter().filter_map(|x| x.is_match(other)).next(),
            None => Some(MatchResult::default()),
        };
        if let Some(ref mut res) = &mut res {
            if let Some(name) = &self.name {
                res.names.insert(name.clone(), format!("info for {}", name.clone()).to_string());
            }
        }
        res
    }
}

#[derive(Clone)]
pub struct MatchSequences<T> {
    pub seq: Vec<Repeat<T>>
}

impl<T, U> IsMatch<&[&U]> for MatchSequences<T> 
where T: IsMatch<U> {
    fn is_match(&self, other: &&[&U]) -> Option<MatchResult> {
        
        let iterators: Vec<_> = self.seq.iter().map(
            |x| x.range.start..x.range.end.unwrap_or_else(|| other.len()+1)
        ).multi_cartesian_product()
         .filter(|x| x.iter().sum::<usize>() == other.len())
         .collect();

        'outer: for vals in iterators {
            let mut skip = 0;
            let mut res = Some(MatchResult::default());
            for (i, v) in vals.iter().enumerate() {
                res = other.iter().skip(skip).take(*v).fold(res, |r, x| {
                    r.join(self.seq[i].elmt.is_match(x))
                });
                if res.is_none() {
                    continue 'outer;
                }
                skip += v;
            }
            return res;
        }
        
        None
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::repeat::RepeatRange;

    #[test]
    fn test_primitive() {
        assert!(true.is_match(&true));
        assert!(!true.is_match(&false));
        assert!('a'.is_match(&'a'));
        assert!(!'a'.is_match(&'b'));
    }

    #[test]
    fn test_match_values_empty() {
        // None matches anything
        let m = MatchValues::<char> { values: None };
        assert!(m.is_match(&'a'));
        assert!(m.is_match(&'b'));
        assert!(m.is_match(&'c'));
    }

    #[test]
    fn test_match_values() {
        // Some(..) matches specified values
        let m = MatchValues::<char> { values: Some(vec!('a', 'b')) };
        assert!(m.is_match(&'a'));
        assert!(m.is_match(&'b'));
        assert!(!m.is_match(&'c'));
    }

    #[test]
    fn test_match_sequences() {
        let m = MatchSequences::<char> { seq: vec!(
            Repeat { elmt: 'a', range: RepeatRange { start: 0, end: None } },
            Repeat { elmt: 'b', range: RepeatRange { start: 1, end: Some(2) } },
            Repeat { elmt: 'c', range: RepeatRange { start: 1, end: Some(3) } },
        ) };
        assert!(m.is_match(&&[&'b', &'c'][..]));
        assert!(m.is_match(&&[&'a', &'b', &'c'][..]));
        assert!(m.is_match(&&[&'a', &'a', &'b', &'c'][..]));
        assert!(m.is_match(&&[&'a', &'a', &'b', &'c', &'c'][..]));
        assert!(m.is_match(&&[&'b', &'c', &'c'][..]));
        
        assert!(!m.is_match(&&[&'x', &'b', &'c'][..]));
        assert!(!m.is_match(&&[&'b', &'c', &'x'][..]));
        assert!(!m.is_match(&&[&'b', &'c', &'c', &'c'][..]));
    }
}

*/