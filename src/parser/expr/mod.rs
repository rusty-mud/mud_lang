use crate::parser::expr::precedence_group::addition::parse_addition_level_expression;
use nom::{combinator::map, error::VerboseError, IResult};

mod literal;
mod operator;
mod precedence_group;

use literal::Literal;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // Exponentiation-level operators
    Exponentiation(Box<Expr>, Box<Expr>),

    // Multiplication-level operators
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),

    // Additional-level operators
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),

    Constant(Literal),
}

type ExprResult<'a> = IResult<&'a str, Expr, VerboseError<&'a str>>;

fn parse_constant<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    map(literal::parse_literal, |literal| Expr::Constant(literal))(i)
}

pub fn parse_expression<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    parse_addition_level_expression(i)
}

impl std::convert::From<i64> for Expr {
    fn from(val: i64) -> Expr {
        Expr::Constant(Literal::Num(val))
    }
}
