use crate::{interpreter::Interpreter, stmt::Stmt, token::Token, value::Value};
use std::fmt::Debug;

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

// pub trait LoxCallable: Clone + Debug + PartialEq + PartialOrd {
//     fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value;
//     fn arity(&self) -> usize;
// }

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

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub name: String,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

// impl PartialEq for LoxFunction {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }
//
// impl PartialOrd for LoxFunction {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         todo!()
//     }
// }
