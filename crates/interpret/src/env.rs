use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::values::Value;
use senbonzakura_ast as ast;

/// A function value: its declaration, and the scope it was declared in, which its body sees.
#[derive(Debug)]
pub(crate) struct Function {
    pub declaration: Rc<ast::Function>,
    pub closure: Env,
}

/// The variables and functions of one scope, by name, and the scope around it. Cloning shares the
/// scope, so a function and the block that declared it see the same variables.
///
/// A function defined in a scope keeps that scope alive, and the scope keeps the function, so the
/// two are only freed with the interpreter's whole run.
#[derive(Debug, Clone)]
pub(crate) struct Env(Rc<Scope>);

#[derive(Debug)]
struct Scope {
    parent: Option<Env>,
    values: RefCell<HashMap<String, Value>>,
}

impl Env {
    pub fn new(parent: Option<&Env>) -> Env {
        Env(Rc::new(Scope { parent: parent.cloned(), values: RefCell::new(HashMap::new()) }))
    }

    pub fn define(&self, name: &str, value: Value) {
        self.0.values.borrow_mut().insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Value {
        self.scope_of(name).0.values.borrow()[name].clone()
    }

    pub fn assign(&self, name: &str, value: Value) {
        self.scope_of(name).0.values.borrow_mut().insert(name.to_string(), value);
    }

    fn scope_of(&self, name: &str) -> &Env {
        let mut env = self;
        loop {
            if env.0.values.borrow().contains_key(name) {
                return env;
            }
            env = env
                .0
                .parent
                .as_ref()
                .unwrap_or_else(|| unreachable!("the checker rejects undefined '{name}'"));
        }
    }
}
