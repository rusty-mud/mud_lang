use crate::parser::{expr::Expr, parse};

#[test]
fn can_parse_exponentiation_of_unsigned_integers_without_spaces() -> Result<(), String> {
    assert_eq!(
        parse("56^89")?,
        Expr::Exponentiation(Box::from(Expr::from(56)), Box::from(Expr::from(89)))
    );
    Ok(())
}

#[test]
fn can_parse_exponentiation_of_unsigned_integers_with_spaces() -> Result<(), String> {
    assert_eq!(
        parse("56 ^ 89")?,
        Expr::Exponentiation(Box::from(Expr::from(56)), Box::from(Expr::from(89)))
    );
    Ok(())
}

#[test]
fn can_parse_exponentiation_of_negative_integers() -> Result<(), String> {
    assert_eq!(
        parse("-56 ^ -89")?,
        Expr::Exponentiation(Box::from(Expr::from(-56)), Box::from(Expr::from(-89)))
    );
    Ok(())
}

#[test]
fn exponentiation_is_right_associative() -> Result<(), String> {
    assert_eq!(
        parse("2 ^ 3 ^ 4")?,
        Expr::Exponentiation(
            Box::from(Expr::from(2)),
            Box::from(Expr::Exponentiation(
                Box::from(Expr::from(3)),
                Box::from(Expr::from(4))
            )),
        )
    );
    Ok(())
}
