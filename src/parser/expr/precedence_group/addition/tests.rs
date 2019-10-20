use crate::parser::{
    expr::{Expr, Literal},
    parse,
};

#[test]
fn can_parse_addition_of_unsigned_integers_without_spaces() -> Result<(), String> {
    use Literal::Num;

    assert_eq!(
        parse("56+89")?,
        Expr::Addition(
            Box::from(Expr::Constant(Num(56))),
            Box::from(Expr::Constant(Num(89)))
        )
    );
    Ok(())
}

#[test]
fn can_parse_subtraction_of_unsigned_integers_without_spaces() -> Result<(), String> {
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
fn can_parse_addition_of_unsigned_integers_with_spaces() -> Result<(), String> {
    use Literal::Num;

    assert_eq!(
        parse("56 + 89")?,
        Expr::Addition(
            Box::from(Expr::Constant(Num(56))),
            Box::from(Expr::Constant(Num(89)))
        )
    );
    Ok(())
}

#[test]
fn can_parse_subtraction_of_unsigned_integers_with_spaces() -> Result<(), String> {
    use Literal::Num;

    assert_eq!(
        parse("56 - 89")?,
        Expr::Subtraction(
            Box::from(Expr::Constant(Num(56))),
            Box::from(Expr::Constant(Num(89)))
        )
    );
    Ok(())
}

#[test]
fn can_parse_subtraction_of_negative_integers() -> Result<(), String> {
    use Literal::Num;
    assert_eq!(
        parse("-56 - -89")?,
        Expr::Subtraction(
            Box::from(Expr::Constant(Num(-56))),
            Box::from(Expr::Constant(Num(-89)))
        )
    );
    Ok(())
}

#[test]
fn addition_is_left_associative() -> Result<(), String> {
    use Literal::Num;
    assert_eq!(
        parse("2 + 3 + 4")?,
        Expr::Addition(
            Box::from(Expr::Addition(
                Box::from(Expr::Constant(Num(2))),
                Box::from(Expr::Constant(Num(3)))
            )),
            Box::from(Expr::Constant(Num(4)))
        )
    );
    Ok(())
}

#[test]
fn subtraction_is_left_associative() -> Result<(), String> {
    use Literal::Num;
    assert_eq!(
        parse("2 - 3 - 4")?,
        Expr::Subtraction(
            Box::from(Expr::Subtraction(
                Box::from(Expr::Constant(Num(2))),
                Box::from(Expr::Constant(Num(3)))
            )),
            Box::from(Expr::Constant(Num(4)))
        )
    );
    Ok(())
}
