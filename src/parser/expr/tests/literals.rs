use crate::parser::{
    expr::{Expr, Literal},
    parse,
};

#[test]
fn can_parse_an_unsigned_integer() -> Result<(), String> {
    let val = parse("952345234")?;

    assert_eq!(val, Expr::Constant(Literal::Num(952345234)));
    Ok(())
}

#[test]
fn can_parse_an_unsigned_integer_with_plus() -> Result<(), String> {
    let val = parse("+4235543435")?;

    assert_eq!(val, Expr::Constant(Literal::Num(4235543435)));
    Ok(())
}
