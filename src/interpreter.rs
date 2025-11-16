use crate::environment::Environment;
use crate::expr::Expr;
use crate::lox_callable::LoxCallable;
use crate::lox_function::LoxFunction;
use crate::native_functions::global_env;
use crate::runtime_error::{Return, RuntimeError};
use crate::stmt::{LoxFunctionNode, Stmt};
use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use crate::value::Value;
use crate::value::Value::*;
use crate::Lox;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub(crate) struct Interpreter {
    pub(crate) globals: Rc<RefCell<Environment>>,
    pub(crate) environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        let globals = global_env();
        let environment = Rc::clone(&globals);

        Self {
            globals,
            environment,
        }
    }
    pub(crate) fn interpret(&mut self, statements: Vec<Stmt>) {
        for i in statements {
            if let Err(e) = self.execute(&i) {
                Lox::runtime_error(*e.downcast::<RuntimeError>().unwrap());
                return;
            }
        }
    }
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, Box<dyn Error>> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), Box<dyn Error>> {
        stmt.accept(self)
    }

    pub(crate) fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), Box<dyn Error>> {
        let previous = self.environment.clone();
        self.environment = environment;
        for stmt in statements {
            if let Err(e) = self.execute(stmt) {
                self.environment = previous;
                return Err(e);
            }
        }
        self.environment = previous;
        Ok(())
    }

    fn check_number_operand(operator: &Token, operand: &Value) -> Result<(), Box<dyn Error>> {
        if let Number(_) = operand {
            return Ok(());
        }
        Err(Box::new(RuntimeError::new(
            operator.clone(),
            "Operand must be a number.".to_string(),
        )))
    }

    fn check_number_operands(
        operator: &Token,
        left: &Value,
        right: &Value,
    ) -> Result<(), Box<dyn Error>> {
        if let (Number(_), Number(_)) = (left, right) {
            return Ok(());
        }
        Err(Box::new(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers.".to_string(),
        )))
    }
}

// impl crate::expr::Visitor<Result<Value, RuntimeError>> for Interpreter {
impl crate::expr::Visitor<Result<Value, Box<dyn Error>>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
        // ) -> Result<Value, Box<dyn Error>> {
    ) -> Result<Value, Box<dyn Error>> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;

        match operator.token_type {
            TokenType::MINUS => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(left_value - right_value)
            }
            TokenType::PLUS => match (&left_value, &right_value) {
                (Number(_), Number(_)) | (String(_), String(_)) => Ok(left_value + right_value),
                _ => Err(Box::new(RuntimeError::new(
                    operator.clone(),
                    "Operands must be two numbers or two strings.".to_string(),
                ))),
            },
            TokenType::SLASH => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(left_value / right_value)
            }
            TokenType::STAR => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(left_value * right_value)
            }
            TokenType::GREATER => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value > right_value))
            }
            TokenType::GREATER_EQUAL => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value >= right_value))
            }
            TokenType::LESS => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value < right_value))
            }
            TokenType::LESS_EQUAL => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value <= right_value))
            }
            TokenType::BANG_EQUAL => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value != right_value))
            }
            TokenType::EQUAL_EQUAL => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value == right_value))
            }
            _ => unreachable!("Invalid binary operator"),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Value, Box<dyn Error>> {
        self.evaluate(expression)
    }

    fn visit_literal_expr(&mut self, value: &Literal) -> Result<Value, Box<dyn Error>> {
        match value {
            Literal::String(s) => Ok(String(s.clone())),
            Literal::Number(n) => Ok(Number(*n)),
            Literal::Bool(b) => Ok(Boolean(*b)),
            Literal::Nil => Ok(Nil),
        }
    }

    fn visit_unary_expr(
        &mut self,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, Box<dyn Error>> {
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            TokenType::MINUS => {
                Self::check_number_operand(operator, &right_value)?;
                Ok(-right_value)
            }
            TokenType::BANG => Ok(Boolean(!right_value.as_ref())),
            _ => unreachable!(),
        }
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> Result<Value, Box<dyn Error>> {
        // getCallback(1+2)();
        let callee = self.evaluate(callee);

        let arguments = arguments
            .into_iter()
            .map(|f| match self.evaluate(f) {
                Ok(v) => Ok(Box::new(v)),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<Box<Value>>, Box<dyn Error>>>()?;

        let function = match callee {
            Ok(Callable(lox_callable)) => lox_callable,
            _ => {
                return Err(Box::new(RuntimeError::new(
                    paren.clone(),
                    "Can only call functions.".to_string(),
                )))
            }
        };

        if arguments.len() != function.arity() {
            return Err(Box::new(RuntimeError {
                token: paren.clone(),
                message: format!(
                    "Expected {} arguments but got {}.",
                    function.arity(),
                    arguments.len()
                ),
            }));
        }

        Ok(function.call(self, arguments)?)
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Value, Box<dyn Error>> {
        // Very interesting method to alter the return error type
        Ok(self.environment.borrow().get(name)?)
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Value, Box<dyn Error>> {
        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, Box<dyn Error>> {
        let left = self.evaluate(left)?;
        if operator.token_type == TokenType::OR {
            if *left.as_ref() {
                return Ok(left);
            }
        } else if !*left.as_ref() {
            return Ok(left);
        }

        self.evaluate(right)
    }
}

impl crate::stmt::Visitor<Result<(), Box<dyn Error>>> for Interpreter {
    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<(), Box<dyn Error>> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), Box<dyn Error>> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> Result<(), Box<dyn Error>> {
        let value = self.evaluate(value)?;
        Err(Box::new(Return(value)))
    }

    fn visit_var_stmt(
        &mut self,
        name: &Token,
        initializer: Option<&Expr>,
    ) -> Result<(), Box<dyn Error>> {
        let mut value = Nil;
        if let Some(v) = initializer {
            value = self.evaluate(v)?;
        }
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), Box<dyn Error>> {
        self.execute_block(
            statements,
            Environment::new_enclosing(self.environment.clone()),
        )
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<(), Box<dyn Error>> {
        if *self.evaluate(condition)?.as_ref() {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), Box<dyn Error>> {
        while *self.evaluate(condition)?.as_ref() {
            self.execute(body)?;
        }

        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: Box<LoxFunctionNode>) -> Result<(), Box<dyn Error>> {
        let function = LoxFunction::new(stmt.clone(), self.environment.clone());
        self.environment.borrow_mut().define(
            stmt.name.lexeme.clone(),
            Callable(Box::new(LoxCallable::Function(function))),
        );
        Ok(())
    }
}
