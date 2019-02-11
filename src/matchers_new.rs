use crate::matchers::IsMatch;
use crate::pattern_tree_old::PatternTreeNode;

use pattern_tree::matchers::*;

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
        /*
        match self {
            Seq::Any => true,
            Seq::Elmt(e) => e.is_match(other),
            Seq::Named(e, _) => e.is_match(other),
            Seq::Alt(a, b) => a.is_match(other) || b.is_match(other),
            Seq::Empty => other.is_empty(),
        }
        */

        false

        /*
        let iterators = self.0.iter().map(
            |x| x.range.start..x.range.end.unwrap_or_else(|| other.len()+1)
        ).multi_cartesian_product()
         .filter(|x| x.iter().sum::<usize>() == other.len());

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
        */
    }
}

impl<T, U> IsMatch<Vec<U>> for Seq<T> 
where T: PatternTreeNode + IsMatch<U> {
    fn is_match(&self, other: &Vec<U>) -> bool {
        self.is_match(&other.as_slice())
    }
}