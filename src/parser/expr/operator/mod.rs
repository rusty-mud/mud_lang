use crate::parser::expr::Expr;
use std::convert::From;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl From<&str> for Operator {
    fn from(op_str: &str) -> Operator {
        match op_str {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "^" => Operator::Exp,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add for Expr {
    type Output = Self;
    fn add(self, rhs: Expr) -> Expr {
        Expr::Addition(Box::from(self), Box::from(rhs))
    }
}

impl std::ops::Sub for Expr {
    type Output = Self;
    fn sub(self, rhs: Expr) -> Expr {
        Expr::Subtraction(Box::from(self), Box::from(rhs))
    }
}

impl std::ops::Mul for Expr {
    type Output = Self;
    fn mul(self, rhs: Expr) -> Expr {
        Expr::Multiplication(Box::from(self), Box::from(rhs))
    }
}
impl std::ops::Div for Expr {
    type Output = Self;
    fn div(self, rhs: Expr) -> Expr {
        Expr::Division(Box::from(self), Box::from(rhs))
    }
}

impl Operator {
    pub fn create_expr(op_str: &str, lhs: Expr, rhs: Expr) -> Expr {
        match Operator::from(op_str) {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
            Operator::Exp => Expr::Exponentiation(Box::from(lhs), Box::from(rhs)),
        }
    }
}
