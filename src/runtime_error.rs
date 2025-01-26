use crate::{token::Token, value::Value};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub(crate) struct RuntimeError {
    pub(crate) token: Token,
    pub(crate) message: String,
}

impl RuntimeError {
    pub(crate) fn new(token: Token, message: String) -> RuntimeError {
        RuntimeError { token, message }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RuntimeError {}

#[derive(Debug)]
pub(crate) struct Return(pub(crate) Value);

impl Return {
    pub(crate) fn new(value: Value) -> Self {
        Return(value)
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return: {}", self.0)
    }
}

impl Error for Return {}
