//! Runs a checked program by walking its tree.

mod env;
mod values;

#[cfg(test)]
mod tests;

use std::fmt;
use std::io::Write;
use std::rc::Rc;

use env::{Env, Function};
use senbonzakura_ast::{
  AssignOp, BinaryOp, Block, Expr, ExprKind, Stmt, TypeKind, UnaryOp, UpdateOp, VarStmt,
};
use senbonzakura_check::{Analysis, Ty};
use values::Value;

/// How deeply calls may nest before the run fails with a stack overflow.
const MAX_CALL_DEPTH: usize = 10_000;

/// A problem found while running a program that checked cleanly, like a division by zero. Covers
/// the source offsets `start` up to `end` of the code that failed, or neither when no place can
/// be named, as for a stack overflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeError {
  pub message: String,
  pub location: Option<(usize, usize)>,
}

impl RuntimeError {
  fn at(start: usize, end: usize, message: impl Into<String>) -> RuntimeError {
    RuntimeError { message: message.into(), location: Some((start, end)) }
  }

  fn stack_overflow() -> RuntimeError {
    RuntimeError {
      message: "stack overflow: the calls nest too deeply".to_string(),
      location: None,
    }
  }
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.message)
  }
}

impl std::error::Error for RuntimeError {}

/// Runs a file by walking its tree. The file must have checked cleanly: the interpreter relies on
/// the checker for every name to be defined and every value to have the right type, and takes the
/// type of each operation from its [`Analysis`].
///
/// A file runs its top-level statements in order, like a script. As in the checker, the functions
/// of a block are defined before its statements run, so they can be called before their
/// declaration, and a nested function sees the variables around it as they are when it runs.
///
/// `print` writes its value and a line break to the output. Problems only a run can find, like a
/// division by zero, end it with a [`RuntimeError`].
///
/// Deep recursion needs a large stack; see [`with_large_stack`].
pub struct Interpreter<'a> {
  analysis: &'a Analysis,
  out: &'a mut dyn Write,
  /// The values a `return` gave, read by the call that is returning.
  returned: Vec<Value>,
  depth: usize,
}

/// Runs `f` on a thread with a stack large enough for the interpreter's deepest allowed
/// recursion, and returns what it returns. Rust cannot catch a stack overflow, so the
/// interpreter limits how deeply calls nest, and this gives that limit room to be reached.
pub fn with_large_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
  const STACK_SIZE: usize = 1 << 30;
  std::thread::Builder::new()
    .stack_size(STACK_SIZE)
    .spawn(f)
    .expect("cannot start the interpreter thread")
    .join()
    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
}

/// How a statement finished, so loops and calls know whether to go on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flow {
  Normal,
  Break,
  Continue,
  Return,
}

type Run<T> = Result<T, RuntimeError>;

