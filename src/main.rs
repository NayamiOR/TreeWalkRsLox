// mod ast_printer;
mod environment;
mod expr;
mod interpreter;
mod lox_callable;
mod lox_function;
mod native_functions;
mod parser;
mod runtime_error;
mod scanner;
mod stmt;
mod token;
mod token_type;
mod value;

use crate::interpreter::Interpreter;
use once_cell::unsync::Lazy;
use scanner::Scanner;
use std::io::Write;
use token::Token;

struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    interpreter: Interpreter,
}

static mut LOX: Lazy<Lox> = Lazy::new(|| Lox::new());

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => Lox::run_prompt().unwrap(),
        2 => Lox::run_file(args[1].clone()).unwrap(),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

impl Lox {
    pub(crate) fn new() -> Self {
        Lox {
            had_error: false,
            had_runtime_error: false,
            interpreter: Interpreter::new(),
        }
    }
    pub(crate) fn run_file(path: String) -> Result<(), std::io::Error> {
        let source = std::fs::read_to_string(path)?;
        Self::run(source);
        if unsafe { LOX.had_error } {
            std::process::exit(65);
        }
        if unsafe { LOX.had_runtime_error } {
            std::process::exit(70);
        }
        Ok(())
    }

    pub(crate) fn run_prompt() -> Result<(), std::io::Error> {
        loop {
            print!("> ");
            std::io::stdout().flush()?;
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            Self::run(line);
            unsafe {
                LOX.had_error = false;
            }
        }
    }

    pub(crate) fn run(source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse();
        if unsafe { LOX.had_error } {
            return;
        }

        unsafe { LOX.interpreter.interpret(statements) }
    }

    pub(crate) fn error_at_line(line: i32, message: String) {
        Self::report(line, "".to_string(), message);
    }

    pub(crate) fn report(line: i32, location: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        unsafe {
            LOX.had_error = true;
        }
    }

    pub(crate) fn error_at_token(token: Token, message: String) {
        if token.token_type == token_type::TokenType::EOF {
            Self::report(token.line, " at end".to_string(), message);
        } else {
            Self::report(token.line, format!(" at '{}'", token.lexeme), message)
        }
    }

    pub(crate) fn runtime_error(error: runtime_error::RuntimeError) {
        eprintln!("{}\n[line {}]", error.message, error.token.line);
        unsafe {
            LOX.had_runtime_error = true;
        }
    }
}
