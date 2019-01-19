

macro_rules! any {
    () => {
        crate::matchers::MatchValues { values: None, name: None }
    };
    ( $( $element:expr ) , * ) => {
        {
            crate::matchers::MatchValues { 
                values: Some( vec!($($element ,)*) ), name: None
            }
        }
    };
    ( $( $element:expr ) , * ; $name:expr ) => {
        {
            crate::matchers::MatchValues { 
                values: Some( vec!($($element ,)*) ), name: Some($name)
            }
        }
    };
}


macro_rules! seq {
    ( $( $element:expr ; $repeat:expr ) , * ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(
                    crate::repeat::Repeat {
                        elmt: $element, 
                        range: crate::repeat::RepeatRange::from($repeat) });
            )*
            crate::matchers::MatchSequences {
                seq: v
            }
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_any_allow_all() {
        let m: crate::matchers::MatchValues<char> = any!();
        assert_eq!(m.values, None);
    }

    #[test]
    fn test_any_allow_some() {
        let m = any!('a', 'b');
        assert_eq!(m.values, Some(vec!('a', 'b')));
    }
}