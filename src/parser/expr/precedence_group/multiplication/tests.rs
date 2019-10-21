use crate::parser::{expr::Expr, parse};

#[test]
fn can_parse_multiplication_of_unsigned_integers_without_spaces() -> Result<(), String> {
    assert_eq!(
        parse("56*89")?,
        Expr::Multiplication(Box::from(Expr::from(56)), Box::from(Expr::from(89)))
    );
    Ok(())
}

#[test]
fn can_parse_division_of_unsigned_integers_without_spaces() -> Result<(), String> {
    assert_eq!(
        parse("56-89")?,
        Expr::Subtraction(Box::from(Expr::from(56)), Box::from(Expr::from(89)))
    );
    Ok(())
}

#[test]
fn can_parse_multiplication_of_unsigned_integers_with_spaces() -> Result<(), String> {
    assert_eq!(parse("56 * 89")?, Expr::from(56) * Expr::from(89));
    Ok(())
}

#[test]
fn can_parse_division_of_unsigned_integers_with_spaces() -> Result<(), String> {
    assert_eq!(parse("56 / 89")?, Expr::from(56) / Expr::from(89));
    Ok(())
}

#[test]
fn can_parse_division_of_negative_integers() -> Result<(), String> {
    assert_eq!(parse("-56 / -89")?, Expr::from(-56) / Expr::from(-89));
    Ok(())
}

#[test]
fn multiplication_is_left_associative() -> Result<(), String> {
    assert_eq!(
        parse("2 * 3 * 4")?,
        (Expr::from(2) * Expr::from(3)) * Expr::from(4)
    );
    Ok(())
}

#[test]
fn division_is_left_associative() -> Result<(), String> {
    assert_eq!(
        parse("2 / 3 / 4")?,
        Expr::Division(
            Box::from(Expr::Division(
                Box::from(Expr::from(2)),
                Box::from(Expr::from(3))
            )),
            Box::from(Expr::from(4))
        )
    );
    Ok(())
}
