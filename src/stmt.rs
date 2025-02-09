use crate::expr::Expr;
use crate::token::Token;

pub(crate) trait Visitor<R> {
    fn visit_expression_stmt(&mut self, expr: &Expr) -> R;
    fn visit_print_stmt(&mut self, expr: &Expr) -> R;
    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> R;
    fn visit_var_stmt(&mut self, name: &Token, initializer: Option<&Expr>) -> R;
    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> R;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> R;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> R;
    fn visit_function_stmt(&mut self, stmt: Box<LoxFunctionNode>) -> R;
}

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Expression {
        expression: Box<Expr>,
    },
    Print {
        expression: Box<Expr>,
    },
    Return {
        keyword: Token,
        value: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Box<Expr>>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
    Function {
        function: Box<LoxFunctionNode>,
        // name: Token,
        // params: Vec<Token>,
        // body: Vec<Stmt>,
    },
}

impl Stmt {
    pub(crate) fn accept<R>(&self, visitor: &mut impl Visitor<R>) -> R {
        match self {
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Return { keyword, value } => visitor.visit_return_stmt(keyword, value),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer.as_deref()),
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_stmt(condition, then_branch, else_branch.as_deref()),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Function { function } => visitor.visit_function_stmt(function.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoxFunctionNode {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}