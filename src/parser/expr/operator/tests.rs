use super::Operator;

#[test]
fn can_convert_strings_to_operators() {
    assert_eq!(Operator::from("+"), Operator::Add);
    assert_eq!(Operator::from("-"), Operator::Sub);
    assert_eq!(Operator::from("*"), Operator::Mul);
    assert_eq!(Operator::from("/"), Operator::Div);
}

#[test]
#[should_panic]
fn operator_from_trait_panics_on_bad_input() {
    Operator::from("not an operator");
}
