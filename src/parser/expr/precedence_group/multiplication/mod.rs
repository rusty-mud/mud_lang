use crate::parser::expr::parse_constant;
use crate::parser::expr::{Expr, ExprResult};
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
            parse_constant,
            many0(tuple((
                multispace0,
                alt((tag("*"), tag("/"))),
                multispace0,
                parse_constant,
            ))),
        )),
        |(lhs, vec_rhs)| {
            println!("lhs: {:?}", lhs);
            vec_rhs
                .into_iter()
                .fold(lhs, |acc, (_, op, _, next_expr)| match op {
                    "*" => Expr::Multiplication(Box::from(acc), Box::from(next_expr)),
                    "/" => Expr::Division(Box::from(acc), Box::from(next_expr)),
                    _ => panic!("invalid op"),
                })
        },
    )(i)
}
