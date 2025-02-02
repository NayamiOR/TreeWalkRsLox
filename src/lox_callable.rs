use crate::stmt::LoxFunctionNode;
use crate::{interpreter::Interpreter, token::Token, value::Value};
use std::fmt::{Debug, Display, Formatter};
use crate::environment::Environment;
use crate::lox_function::LoxFunction;
use crate::runtime_error::RuntimeError;

#[derive(Clone, Debug)]
pub enum LoxCallable {
    Function(LoxFunction),
    NativeFunction(LoxNativeFunction),
}

impl LoxCallable {
    pub fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Box<Value>>) -> Result<Value, RuntimeError> {
        match self {
            LoxCallable::Function(f) => {
                let mut environment = Environment::new_enclosing(interpreter.globals.clone());
                for (i, param) in f.declaration.params.iter().enumerate() {
                    environment.borrow_mut().define(param.lexeme.clone(), *arguments.get(i).unwrap().clone());
                }

                interpreter.execute_block(&f.declaration.body, environment)?;
                Ok(Value::Nil)
            }
            LoxCallable::NativeFunction(f) => Ok((f.function)(interpreter)),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::Function(f) => f.declaration.params.len(),
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
