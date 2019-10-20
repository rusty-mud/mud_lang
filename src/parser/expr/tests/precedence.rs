use crate::parser::{
    expr::{Expr, Literal},
    parse,
};

#[test]
fn multiplication_has_higher_precedence_than_addition() -> Result<(), String> {
    use Literal::Num;
    assert_eq!(
        parse("2 + 3 * 4")?,
        Expr::Addition(
            Box::from(Expr::Constant(Num(2))),
            Box::from(Expr::Multiplication(
                Box::from(Expr::Constant(Num(3))),
                Box::from(Expr::Constant(Num(4)))
            )),
        )
    );
    Ok(())
}
