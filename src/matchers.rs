use crate::repeat::Repeat;
use itertools::Itertools;

pub trait IsMatch<Rhs = Self> {
    fn is_match(&self, other: &Rhs) -> bool;
}


pub struct MatchValues<T> {
    pub values: Option<Vec<T>>,
}


impl<T> IsMatch<T> for T
where T: PartialEq {
    fn is_match(&self, other: &T) -> bool {
        self == other
    }
}

impl<T, U> IsMatch<U> for MatchValues<T>
where T: IsMatch<U> {
    fn is_match(&self, other: &U) -> bool {
        match &self.values {
            Some(v) => v.iter().any(|x| x.is_match(other)),
            None => true,
        }
    }
}

pub struct MatchSequences<T> {
    pub seq: Vec<Repeat<T>>
}

impl<T, U> IsMatch<&[&U]> for MatchSequences<T> 
where T: IsMatch<U> {
    fn is_match(&self, other: &&[&U]) -> bool {
        
        let iterators: Vec<_> = self.seq.iter().map(
            |x| x.range.start..x.range.end.unwrap_or_else(|| other.len())
        ).multi_cartesian_product()
         .filter(|x| x.iter().sum::<usize>() == other.len())
         .collect();

        'outer: for vals in iterators {
            let mut skip = 0;
            for (i, v) in vals.iter().enumerate() {
                if !other.iter().skip(skip).take(*v).all(|x| self.seq[i].elmt.is_match(x)) {
                    continue 'outer;
                }
                skip += v;
            }
            return true;
        }
        
        false
    }
}

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