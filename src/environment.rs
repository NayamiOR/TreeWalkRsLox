use crate::runtime_error::RuntimeError;
use crate::token::Token;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct Environment {
    pub(crate) values: HashMap<String, Value>, // 变量名到值的映射
    pub(crate) enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    pub(crate) fn new_enclosing(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }

    pub(crate) fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub(crate) fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(RuntimeError {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", &name.lexeme),
        })
    }

    pub(crate) fn get(&self, name: &Token) -> Result<Value, Box<RuntimeError>> {
        if let Some(v) = self.values.get(&name.lexeme) {
            return Ok(v.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }
        Err(RuntimeError {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", &name.lexeme),
        }
        .into())
    }
}
