use crate::pattern_tree_old::*;
use crate::matchers::IsMatch;

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
