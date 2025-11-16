use crate::environment::Environment;
use crate::lox_function::LoxFunction;
use crate::runtime_error::Return;
use crate::{interpreter::Interpreter, token::Token, value::Value};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub enum LoxCallable {
    Function(LoxFunction),
    NativeFunction(LoxNativeFunction),
}

impl LoxCallable {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Box<Value>>,
    ) -> Result<Value, Box<dyn Error>> {
        match self {
            LoxCallable::Function(f) => {
                let environment = Environment::new_enclosing(f.closure.clone());
                for (i, param) in f.declaration.params.iter().enumerate() {
                    environment
                        .borrow_mut()
                        .define(param.lexeme.clone(), *arguments.get(i).unwrap().clone());
                }

                match interpreter.execute_block(&f.declaration.body, environment) {
                    Ok(value) => Ok(value),
                    Err(e) => match e.downcast::<Return>() {
                        Ok(value) => return Ok(value.0),
                        Err(e) => Err(e),
                    },
                }?;
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

impl Display for LoxCallable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxCallable::Function(func) => {
                write!(f, "<fn {}>", func.declaration.name.lexeme)
            }
            LoxCallable::NativeFunction(func) => {
                write!(f, "<native fn>")
            }
        }
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
