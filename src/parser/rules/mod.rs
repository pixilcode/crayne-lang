use crate::parser::tokens::Span;

use nom::{
    IResult,
    InputTakeAtPosition,
    AsChar,
    combinator::{
        verify
    },
    sequence::{
        preceded,
    },
    character::complete::{
        multispace0,
    },
    bytes::complete::take_while1
};

/// Ignore any preceding whitespace
fn ws<'a, F, I, O>(parser: F) -> impl Fn(I) -> IResult<I, O>
where F: Fn(I) -> IResult<I, O>,
      I: InputTakeAtPosition,
      <I as InputTakeAtPosition>::Item: AsChar + Clone
{
    preceded(multispace0, parser)
}

/// Match any identifier
/// 
/// An identifier is a sequence of characters where
/// the first character is alphabetic or `_`
/// character and each subsequent character is an
/// alphanumeric character or `_`
fn identifier<'a>(input: Span<'a>) -> IResult<Span<'a>, Span<'a>> {
    verify(
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
        |slice: &Span<'a>|
            slice.as_slice().chars().nth(0).unwrap().is_alphabetic() ||
            slice.as_slice().chars().nth(0).unwrap() == '_'
    )(input)
}