impl<'a> Interpreter<'a> {
  pub fn new(analysis: &'a Analysis, out: &'a mut dyn Write) -> Interpreter<'a> {
    Interpreter { analysis, out, returned: Vec::new(), depth: 0 }
  }

  /// Runs a file's statements; fails if the program does, for example by dividing by zero.
  pub fn run(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
    let env = Env::new(Some(&builtins()));
    self.statements(statements, &env)?;
    Ok(())
  }

  // Statements

  /// Runs statements in `env`, after defining their functions.
  fn statements(&mut self, statements: &[Stmt], env: &Env) -> Run<Flow> {
    for stmt in statements {
      if let Stmt::Function(declaration) = stmt {
        let function = Function { declaration: declaration.clone(), closure: env.clone() };
        env.define(&declaration.name, Value::Function(Rc::new(function)));
      }
    }
    for stmt in statements {
      let flow = self.execute(stmt, env)?;
      if flow != Flow::Normal {
        return Ok(flow);
      }
    }
    Ok(Flow::Normal)
  }

  fn execute(&mut self, stmt: &Stmt, env: &Env) -> Run<Flow> {
    match stmt {
      Stmt::Expression(s) => {
        self.evaluate(&s.expr, env)?;
        Ok(Flow::Normal)
      }
      Stmt::Var(s) => {
        self.declare(s, env)?;
        Ok(Flow::Normal)
      }
      Stmt::If(s) => {
        if self.is_true(&s.condition, env)? {
          return self.block(&s.then, env);
        }
        match &s.otherwise {
          Some(otherwise) => self.execute(otherwise, env),
          None => Ok(Flow::Normal),
        }
      }
      Stmt::Loop(s) => {
        while match &s.condition {
          Some(condition) => self.is_true(condition, env)?,
          None => true,
        } {
          match self.block(&s.body, env)? {
            Flow::Break => break,
            Flow::Return => return Ok(Flow::Return),
            Flow::Normal | Flow::Continue => {}
          }
        }
        Ok(Flow::Normal)
      }
      Stmt::LoopIn(s) => {
        let Value::Range(lower, upper) = self.evaluate(&s.iterable, env)? else {
          unreachable!("the checker lets only ranges be looped over")
        };
        let Ty::Range(element) = self.type_of(&s.iterable) else {
          unreachable!("a range has a range type")
        };
        let Ty::Primitive(kind) = **element else {
          unreachable!("the checker settles the range on an integer type")
        };
        let element = Ty::Primitive(kind);
        let mut i = lower;
        while values::compare(BinaryOp::Lt, &element, &Value::Int(i), &Value::Int(upper)) {
          let loop_env = Env::new(Some(env));
          loop_env.define(&s.name, Value::Int(i));
          match self.block(&s.body, &loop_env)? {
            Flow::Break => break,
            Flow::Return => return Ok(Flow::Return),
            Flow::Normal | Flow::Continue => {}
          }
          i = values::wrap(i.wrapping_add(1), kind);
        }
        Ok(Flow::Normal)
      }
      Stmt::LoopStep(s) => {
        let loop_env = Env::new(Some(env));
        self.declare(&s.init, &loop_env)?;
        while self.is_true(&s.condition, &loop_env)? {
          match self.block(&s.body, &loop_env)? {
            Flow::Break => break,
            Flow::Return => return Ok(Flow::Return),
            Flow::Normal | Flow::Continue => {}
          }
          self.evaluate(&s.step, &loop_env)?;
        }
        Ok(Flow::Normal)
      }
      Stmt::Continue(s) => self.jump(s.condition.as_ref(), env, Flow::Continue),
      Stmt::Break(s) => self.jump(s.condition.as_ref(), env, Flow::Break),
      Stmt::Return(s) => {
        if let Some(condition) = &s.condition
          && !self.is_true(condition, env)?
        {
          return Ok(Flow::Normal);
        }
        let mut returned = Vec::with_capacity(s.values.len());
        for value in &s.values {
          returned.push(self.evaluate(value, env)?);
        }
        self.returned = returned;
        Ok(Flow::Return)
      }
      Stmt::Function(_) => Ok(Flow::Normal), // defined with the block around it
      Stmt::Block(s) => self.block(s, env),
    }
  }

  fn jump(&mut self, condition: Option<&Expr>, env: &Env, flow: Flow) -> Run<Flow> {
    match condition {
      Some(condition) if !self.is_true(condition, env)? => Ok(Flow::Normal),
      _ => Ok(flow),
    }
  }

  fn block(&mut self, block: &Block, env: &Env) -> Run<Flow> {
    self.statements(&block.statements, &Env::new(Some(env)))
  }

  fn declare(&mut self, var: &VarStmt, env: &Env) -> Run<()> {
    for declarator in &var.declarators {
      let value = match (&declarator.initializer, &declarator.ty) {
        (Some(initializer), _) => self.evaluate(initializer, env)?,
        (None, Some(ty)) => match ty.kind {
          TypeKind::Primitive(kind) => values::zero(kind),
          TypeKind::Error => unreachable!("the checker rejects a broken type"),
        },
        (None, None) => unreachable!("the checker rejects a var with no type or value"),
      };
      env.define(&declarator.name, value);
    }
    Ok(())
  }

  fn is_true(&mut self, condition: &Expr, env: &Env) -> Run<bool> {
    Ok(self.evaluate(condition, env)?.bool())
  }

  // Expressions

  fn evaluate(&mut self, expr: &Expr, env: &Env) -> Run<Value> {
    Ok(match &expr.kind {
      ExprKind::IntLiteral(value) => values::integer_literal(*value, self.type_of(expr)),
      ExprKind::FloatLiteral(value) => values::float_literal(*value, self.type_of(expr)),
      ExprKind::StringLiteral(value) => Value::Str(value.as_str().into()),
      ExprKind::CharLiteral(value) => Value::Int(i64::from(u32::from(*value))),
      ExprKind::BoolLiteral(value) => Value::Bool(*value),
      ExprKind::Identifier(name) => env.get(name),
      ExprKind::Call { callee, arguments } => self.call(callee, arguments, env)?,
      ExprKind::Unary { op, operand } => {
        let operand = self.evaluate(operand, env)?;
        match op {
          UnaryOp::Neg => values::negate(self.type_of(expr), &operand),
          UnaryOp::Not => Value::Bool(!operand.bool()),
        }
      }
      ExprKind::Binary { op, left, right } => self.binary(expr, *op, left, right, env)?,
      ExprKind::Range { lower, upper } => {
        let lower = self.evaluate(lower, env)?.int();
        let upper = self.evaluate(upper, env)?.int();
        Value::Range(lower, upper)
      }
      ExprKind::Assign { op, target, value } => {
        let name = identifier(target);
        let mut value = self.evaluate(value, env)?;
        if *op != AssignOp::Assign {
          let ty = self.type_of(target);
          value = values::arithmetic(arithmetic(*op), ty, &env.get(name), &value, expr)?;
        }
        env.assign(name, value.clone());
        value
      }
      ExprKind::Update { op, target } => {
        let name = identifier(target);
        let ty = self.type_of(target);
        let old = env.get(name);
        let op = match op {
          UpdateOp::Increment => BinaryOp::Add,
          UpdateOp::Decrement => BinaryOp::Sub,
        };
        env.assign(name, values::arithmetic(op, ty, &old, &values::one(ty), expr)?);
        old
      }
      ExprKind::Cast { value, .. } => {
        let converted = self.evaluate(value, env)?;
        values::cast(&converted, self.type_of(value), self.type_of(expr), expr)?
      }
      ExprKind::Group(inner) => self.evaluate(inner, env)?,
      ExprKind::Index { .. } | ExprKind::Field { .. } | ExprKind::Error => {
        unreachable!("the checker rejects {expr:?}")
      }
    })
  }

  fn binary(
    &mut self,
    expr: &Expr,
    op: BinaryOp,
    left: &Expr,
    right: &Expr,
    env: &Env,
  ) -> Run<Value> {
    match op {
      BinaryOp::And => {
        return Ok(Value::Bool(self.is_true(left, env)? && self.is_true(right, env)?));
      }
      BinaryOp::Or => {
        return Ok(Value::Bool(self.is_true(left, env)? || self.is_true(right, env)?));
      }
      _ => {}
    }
    let left_value = self.evaluate(left, env)?;
    let right_value = self.evaluate(right, env)?;
    // The checker settles both operands into one type, so the left one's is the operation's.
    let ty = self.type_of(left);
    match op {
      BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => {
        values::arithmetic(op, ty, &left_value, &right_value, expr)
      }
      _ => Ok(Value::Bool(values::compare(op, ty, &left_value, &right_value))),
    }
  }

  /// Calls a function by name. It gives [`Value::Nothing`] for no value, the value for one, and
  /// [`Value::Multiple`] for several, which only a statement can receive.
  fn call(
    &mut self,
    callee: &Expr,
    arguments: &[senbonzakura_ast::Argument],
    env: &Env,
  ) -> Run<Value> {
    let callee = env.get(identifier(callee));
    let mut values = Vec::with_capacity(arguments.len());
    for argument in arguments {
      values.push(self.evaluate(&argument.value, env)?);
    }

    let function = match callee {
      Value::Print => {
        let shown = values::format(self.type_of(&arguments[0].value), &values[0]);
        // Like print in most languages, a closed or full output does not stop the program.
        let _ = writeln!(self.out, "{shown}");
        return Ok(Value::Nothing);
      }
      Value::Function(function) => function,
      other => unreachable!("the checker lets only functions be called, not {other:?}"),
    };

    if self.depth == MAX_CALL_DEPTH {
      return Err(RuntimeError::stack_overflow());
    }
    let parameters = &function.declaration.parameters;
    let call_env = Env::new(Some(&function.closure));
    for (i, (argument, value)) in arguments.iter().zip(values).enumerate() {
      let name = argument.name.as_deref().unwrap_or(&parameters[i].name);
      call_env.define(name, value);
    }

    self.depth += 1;
    let flow = self.statements(&function.declaration.body.statements, &call_env);
    self.depth -= 1;
    if flow? != Flow::Return {
      return Ok(Value::Nothing);
    }
    let mut returned = std::mem::take(&mut self.returned);
    Ok(match returned.len() {
      0 => Value::Nothing,
      1 => returned.pop().expect("one value"),
      _ => Value::Multiple(returned),
    })
  }

  fn type_of(&self, expr: &Expr) -> &'a Ty {
    self.analysis.type_of(expr)
  }
}

