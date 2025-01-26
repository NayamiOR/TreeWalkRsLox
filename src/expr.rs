use std::vec;

use crate::token::Token;

pub(crate) trait Visitor<R> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> R;
    fn visit_literal_expr(&mut self, value: &crate::token::Literal) -> R;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> R;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Box<Expr>>) -> R;
    fn visit_variable_expr(&mut self, name: &Token) -> R;
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> R;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: crate::token::Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub(crate) fn accept<R>(&self, visitor: &mut impl Visitor<R>) -> R {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expr(left, operator, right),
        }
    }
}
