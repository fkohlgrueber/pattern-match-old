
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
pub trait IsMatch<'cx, 'o, Cx, O> {
    fn is_match(&self, cx: &'cx mut Cx, other: &'o O) -> (bool, &'cx mut Cx);
}

// Trait for types that can be matched by their equality
pub trait IsMatchEquality: PartialEq {}

impl<'cx, 'o, Cx, T> IsMatch<'cx, 'o, Cx, T> for T 
where T: IsMatchEquality {
    fn is_match(&self, cx: &'cx mut Cx, other: &T) -> (bool, &'cx mut Cx) {
        (self == other, cx)
    }
}

impl<'cx, 'o, T, U, Cx, O> IsMatch<'cx, 'o, Cx, U> for Alt<'cx, 'o, T, Cx, O>
where T: PatternTreeNode + IsMatch<'cx, 'o, Cx, U> {
    fn is_match(&self, cx: &'cx mut Cx, other: &'o U) -> (bool, &'cx mut Cx) {
        match self {
            Alt::Any => (true, cx),
            Alt::Elmt(e) => e.is_match(cx, other),
            Alt::Named(e, _) => e.is_match(cx, other),
            Alt::Alt(i, j) => {
                let (r_i, cx) = i.is_match(cx, other);
                let (r_j, cx) = j.is_match(cx, other);
                (r_i || r_j, cx)
            }
        }
    }
}

impl<'cx, 'o, T, U, Cx, O> IsMatch<'cx, 'o, Cx, &[U]> for Seq<'cx, 'o, T, Cx, O>
where T: PatternTreeNode + IsMatch<'cx, 'o, Cx, U> {
    fn is_match(&self, cx: &'cx mut Cx, other: &'o &[U]) -> (bool, &'cx mut Cx) {
        let mut cx = cx;
        match self {
            Seq::Any => (other.len() == 1, cx),
            Seq::Elmt(e) => {
                let (r_e, cx) = e.is_match(cx, &other[0]);
                (other.len() == 1 && r_e, cx)
            },
            Seq::Named(e, _) => e.is_match(cx, other),
            Seq::Alt(i, j) => {
                let (r_i, cx) = i.is_match(cx, other);
                let (r_j, cx) = j.is_match(cx, other);
                (r_i || r_j, cx)
            },
            Seq::Empty => (other.is_empty(), cx),
            Seq::Repeat(e, r) => {
                let e_range = e.num_elmts_range();
                let e_range = e_range.start..e_range.end.unwrap_or(other.len()+1);

                if r.start == 0 && other.is_empty() {
                    return (true, cx);
                }

                for i in r.start..r.end.unwrap_or(other.len()+1) {
                    
                    let iterators = repeat_n(e_range.clone(), i)
                        .multi_cartesian_product()
                        .filter(|x| x.iter().sum::<usize>() == other.len());

                    'outer: for vals in iterators {
                        let mut skip = 0;
                        for v in vals.iter() {
                            
                            let (r_e, cx_tmp) = e.is_match(cx, &&other[skip..skip+v]);
                            cx = cx_tmp;
                            if !r_e {
                                continue 'outer;
                            }
                            skip += v;
                        }
                        return (true, cx);
                    }
                }

                (false, cx)
            },
            Seq::Seq(a, b) => {
                let mut cx = cx;
                let range = a.num_elmts_range();
                for i in range.start..range.end.unwrap_or(other.len()+1) {
                    if i > other.len() {
                        break;
                    }
                    let (l, r) = other.split_at(i);
                    let (r_a, cx_tmp) = a.is_match(cx, &l);
                    cx = cx_tmp;
                    let (r_b, cx_tmp) = b.is_match(cx, &r);
                    cx = cx_tmp;
                    if r_a && r_b {
                        return (true, cx);
                    }
                }
                (false, cx)
            },
            
        }
    }
}

impl<'cx, 'o, T, U, Cx, O> IsMatch<'cx, 'o, Cx, Vec<U>> for Seq<'cx, 'o, T, Cx, O>
where T: PatternTreeNode + IsMatch<'cx, 'o, Cx, U> {
    fn is_match(&self, cx: &'cx mut Cx, other: &'o Vec<U>) -> (bool, &'cx mut Cx) {
        self.is_match(cx, &other.as_slice())
    }
}


impl<'cx, 'o, T, U, Cx, O> IsMatch<'cx, 'o, Cx, Option<U>> for Opt<'cx, 'o, T, Cx, O>
where T: PatternTreeNode + IsMatch<'cx, 'o, Cx, U> {
    fn is_match(&self, cx: &'cx mut Cx, other: &'o Option<U>) -> (bool, &'cx mut Cx) {
        
        match self {
            Opt::Any => (other.is_some(), cx),
            Opt::Elmt(e) => match other {
                Some(other) => e.is_match(cx, other),
                None => (false, cx)
            },
            Opt::Named(e, _) => e.is_match(cx, other),
            Opt::Alt(a, b) => {
                let (r_a, cx) = a.is_match(cx, other);
                let (r_b, cx) = b.is_match(cx, other);
                (r_a && r_b, cx)
            },
            Opt::None => (other.is_none(), cx),
        }
    }
}