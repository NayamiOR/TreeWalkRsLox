use crate::stmt::LoxFunctionNode;

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub(crate) declaration: LoxFunctionNode,
}

impl LoxFunction {
    pub fn new(declaration: LoxFunctionNode) -> Self {
        Self { declaration }
    }
}