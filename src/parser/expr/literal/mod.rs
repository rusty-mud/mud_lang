use crate::parser::Span;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    error::VerboseError,
    sequence::preceded,
    IResult,
};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Num(i64),
    Str(String),
}

type LiteralResult<'a> = IResult<Span<'a>, Literal, VerboseError<Span<'a>>>;

fn parse_boolean<'a>(i: Span<'a>) -> LiteralResult<'a> {
    alt((
        map(tag("true"), |_| Literal::Bool(true)),
        map(tag("false"), |_| Literal::Bool(false)),
    ))(i)
}

fn parse_num<'a>(i: Span<'a>) -> LiteralResult<'a> {
    alt((
        map_res(digit1, |digit_str: Span| {
            digit_str.fragment.parse::<i64>().map(Literal::Num)
        }),
        map_res(preceded(tag("+"), digit1), |digit_str: Span| {
            digit_str.fragment.parse::<i64>().map(Literal::Num)
        }),
        map(preceded(tag("-"), digit1), |digit_str: Span| {
            Literal::Num(-1 * digit_str.fragment.parse::<i64>().unwrap())
        }),
    ))(i)
}

pub fn parse_literal<'a>(i: Span<'a>) -> LiteralResult<'a> {
    alt((parse_boolean, parse_num))(i)
}
