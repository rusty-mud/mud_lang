use crate::parser::{expr::Expr, parse};

#[test]
fn multiplication_has_higher_precedence_than_addition() -> Result<(), String> {
    assert_eq!(
        parse("2 + 3 * 4")?,
        Expr::from(2) + (Expr::from(3) * Expr::from(4))
    );
    Ok(())
}
