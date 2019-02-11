
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
