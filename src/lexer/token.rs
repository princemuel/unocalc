use nom::*;
use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    Number(f64),
    Operator(char),
    Paren(char),
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Tokens<'a> {
    pub token: &'a [Token],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(input: &'a [Token]) -> Self {
        Tokens { token: input, start: 0, end: input.len() }
    }
}

impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.token.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        Tokens { token: &self.token[0..count], start: 0, end: count }
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.token.split_at(count);
        let first = Tokens { token: prefix, start: 0, end: prefix.len() };
        let second = Tokens { token: suffix, start: 0, end: suffix.len() };
        (second, first)
    }
}

impl InputLength for Token {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            token: self.token.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}
impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}
impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}
impl<'a> Slice<RangeFull> for Tokens<'a> {
    #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        Tokens { token: self.token, start: self.start, end: self.end }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token;
    type Iter = Enumerate<::std::slice::Iter<'a, Token>>;
    type IterElem = ::std::slice::Iter<'a, Token>;

    #[inline]
    fn iter_indices(&self) -> Enumerate<::std::slice::Iter<'a, Token>> {
        self.token.iter().enumerate()
    }

    #[inline]
    fn iter_elements(&self) -> ::std::slice::Iter<'a, Token> {
        self.token.iter()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.token.iter().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.token.len() >= count {
            Ok(count)
        } else {
            Err(Needed::Unknown)
        }
    }
}
