use crate::stmt::LoxFunctionNode;

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub(crate) declaration: Box<LoxFunctionNode>,
}

impl LoxFunction {
    pub fn new(declaration: Box<LoxFunctionNode>) -> Self {
        Self { declaration }
    }
}