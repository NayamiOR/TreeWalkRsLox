use crate::lox_function::LoxFunction;
use crate::{interpreter::Interpreter, token::Token, value::Value};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub enum LoxCallable {
    Function(LoxFunction),
    NativeFunction(LoxNativeFunction),
}

impl LoxCallable {
    pub fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Box<Value>>) -> Value {
        match self {
            LoxCallable::Function(f) => {
                todo!()
            }
            LoxCallable::NativeFunction(f) => (f.function)(interpreter),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::Function(f) => f.params.len(),
            LoxCallable::NativeFunction(f) => f.params.len(),
        }
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

impl PartialOrd for LoxCallable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct LoxNativeFunction {
    pub name: String,
    pub params: Vec<Token>,
    pub function: fn(&mut Interpreter) -> Value,
}
impl PartialEq for LoxNativeFunction {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for LoxNativeFunction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Display for LoxNativeFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}
