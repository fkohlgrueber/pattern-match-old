

macro_rules! any {
    () => {
        crate::matchers::MatchValues { values: None }
    };
    ( $( $element:expr ) , * ) => {
        {
            crate::matchers::MatchValues { 
                values: Some( vec!($($element ,)*) )
            }
        }
    };
}


macro_rules! seq {
    () => {
        crate::matchers::MatchSequences {
            seq: vec!(
                crate::repeat::Repeat {
                    elmt: crate::matchers::MatchValues { values: None }, 
                    range: crate::repeat::RepeatRange::from(..)
                }
            )
        }
    };
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

    #[test]
    fn test_seq_empty() {
        let m: crate::matchers::MatchSequences<crate::matchers::MatchValues<char>> = seq!();
        assert_eq!(m.seq.len(), 1);
        assert_eq!(m.seq[0].elmt.values, None);
        assert_eq!(m.seq[0].range, crate::repeat::RepeatRange { start: 0, end: None});
    }
}