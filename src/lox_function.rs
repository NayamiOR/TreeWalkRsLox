use crate::environment::Environment;
use crate::stmt::LoxFunctionNode;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub(crate) declaration: Box<LoxFunctionNode>,
    pub(crate) closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: Box<LoxFunctionNode>, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}
