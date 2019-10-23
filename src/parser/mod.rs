use crate::parser::expr::parse_expression;
use crate::parser::expr::Expr;
// use nom::error::convert_error;
use nom_locate::LocatedSpan;

mod expr;

type Span<'a> = LocatedSpan<&'a str>;

pub fn parse<'a>(source: &'a str) -> Result<Expr, String> {
    parse_expression(Span::new(source))
        .map(|(_, expr)| expr)
        .map_err(|err| match err {
            nom::Err::Incomplete(_) => "Incomplete input".to_string(),
            nom::Err::Error(_e) => "placeholder1".to_string(), //convert_error(source.fragment, e),
            nom::Err::Failure(_e) => "placeholder2".to_string(), //convert_error(source.fragment, e),
        })
}
