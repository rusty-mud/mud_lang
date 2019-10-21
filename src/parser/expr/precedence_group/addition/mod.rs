use crate::parser::expr::operator::Operator;
use crate::parser::expr::precedence_group::multiplication::parse_multiplication_level_expression;
use crate::parser::expr::Expr;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::map,
    error::VerboseError, multi::many0, sequence::tuple, IResult,
};

#[cfg(test)]
mod tests;

// Addition level expressions
// + and -
pub fn parse_addition_level_expression<'a>(
    i: &'a str,
) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    map(
        tuple((
            parse_multiplication_level_expression,
            many0(tuple((
                multispace0,
                alt((tag("+"), tag("-"))),
                multispace0,
                parse_multiplication_level_expression,
            ))),
        )),
        |(lhs, vec_rhs)| {
            vec_rhs.into_iter().fold(lhs, |acc, (_, op, _, rhs)| {
                Operator::create_expr(op, acc, rhs)
            })
        },
    )(i)
}