/// Runs a program one entry at a time, in step with a [`check::Session`](senbonzakura_check::Session):
/// each entry runs in an environment inside the earlier entries', and one that fails is
/// forgotten.
pub struct Session {
  env: Env,
}

impl Default for Session {
  fn default() -> Session {
    Session::new()
  }
}

impl Session {
  pub fn new() -> Session {
    Session { env: builtins() }
  }

  /// Runs an entry that checked cleanly. With `show_last`, the last statement must be an
  /// expression statement, and this returns how `print` would show its value, or `None` when
  /// it has none; otherwise it returns `None`. If the entry fails, its variables are forgotten.
  pub fn run(
    &mut self,
    statements: &[Stmt],
    analysis: &Analysis,
    show_last: bool,
    out: &mut dyn Write,
  ) -> Result<Option<String>, RuntimeError> {
    let mut interpreter = Interpreter::new(analysis, out);
    let entry = Env::new(Some(&self.env));
    let mut shown = None;
    match statements.split_last() {
      Some((Stmt::Expression(last), rest)) if show_last => {
        interpreter.statements(rest, &entry)?;
        let value = interpreter.evaluate(&last.expr, &entry)?;
        let ty = analysis.type_of(&last.expr);
        if !matches!(value, Value::Nothing) && !matches!(ty, Ty::Multiple(_)) {
          shown = Some(values::format(ty, &value));
        }
      }
      _ => {
        interpreter.statements(statements, &entry)?;
      }
    }
    self.env = entry;
    Ok(shown)
  }
}

fn builtins() -> Env {
  let builtins = Env::new(None);
  builtins.define("print", Value::Print);
  builtins
}

/// The name an assignment or call goes to; the checker lets nothing else through.
fn identifier(expr: &Expr) -> &str {
  match &expr.kind {
    ExprKind::Identifier(name) => name,
    _ => unreachable!("the checker lets only names be assigned to and called: {expr:?}"),
  }
}

fn arithmetic(op: AssignOp) -> BinaryOp {
  match op {
    AssignOp::Add => BinaryOp::Add,
    AssignOp::Sub => BinaryOp::Sub,
    AssignOp::Mul => BinaryOp::Mul,
    AssignOp::Div => BinaryOp::Div,
    AssignOp::Rem => BinaryOp::Rem,
    AssignOp::Assign => unreachable!("not compound: {op:?}"),
  }
}
