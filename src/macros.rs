#![allow(unused_macros)]
use crate::matchers::{Alternative, Sequence};


macro_rules! any {
    () => {
        crate::matchers::Alternative(vec!()).into()
    };
    ( $( $element:expr ) , * ) => {
        crate::matchers::Alternative(vec!( $($element ,)*) ).into()
    };
}


macro_rules! seq {
    ( $( $element:expr ; $repeat:expr ) , * ) => {
        crate::matchers::Sequence(vec!(
            $( 
                crate::repeat::Repeat { 
                    elmt: $element, 
                    range: crate::repeat::RepeatRange::from($repeat)
                }
            ),*
        ))
    };
}

macro_rules! opt {
    ( $element:expr ) => {
        crate::matchers::Optional(Some($element))
    };
    () => {
        crate::matchers::Optional(None)
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_any_allow_all() {
        //let x = any!(1, 2, 3);
        //let y = seq!(1; .., 2; 4);
    }

    #[test]
    fn test_any_allow_some() {
    }
}