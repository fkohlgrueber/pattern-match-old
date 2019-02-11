use crate::matchers::*;

// Trait that has to be implemented on all types that can be used in a pattern tree
pub trait PatternTreeNode {}

impl PatternTreeNode for char {}
impl PatternTreeNode for u128 {}
impl PatternTreeNode for bool {}

impl IsMatchEquality for u128 {}
impl IsMatchEquality for char {}
impl IsMatchEquality for bool {}

