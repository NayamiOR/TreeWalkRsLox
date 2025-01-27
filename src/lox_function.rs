use crate::stmt::Stmt;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
