use crate::parser::expr::parse_expression;
use crate::parser::expr::Expr;
use nom::error::{VerboseError, VerboseErrorKind};
// use nom::error::convert_error;
use nom_locate::LocatedSpan;

mod expr;

type Span<'a> = LocatedSpan<&'a str>;

pub fn parse<'a>(source: &'a str) -> Result<Expr, String> {
    parse_expression(Span::new(source))
        .map(|(_, expr)| expr)
        .map_err(|err| match err {
            nom::Err::Incomplete(_) => "Incomplete input".to_string(),
            nom::Err::Error(e) => convert_error(source, e),
            nom::Err::Failure(e) => convert_error(source, e),
        })
}

fn convert_error<'a>(_source: &'a str, verbose_errors: VerboseError<Span<'a>>) -> String {
    verbose_errors
        .errors
        .iter()
        .fold("".to_owned(), |acc, (_substring, kind)| {
            format!(
                "{}\n{}",
                acc,
                match kind {
                    VerboseErrorKind::Char(c) =>
                        format!("??: expected '{}', got empty input\n\n", c),
                    VerboseErrorKind::Context(s) => format!("??: in {}, got empty input\n\n", s),
                    VerboseErrorKind::Nom(e) => format!("??: in {:?}, got empty input\n\n", e),
                }
            )
        })
        .to_string()
}
