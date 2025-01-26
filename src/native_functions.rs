use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::lox_callable::{LoxCallable, LoxNativeFunction};
use crate::value::Value;
use crate::value::Value::Callable;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn global_env() -> Rc<RefCell<Environment>> {
    let globals = globals();
    Rc::new(RefCell::new(Environment {
        values: globals,
        enclosing: None,
    }))
}

pub fn globals() -> HashMap<String, Value> {
    let mut globals = HashMap::new();
    let clock_function: Value =
        Callable(Box::new(LoxCallable::NativeFunction(LoxNativeFunction {
            name: String::from("clock"),
            params: vec![],
            function: |interpreter: &mut Interpreter| {
                let now = std::time::SystemTime::now();
                let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
                Value::Number(duration.as_secs_f64())
            },
        })));

    globals.insert(String::from("clock"), clock_function);
    globals
}
