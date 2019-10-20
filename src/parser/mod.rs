use crate::parser::expr::parse_expression;
use crate::parser::expr::Expr;
use nom::error::convert_error;

mod expr;

pub fn parse<'a>(source: &'a str) -> Result<Expr, String> {
    parse_expression(source)
        .map(|(_, expr)| expr)
        .map_err(|err| match err {
            nom::Err::Incomplete(_) => "Incomplete input".to_string(),
            nom::Err::Error(e) => convert_error(source, e),
            nom::Err::Failure(e) => convert_error(source, e),
        })
}
