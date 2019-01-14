
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

#[derive(PartialEq, Debug)]
pub struct RepeatRange {
    pub start: usize,
    pub end: Option<usize>  // exclusive
}

impl From<Range<usize>> for RepeatRange {
    fn from(range: Range<usize>) -> Self {
        RepeatRange { 
            start: range.start, 
            end: Some(range.end)
        }
    }
}

impl From<RangeFrom<usize>> for RepeatRange {
    fn from(range: RangeFrom<usize>) -> Self {
        RepeatRange { 
            start: range.start, 
            end: None
        }
    }
}

impl From<RangeFull> for RepeatRange {
    fn from(_: RangeFull) -> Self {
        RepeatRange { 
            start: 0, 
            end: None
        }
    }
}

impl From<RangeInclusive<usize>> for RepeatRange {
    fn from(range: RangeInclusive<usize>) -> Self {
        RepeatRange { 
            start: *range.start(), 
            end: Some(range.end()+1)
        }
    }
}

impl From<RangeTo<usize>> for RepeatRange {
    fn from(range: RangeTo<usize>) -> Self {
        RepeatRange { 
            start: 0, 
            end: Some(range.end)
        }
    }
}


impl From<RangeToInclusive<usize>> for RepeatRange {
    fn from(range: RangeToInclusive<usize>) -> Self {
        RepeatRange { 
            start: 0, 
            end: Some(range.end+1)
        }
    }
}

impl From<usize> for RepeatRange {
    fn from(n: usize) -> Self {
        RepeatRange { 
            start: n, 
            end: Some(n+1)
        }
    }
}


pub struct Repeat<T> {
    pub elmt: T,
    pub range: RepeatRange,
}
