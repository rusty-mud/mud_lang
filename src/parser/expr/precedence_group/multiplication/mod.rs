use crate::parser::expr::precedence_group::exponentiation::parse_exponentiation_level_expression;
use crate::parser::expr::{operator::Operator, ExprResult};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::map,
    multi::many0, sequence::tuple,
};

#[cfg(test)]
mod tests;

// Multiplication level expressions
// *, /, and %
pub fn parse_multiplication_level_expression<'a>(i: &'a str) -> ExprResult<'a> {
    map(
        tuple((
            parse_exponentiation_level_expression,
            many0(tuple((
                multispace0,
                alt((tag("*"), tag("/"))),
                multispace0,
                parse_exponentiation_level_expression,
            ))),
        )),
        |(lhs, vec_rhs)| {
            vec_rhs.into_iter().fold(lhs, |acc, (_, op, _, rhs)| {
                Operator::create_expr(op, acc, rhs)
            })
        },
    )(i)
}
