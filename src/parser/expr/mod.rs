use crate::parser::expr::precedence_group::addition::parse_addition_level_expression;
use nom::{combinator::map, error::VerboseError, IResult};

mod literal;
mod precedence_group;

use literal::Literal;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // Multiplication-levle operations
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),

    // Additional-level operations
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
