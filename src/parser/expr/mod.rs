use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map, map_res},
    error::VerboseError,
    multi::many0,
    sequence::{preceded, tuple},
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

type LiteralResult<'a> = IResult<&'a str, Literal, VerboseError<&'a str>>;
type ExprResult<'a> = IResult<&'a str, Expr, VerboseError<&'a str>>;

fn parse_boolean<'a>(i: &'a str) -> LiteralResult<'a> {
    alt((
        map(tag("true"), |_| Literal::Bool(true)),
        map(tag("false"), |_| Literal::Bool(false)),
    ))(i)
}

fn parse_num<'a>(i: &'a str) -> LiteralResult<'a> {
    alt((
        map_res(digit1, |digit_str: &str| {
            digit_str.parse::<i64>().map(Literal::Num)
        }),
        map_res(preceded(tag("+"), digit1), |digit_str: &str| {
            digit_str.parse::<i64>().map(Literal::Num)
        }),
        map(preceded(tag("-"), digit1), |digit_str: &str| {
            Literal::Num(-1 * digit_str.parse::<i64>().unwrap())
        }),
    ))(i)
}

fn parse_literal<'a>(i: &'a str) -> LiteralResult<'a> {
    alt((parse_boolean, parse_num))(i)
}

fn parse_constant<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    map(parse_literal, |literal| Expr::Constant(literal))(i)
}

// Multiplication level expressions
// *, /, and %
fn parse_multiplication_level_expression<'a>(i: &'a str) -> ExprResult<'a> {
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

// Addition level expressions
// + and -
fn parse_addition_level_expression<'a>(
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
            vec_rhs
                .into_iter()
                .fold(lhs, |acc, (_, op, _, rhs)| match op {
                    "+" => Expr::Addition(Box::from(acc), Box::from(rhs)),
                    "-" => Expr::Subtraction(Box::from(acc), Box::from(rhs)),
                    _ => panic!("invalid op"),
                })
        },
    )(i)
}

pub fn parse_expression<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    parse_addition_level_expression(i)
}
