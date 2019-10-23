use crate::parser::expr::parse_constant;
use crate::parser::expr::{operator::Operator, ExprResult};
use crate::parser::Span;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::map,
    sequence::tuple,
};

#[cfg(test)]
mod tests;

// Exponentiation level expressions
// ^
pub fn parse_exponentiation_level_expression<'a>(i: Span<'a>) -> ExprResult<'a> {
    alt((
        map(
            tuple((
                parse_constant,
                multispace0,
                tag("^"),
                multispace0,
                parse_exponentiation_level_expression,
            )),
            |(lhs, _, op_span, _, rhs)| Operator::create_expr(op_span.fragment, lhs, rhs),
        ),
        parse_constant,
    ))(i)
}
