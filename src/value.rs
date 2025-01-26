use std::{any::Any, fmt::Display, rc::Rc};

use crate::lox_callable::LoxCallable;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    // Callable(Box<dyn LoxCallable>),
    Callable(Box<LoxCallable>),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
            Value::Callable(c) => write!(f, "{:?}", c),
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Value::Number(-n),
            _ => panic!("Unary negation is only defined for numbers"),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
            (Value::String(mut l), Value::String(r)) => {
                l.push_str(&r);
                Value::String(l)
            }
            _ => panic!("Addition is only defined for two numbers or two strings"),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
            _ => panic!("Subtraction is only defined for two numbers"),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
            _ => panic!("Multiplication is only defined for two numbers"),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l / r),
            _ => panic!("Division is only defined for two numbers"),
        }
    }
}

impl AsRef<bool> for Value {
    fn as_ref(&self) -> &bool {
        match self {
            Value::Boolean(b) => b,
            Value::Nil => &false,
            _ => &true,
        }
    }
}
