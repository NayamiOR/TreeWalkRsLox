use crate::{token::Token, value::Value};

pub trait RuntimeErrorTrait {
    fn token(&self) -> &Token;
    fn message(&self) -> &str;
}

impl RuntimeErrorTrait for RuntimeError {
    fn token(&self) -> &Token {
        &self.token
    }

    fn message(&self) -> &str {
        &self.message
    }
}

pub(crate) struct RuntimeError {
    pub(crate) token: Token,
    pub(crate) message: String,
}

impl RuntimeError {
    pub(crate) fn new(token: Token, message: String) -> RuntimeError {
        RuntimeError { token, message }
    }
}

pub(crate) struct Return(pub(crate) Value,RuntimeError);