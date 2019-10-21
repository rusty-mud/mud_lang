use crate::parser::{expr::Expr, parse};

#[test]
fn can_parse_addition_of_unsigned_integers_without_spaces() -> Result<(), String> {
    assert_eq!(parse("56+89")?, Expr::from(56) + Expr::from(89));
    Ok(())
}

#[test]
fn can_parse_subtraction_of_unsigned_integers_without_spaces() -> Result<(), String> {
    assert_eq!(parse("56-89")?, Expr::from(56) - Expr::from(89));
    Ok(())
}

#[test]
fn can_parse_addition_of_unsigned_integers_with_spaces() -> Result<(), String> {
    assert_eq!(parse("56 + 89")?, Expr::from(56) + Expr::from(89));
    Ok(())
}

#[test]
fn can_parse_subtraction_of_unsigned_integers_with_spaces() -> Result<(), String> {
    assert_eq!(parse("56 - 89")?, Expr::from(56) - Expr::from(89));
    Ok(())
}

#[test]
fn can_parse_subtraction_of_negative_integers() -> Result<(), String> {
    assert_eq!(parse("-56 - -89")?, Expr::from(-56) - Expr::from(-89));
    Ok(())
}

#[test]
fn addition_is_left_associative() -> Result<(), String> {
    assert_eq!(
        parse("2 + 3 + 4")?,
        (Expr::from(2) + Expr::from(3)) + Expr::from(4)
    );
    Ok(())
}

#[test]
fn subtraction_is_left_associative() -> Result<(), String> {
    assert_eq!(
        parse("2 - 3 - 4")?,
        (Expr::from(2) - Expr::from(3)) - Expr::from(4)
    );
    Ok(())
}
