use crate::parser::{
    expr::{Expr, Literal},
    parse,
};

#[test]
fn can_parse_multiplication_of_unsigned_integers_without_spaces() -> Result<(), String> {
    use Literal::Num;

    assert_eq!(
        parse("56*89")?,
        Expr::Multiplication(
            Box::from(Expr::Constant(Num(56))),
            Box::from(Expr::Constant(Num(89)))
        )
    );
    Ok(())
}

#[test]
fn can_parse_division_of_unsigned_integers_without_spaces() -> Result<(), String> {
    use Literal::Num;

    assert_eq!(
        parse("56-89")?,
        Expr::Subtraction(
            Box::from(Expr::Constant(Num(56))),
            Box::from(Expr::Constant(Num(89)))
        )
    );
    Ok(())
}

#[test]
fn can_parse_multiplication_of_unsigned_integers_with_spaces() -> Result<(), String> {
    use Literal::Num;

    assert_eq!(
        parse("56 * 89")?,
        Expr::Multiplication(
            Box::from(Expr::Constant(Num(56))),
            Box::from(Expr::Constant(Num(89)))
        )
    );
    Ok(())
}

#[test]
fn can_parse_division_of_unsigned_integers_with_spaces() -> Result<(), String> {
    use Literal::Num;

    assert_eq!(
        parse("56 / 89")?,
        Expr::Division(
            Box::from(Expr::Constant(Num(56))),
            Box::from(Expr::Constant(Num(89)))
        )
    );
    Ok(())
}

#[test]
fn can_parse_division_of_negative_integers() -> Result<(), String> {
    use Literal::Num;
    assert_eq!(
        parse("-56 / -89")?,
        Expr::Division(
            Box::from(Expr::Constant(Num(-56))),
            Box::from(Expr::Constant(Num(-89)))
        )
    );
    Ok(())
}

#[test]
fn multiplication_is_left_associative() -> Result<(), String> {
    use Literal::Num;
    assert_eq!(
        parse("2 * 3 * 4")?,
        Expr::Multiplication(
            Box::from(Expr::Multiplication(
                Box::from(Expr::Constant(Num(2))),
                Box::from(Expr::Constant(Num(3)))
            )),
            Box::from(Expr::Constant(Num(4)))
        )
    );
    Ok(())
}

#[test]
fn division_is_left_associative() -> Result<(), String> {
    use Literal::Num;
    assert_eq!(
        parse("2 / 3 / 4")?,
        Expr::Division(
            Box::from(Expr::Division(
                Box::from(Expr::Constant(Num(2))),
                Box::from(Expr::Constant(Num(3)))
            )),
            Box::from(Expr::Constant(Num(4)))
        )
    );
    Ok(())
}
