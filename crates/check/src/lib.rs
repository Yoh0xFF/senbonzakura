//! Checks a parsed file for what the parser cannot see on its own:
//!
//! - Every name refers to a variable, parameter or function in scope, and no scope declares the
//!   same name twice.
//! - `break` and `continue` are inside a loop, and `return` inside a function with as many values
//!   as the function declares. A function with return types returns on every path.
//! - A call to a function by its name passes each parameter exactly once.
//! - Every value has the type its place asks for: initializers, arguments, returned values,
//!   conditions, operands and assignments.
//!
//! **Scopes.** Each block opens a scope, a function's parameters share one with its body, and a
//! loop's variables get one for the whole loop. A variable is visible from the end of its
//! declarator on, so `var x = x;` does not see itself. A function is visible throughout the block
//! that declares it, so functions can call each other in any order. Inner scopes see outer names
//! and may shadow them; a function body sees the names around it, but a loop around a function
//! does not hold its `break`. Built-in functions, so far only `print`, live in a scope above the
//! file's, so a file may declare its own.
//!
//! **Types.** No value converts to another type by itself; `x as long` converts explicitly.
//! Number literals are the exception: they take the numeric type their place asks for, if they
//! fit it, and are `int` or `double` when nothing asks. A variable without a type annotation takes
//! its initializer's type. `+` also joins strings.
//!
//! Like the lexer and parser, it never panics on bad input; problems come back as diagnostics.
//! Run it on a tree that parsed cleanly: it skips error nodes and missing names, but the names a
//! broken statement failed to declare would show up as undefined.

mod return_flow;
mod ty;

#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub use ty::Ty;

use senbonzakura_ast::{
  AssignOp, BinaryOp, Block, Declarator, Expr, ExprId, ExprKind, Function, LoopInStmt, ReturnStmt,
  Stmt, Type, TypeKind, UnaryOp, VarStmt,
};
use senbonzakura_lexer::Diagnostic;

/// Checks a file's statements, as the parser's `parse_file` gives them.
pub fn check(statements: &[Stmt]) -> Vec<Diagnostic> {
  analyze(statements).diagnostics
}

/// Like [`check`], and also keeps the type it worked out for each expression.
pub fn analyze(statements: &[Stmt]) -> Analysis {
  let mut checker = Checker::new();
  checker.statements(statements);
  Analysis { diagnostics: checker.diagnostics, types: checker.types }
}

/// Checks a program one entry at a time, as an interactive prompt reads it. Each entry gets a
/// scope inside the earlier entries' scopes, so it sees their names and may declare one again,
/// which shadows the old one. An entry with problems is forgotten, as if it was never entered.
pub struct Session {
  checker: Checker,
}

impl Default for Session {
  fn default() -> Session {
    Session::new()
  }
}

impl Session {
  pub fn new() -> Session {
    Session { checker: Checker::new() }
  }

  /// Checks one entry; its names stay visible to later entries if it has no problems. The
  /// analysis also covers the earlier entries, whose functions the entry may call.
  pub fn check(&mut self, statements: &[Stmt]) -> Analysis {
    let checker = &mut self.checker;
    checker.diagnostics.clear();
    checker.enter_scope();
    checker.statements(statements);
    let analysis =
      Analysis { diagnostics: checker.diagnostics.clone(), types: checker.types.clone() };
    if !analysis.diagnostics.is_empty() {
      checker.exit_scope();
    }
    analysis
  }

  /// Forgets the last entry that checked cleanly, for when running it failed, so the names it
  /// never got to define are not visible later.
  pub fn forget_last(&mut self) {
    self.checker.exit_scope();
  }
}

/// What checking a file found: its problems, and the type of every expression it checked.
#[derive(Debug, Clone)]
pub struct Analysis {
  diagnostics: Vec<Diagnostic>,
  types: HashMap<ExprId, Ty>,
}

impl Analysis {
  pub fn diagnostics(&self) -> &[Diagnostic] {
    &self.diagnostics
  }

  /// Returns the type of an expression in the checked tree. A number literal has the type it
  /// settled into, like `byte` in `var b: byte = 1`; one that nothing settled, like those in
  /// `1 < 2`, keeps its constant type.
  ///
  /// # Panics
  ///
  /// If the expression was not checked, like a callee name.
  pub fn type_of(&self, expr: &Expr) -> &Ty {
    self.types.get(&expr.id).unwrap_or_else(|| panic!("no type recorded for {expr:?}"))
  }
}

#[derive(Debug, Clone)]
enum Symbol {
  Variable(Ty),
  Function(Rc<FunctionSymbol>),
}

/// A function's signature. Names are empty where the parser reported a missing one.
#[derive(Debug)]
struct FunctionSymbol {
  name: String,
  parameter_names: Vec<String>,
  parameter_types: Vec<Ty>,
  return_types: Vec<Ty>,
}

struct Checker {
  diagnostics: Vec<Diagnostic>,
  types: HashMap<ExprId, Ty>,
  /// Innermost last. The first holds the built-ins.
  scopes: Vec<HashMap<String, Symbol>>,
  /// The function whose body is being checked, or `None` at the top level.
  function: Option<Rc<FunctionSymbol>>,
  /// How many loops around the current statement are inside `function`.
  loop_depth: usize,
}

impl Checker {
  fn new() -> Checker {
    let print = FunctionSymbol {
      name: "print".to_string(),
      parameter_names: vec!["value".to_string()],
      parameter_types: vec![Ty::Any],
      return_types: vec![],
    };
    let builtins = HashMap::from([(print.name.clone(), Symbol::Function(Rc::new(print)))]);
    Checker {
      diagnostics: Vec::new(),
      types: HashMap::new(),
      scopes: vec![builtins, HashMap::new()],
      function: None,
      loop_depth: 0,
    }
  }

  // Statements

  /// Checks statements in the current scope, declaring their functions first.
  fn statements(&mut self, statements: &[Stmt]) {
    for stmt in statements {
      if let Stmt::Function(f) = stmt
        && !f.name.is_empty()
      {
        let symbol = Symbol::Function(Rc::new(signature(f)));
        self.declare(&f.name, symbol, f.start, f.end);
      }
    }
    for stmt in statements {
      self.statement(stmt);
    }
  }

  fn statement(&mut self, stmt: &Stmt) {
    match stmt {
      Stmt::Expression(s) => {
        self.expression(&s.expr);
      }
      Stmt::Var(s) => self.declarators(s),
      Stmt::If(s) => {
        self.condition(&s.condition);
        self.block(&s.then);
        if let Some(otherwise) = &s.otherwise {
          self.statement(otherwise);
        }
      }
      Stmt::Loop(s) => {
        if let Some(condition) = &s.condition {
          self.condition(condition);
        }
        self.loop_body(&s.body);
      }
      Stmt::LoopIn(s) => {
        let ty = self.loop_variable(s);
        self.enter_scope();
        if !s.name.is_empty() {
          self.declare(&s.name, Symbol::Variable(ty), s.start, s.end);
        }
        self.loop_body(&s.body);
        self.exit_scope();
      }
      Stmt::LoopStep(s) => {
        self.enter_scope();
        self.declarators(&s.init);
        self.condition(&s.condition);
        self.loop_body(&s.body);
        self.expression(&s.step);
        self.exit_scope();
      }
      Stmt::Continue(s) => self.jump("continue", s.condition.as_ref(), s.start, s.end),
      Stmt::Break(s) => self.jump("break", s.condition.as_ref(), s.start, s.end),
      Stmt::Return(s) => self.return_statement(s),
      Stmt::Function(f) => self.function(f),
      Stmt::Block(s) => self.block(s),
    }
  }

  fn block(&mut self, block: &Block) {
    self.enter_scope();
    self.statements(&block.statements);
    self.exit_scope();
  }

  fn loop_body(&mut self, body: &Block) {
    self.loop_depth += 1;
    self.block(body);
    self.loop_depth -= 1;
  }

  fn declarators(&mut self, var: &VarStmt) {
    for declarator in &var.declarators {
      let ty = self.declarator_type(declarator);
      if !declarator.name.is_empty() {
        self.declare(&declarator.name, Symbol::Variable(ty), declarator.start, declarator.end);
      }
    }
  }

  /// The declared type if there is one, which the initializer must fit, else the initializer's.
  fn declarator_type(&mut self, declarator: &Declarator) -> Ty {
    let declared = declarator.ty.as_ref().map(resolve);
    let Some(initializer) = &declarator.initializer else {
      if declared.is_none() && !declarator.name.is_empty() {
        self.report(
          declarator.start,
          declarator.end,
          format!("'{}' needs a type or an initializer", declarator.name),
        );
      }
      return declared.unwrap_or(Ty::Error);
    };

    let value = self.expression(initializer);
    if let Some(declared) = declared {
      self.expect(initializer, &value, &declared);
      return declared;
    }
    if matches!(value, Ty::Nothing | Ty::Multiple(_)) {
      self.report_at(initializer, format!("expected a value, found {}", value.describe()));
      return Ty::Error;
    }
    self.settle(initializer, value)
  }

  /// The type of a `loop var x in` variable: the range's, or the declared one if it fits.
  fn loop_variable(&mut self, s: &LoopInStmt) -> Ty {
    let iterable = self.expression(&s.iterable);
    let element = match iterable {
      Ty::Range(element) => *element,
      Ty::Error => Ty::Error,
      other => {
        self.report_at(&s.iterable, format!("cannot loop over {}", other.describe()));
        Ty::Error
      }
    };

    let Some(written) = &s.ty else {
      if element == Ty::IntegerConstant {
        self.retype_range(&s.iterable, &Ty::INT);
        return Ty::INT;
      }
      return element;
    };
    let declared = resolve(written);
    if element == Ty::IntegerConstant {
      let settled = if declared.is_integer() { declared.clone() } else { Ty::INT };
      self.retype_range(&s.iterable, &settled);
    }
    let fits = element == Ty::Error
      || declared == Ty::Error
      || declared == element
      || (element == Ty::IntegerConstant && declared.is_integer());
    if !fits {
      let holds = if element == Ty::IntegerConstant { Ty::INT } else { element };
      self.report(
        written.start,
        written.end,
        format!(
          "the loop variable is {}, but the range holds {}",
          declared.describe(),
          holds.describe()
        ),
      );
    }
    declared
  }

  fn jump(&mut self, keyword: &str, condition: Option<&Expr>, start: usize, end: usize) {
    if self.loop_depth == 0 {
      self.report(start, end, format!("'{keyword}' outside a loop"));
    }
    if let Some(condition) = condition {
      self.condition(condition);
    }
  }

  fn return_statement(&mut self, s: &ReturnStmt) {
    let types: Vec<Ty> = s.values.iter().map(|value| self.expression(value)).collect();
    if let Some(condition) = &s.condition {
      self.condition(condition);
    }

    let Some(function) = self.function.clone() else {
      self.report(s.start, s.end, "'return' outside a function");
      return;
    };
    let expected = function.return_types.len();
    if types.len() != expected {
      let wanted =
        if expected == 0 { "no return value".to_string() } else { count(expected, "return value") };
      let found = if types.is_empty() { "none".to_string() } else { types.len().to_string() };
      self.report(s.start, s.end, format!("expected {wanted}, found {found}"));
      return;
    }
    for ((value, found), expected) in s.values.iter().zip(&types).zip(&function.return_types) {
      self.expect(value, found, expected);
    }
  }

  /// The function's name was declared with the block around it; this checks its body.
  fn function(&mut self, f: &Function) {
    let symbol = Rc::new(signature(f));
    let outer_function = self.function.replace(symbol.clone());
    let outer_loop_depth = std::mem::replace(&mut self.loop_depth, 0);

    self.enter_scope();
    for (parameter, ty) in f.parameters.iter().zip(&symbol.parameter_types) {
      if !parameter.name.is_empty() {
        let variable = Symbol::Variable(ty.clone());
        self.declare(&parameter.name, variable, parameter.start, parameter.end);
      }
    }
    self.statements(&f.body.statements);
    self.exit_scope();

    if let Some(last) = f.return_types.last()
      && !return_flow::always_returns(&f.body.statements)
    {
      let name = if f.name.is_empty() { "function".to_string() } else { format!("'{}'", f.name) };
      self.report(f.start, last.end, format!("{name} does not return on every path"));
    }

    self.function = outer_function;
    self.loop_depth = outer_loop_depth;
  }

  fn condition(&mut self, condition: &Expr) {
    let found = self.expression(condition);
    self.expect(condition, &found, &Ty::BOOL);
  }

  // Expressions

  /// Checks an expression and returns its type, [`Ty::Error`] once reported, and records the
  /// type.
  fn expression(&mut self, expr: &Expr) -> Ty {
    let ty = self.compute(expr);
    self.types.insert(expr.id, ty.clone());
    ty
  }

  fn compute(&mut self, expr: &Expr) -> Ty {
    match &expr.kind {
      ExprKind::IntLiteral(_) => Ty::IntegerConstant,
      ExprKind::FloatLiteral(_) => Ty::FloatConstant,
      ExprKind::StringLiteral(_) => Ty::STRING,
      ExprKind::CharLiteral(_) => Ty::CHAR,
      ExprKind::BoolLiteral(_) => Ty::BOOL,
      ExprKind::Identifier(name) => self.identifier(expr, name),
      ExprKind::Call { callee, arguments } => self.call(expr, callee, arguments),
      ExprKind::Index { target, index } => {
        let target_ty = self.expression(target);
        self.expression(index);
        if target_ty != Ty::Error {
          self.report_at(target, format!("cannot index {}", target_ty.describe()));
        }
        Ty::Error
      }
      ExprKind::Field { target, name } => {
        let target_ty = self.expression(target);
        if target_ty != Ty::Error && !name.is_empty() {
          self.report_at(expr, format!("{} has no field '{name}'", target_ty.describe()));
        }
        Ty::Error
      }
      ExprKind::Unary { op, operand } => self.unary(expr, *op, operand),
      ExprKind::Binary { op, left, right } => self.binary(expr, *op, left, right),
      ExprKind::Range { lower, upper } => self.range(expr, lower, upper),
      ExprKind::Assign { op, target, value } => self.assign(expr, *op, target, value),
      ExprKind::Update { op, target } => {
        let target_ty = self.target(target);
        if target_ty == Ty::Error {
          return Ty::Error;
        }
        if !target_ty.is_integer() && !target_ty.is_float() {
          let message = format!("cannot apply '{}' to {}", op.spelling(), target_ty.describe());
          self.report_at(expr, message);
          return Ty::Error;
        }
        target_ty
      }
      ExprKind::Cast { value, ty } => self.cast(expr, value, ty),
      ExprKind::Group(inner) => self.expression(inner),
      ExprKind::Error => Ty::Error,
    }
  }

  fn identifier(&mut self, expr: &Expr, name: &str) -> Ty {
    match self.lookup(name) {
      None => self.not_defined(expr, name),
      Some(Symbol::Variable(ty)) => ty,
      Some(Symbol::Function(_)) => {
        self.report_at(expr, format!("function '{name}' cannot be used as a value"));
        Ty::Error
      }
    }
  }

  /// The type of a name being assigned to; a function cannot be.
  fn target(&mut self, target: &Expr) -> Ty {
    let ExprKind::Identifier(name) = &target.kind else {
      return self.expression(target);
    };
    let ty = match self.lookup(name) {
      None => self.not_defined(target, name),
      Some(Symbol::Variable(ty)) => ty,
      Some(Symbol::Function(_)) => {
        self.report_at(target, format!("cannot assign to function '{name}'"));
        Ty::Error
      }
    };
    self.types.insert(target.id, ty.clone());
    ty
  }

  fn call(&mut self, call: &Expr, callee: &Expr, arguments: &[senbonzakura_ast::Argument]) -> Ty {
    let callee_ty = match &callee.kind {
      ExprKind::Identifier(name) => match self.lookup(name) {
        Some(Symbol::Function(f)) => return self.call_function(call, arguments, &f),
        Some(Symbol::Variable(ty)) => ty,
        None => self.not_defined(callee, name),
      },
      _ => self.expression(callee),
    };

    for argument in arguments {
      self.expression(&argument.value);
    }
    if callee_ty != Ty::Error {
      self.report_at(callee, format!("cannot call {}", callee_ty.describe()));
    }
    Ty::Error
  }

  /// Checks a call's arguments against the parameters of the function it names: as many
  /// positional arguments as parameters, or every parameter named once, each of the parameter's
  /// type. A call that mixes the two, or a function with a missing parameter name, was reported
  /// by the parser and is left alone.
  fn call_function(
    &mut self,
    call: &Expr,
    arguments: &[senbonzakura_ast::Argument],
    f: &FunctionSymbol,
  ) -> Ty {
    let types: Vec<Ty> = arguments.iter().map(|a| self.expression(&a.value)).collect();
    let result = match f.return_types.as_slice() {
      [] => Ty::Nothing,
      [single] => single.clone(),
      several => Ty::Multiple(several.to_vec()),
    };

    let named = arguments.first().is_some_and(|a| a.is_named());
    if arguments.iter().any(|a| a.is_named() != named)
      || f.parameter_names.iter().any(String::is_empty)
    {
      return result;
    }

    let function = format!("'{}'", f.name);
    if !named {
      if arguments.len() != f.parameter_names.len() {
        let expects = count(f.parameter_names.len(), "argument");
        let message = format!("{function} expects {expects}, found {}", arguments.len());
        self.report_at(call, message);
        return result;
      }
      for ((argument, found), expected) in arguments.iter().zip(&types).zip(&f.parameter_types) {
        self.expect(&argument.value, found, expected);
      }
      return result;
    }

    let mut given = HashSet::new();
    for (argument, found) in arguments.iter().zip(&types) {
      let name = argument.name.as_deref().unwrap_or_default();
      given.insert(name);
      match f.parameter_names.iter().position(|p| p == name) {
        Some(parameter) => self.expect(&argument.value, found, &f.parameter_types[parameter]),
        None if !name.is_empty() => self.report(
          argument.start,
          argument.end,
          format!("{function} has no parameter named '{name}'"),
        ),
        None => {}
      }
    }

    let missing: Vec<String> = f
      .parameter_names
      .iter()
      .filter(|name| !given.contains(name.as_str()))
      .map(|name| format!("'{name}'"))
      .collect();
    if !missing.is_empty() {
      let noun = if missing.len() == 1 { "missing argument" } else { "missing arguments" };
      self.report_at(call, format!("{noun} {} for {function}", missing.join(", ")));
    }
    result
  }

  fn unary(&mut self, expr: &Expr, op: UnaryOp, operand: &Expr) -> Ty {
    let operand_ty = self.expression(operand);
    if operand_ty == Ty::Error {
      return Ty::Error;
    }
    match op {
      UnaryOp::Neg if operand_ty.is_constant() || operand_ty.is_signed() => operand_ty,
      UnaryOp::Neg => {
        self.report_at(expr, format!("cannot negate {}", operand_ty.describe()));
        Ty::Error
      }
      UnaryOp::Not if operand_ty == Ty::BOOL => Ty::BOOL,
      UnaryOp::Not => {
        self.report_at(expr, format!("cannot apply '!' to {}", operand_ty.describe()));
        Ty::Error
      }
    }
  }

  fn binary(&mut self, expr: &Expr, op: BinaryOp, left: &Expr, right: &Expr) -> Ty {
    let left_ty = self.expression(left);
    let right_ty = self.expression(right);
    let spelling = op.spelling();
    match op {
      BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => {
        let operands = (left, &left_ty, right, &right_ty);
        self.arithmetic(expr, spelling, op == BinaryOp::Add, operands)
      }
      BinaryOp::And | BinaryOp::Or => {
        if left_ty != Ty::Error
          && right_ty != Ty::Error
          && !(left_ty == Ty::BOOL && right_ty == Ty::BOOL)
        {
          self.cannot_apply(expr, spelling, &left_ty, &right_ty);
        }
        Ty::BOOL
      }
      BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
        if left_ty == Ty::Error || right_ty == Ty::Error {
          return Ty::BOOL;
        }
        let ordered = !matches!(op, BinaryOp::Eq | BinaryOp::Ne);
        let comparable = common(&left_ty, &right_ty).filter(|common| {
          if ordered {
            common.is_numeric() || *common == Ty::CHAR || *common == Ty::STRING
          } else {
            matches!(common, Ty::Primitive(_)) || common.is_constant()
          }
        });
        match comparable {
          Some(common) => {
            self.fit(left, &left_ty, &common);
            self.fit(right, &right_ty, &common);
          }
          None => self.cannot_apply(expr, spelling, &left_ty, &right_ty),
        }
        Ty::BOOL
      }
    }
  }

  /// Checks `+ - * / %` and their compound assignments: two numbers of one type, or two strings
  /// for `+`. Returns that type.
  fn arithmetic(
    &mut self,
    node: &Expr,
    op: &str,
    joins_strings: bool,
    (left, left_ty, right, right_ty): (&Expr, &Ty, &Expr, &Ty),
  ) -> Ty {
    if *left_ty == Ty::Error || *right_ty == Ty::Error {
      return Ty::Error;
    }
    let common = common(left_ty, right_ty)
      .filter(|common| common.is_numeric() || (joins_strings && *common == Ty::STRING));
    let Some(common) = common else {
      self.cannot_apply(node, op, left_ty, right_ty);
      return Ty::Error;
    };
    self.fit(left, left_ty, &common);
    self.fit(right, right_ty, &common);
    common
  }

  fn range(&mut self, expr: &Expr, lower: &Expr, upper: &Expr) -> Ty {
    let lower_ty = self.expression(lower);
    let upper_ty = self.expression(upper);
    if lower_ty == Ty::Error || upper_ty == Ty::Error {
      return Ty::Error;
    }
    let Some(common) = common(&lower_ty, &upper_ty) else {
      let message = format!(
        "range bounds have different types, {} and {}",
        lower_ty.describe(),
        upper_ty.describe()
      );
      self.report_at(expr, message);
      return Ty::Error;
    };
    if !(common.is_integer() || common == Ty::IntegerConstant) {
      let message = format!("range bounds must be integers, found {}", common.describe());
      self.report_at(expr, message);
      return Ty::Error;
    }
    self.fit(lower, &lower_ty, &common);
    self.fit(upper, &upper_ty, &common);
    Ty::range(common)
  }

  fn assign(&mut self, expr: &Expr, op: AssignOp, target: &Expr, value: &Expr) -> Ty {
    let target_ty = self.target(target);
    let value_ty = self.expression(value);
    if target_ty == Ty::Error {
      return Ty::Error;
    }
    if op == AssignOp::Assign {
      self.expect(value, &value_ty, &target_ty);
    } else {
      let operands = (target, &target_ty, value, &value_ty);
      self.arithmetic(expr, op.spelling(), op == AssignOp::Add, operands);
    }
    target_ty
  }

  /// Numbers convert to any numeric type, and integers to and from `char`.
  fn cast(&mut self, expr: &Expr, value: &Expr, ty: &Type) -> Ty {
    let from = self.expression(value);
    let to = resolve(ty);
    if from == Ty::Error || to == Ty::Error {
      return to;
    }
    let converts = from == to
      || (from.is_numeric() && (to.is_integer() || to.is_float()))
      || ((from.is_integer() || from == Ty::IntegerConstant) && to == Ty::CHAR)
      || (from == Ty::CHAR && to.is_integer());
    if !converts {
      let message = format!("cannot convert {} to {}", from.describe(), to.describe());
      self.report_at(expr, message);
    }
    to
  }

  // Types

  /// Reports unless a value of type `found` can go where `expected` is asked for: the same
  /// type, or a constant that fits it.
  fn expect(&mut self, expr: &Expr, found: &Ty, expected: &Ty) {
    if *found == Ty::Error || *expected == Ty::Error {
      return;
    }
    if *expected == Ty::Any {
      if matches!(found, Ty::Nothing | Ty::Multiple(_)) {
        self.report_at(expr, format!("expected a value, found {}", found.describe()));
      } else {
        self.settle(expr, found.clone());
      }
      return;
    }
    if found == expected {
      return;
    }
    if found.is_constant() && takes_constant(expected, found) {
      self.fit(expr, found, expected);
      return;
    }
    let message = format!("expected {}, found {}", expected.describe(), found.describe());
    self.report_at(expr, message);
  }

  /// The type a constant takes when nothing asks for one: `int` or `double`.
  fn settle(&mut self, expr: &Expr, ty: Ty) -> Ty {
    match ty {
      Ty::IntegerConstant => {
        self.fit(expr, &ty, &Ty::INT);
        Ty::INT
      }
      Ty::FloatConstant => {
        self.fit(expr, &ty, &Ty::DOUBLE);
        Ty::DOUBLE
      }
      Ty::Range(element) if *element == Ty::IntegerConstant => {
        self.retype_range(expr, &Ty::INT);
        Ty::range(Ty::INT)
      }
      other => other,
    }
  }

  /// Reports an integer literal that a constant of type `from`, going into `to`, does not fit.
  /// Only literals are checked, possibly negated or in parentheses: `200 + 100` is not worked
  /// out.
  fn fit(&mut self, expr: &Expr, from: &Ty, to: &Ty) {
    if !from.is_constant() || from == to {
      return;
    }
    self.retype(expr, to);
    let Ty::Primitive(kind) = *to else { return };
    if *from != Ty::IntegerConstant || !to.is_integer() {
      return;
    }
    if let Some(value) = literal_value(expr)
      && (value < kind.min_value() || value > kind.max_value())
    {
      self.report_at(expr, format!("{value} does not fit in {}", to.describe()));
    }
  }

  /// Gives a settled constant, and the constants it was computed from, the type it settled
  /// into, so [`Analysis::type_of`] says how to compute each of them.
  fn retype(&mut self, expr: &Expr, to: &Ty) {
    if !self.types.get(&expr.id).is_some_and(Ty::is_constant) {
      return;
    }
    self.types.insert(expr.id, to.clone());
    match &expr.kind {
      ExprKind::Group(inner) => self.retype(inner, to),
      ExprKind::Unary { operand, .. } => self.retype(operand, to),
      ExprKind::Binary { left, right, .. } => {
        self.retype(left, to);
        self.retype(right, to);
      }
      _ => {}
    }
  }

  /// Like [`retype`](Self::retype) for a range of integer constants, such as `0..3`.
  fn retype_range(&mut self, expr: &Expr, element: &Ty) {
    match &expr.kind {
      ExprKind::Group(inner) => {
        self.types.insert(expr.id, Ty::range(element.clone()));
        self.retype_range(inner, element);
      }
      ExprKind::Range { lower, upper } => {
        self.types.insert(expr.id, Ty::range(element.clone()));
        self.fit(lower, &Ty::IntegerConstant, element);
        self.fit(upper, &Ty::IntegerConstant, element);
      }
      _ => {}
    }
  }

  // Scopes and reporting

  fn lookup(&self, name: &str) -> Option<Symbol> {
    self.scopes.iter().rev().find_map(|scope| scope.get(name)).cloned()
  }

  fn declare(&mut self, name: &str, symbol: Symbol, start: usize, end: usize) {
    let scope = self.scopes.last_mut().expect("a scope");
    if scope.contains_key(name) {
      self.report(start, end, format!("'{name}' is already declared in this scope"));
    } else {
      scope.insert(name.to_string(), symbol);
    }
  }

  fn not_defined(&mut self, expr: &Expr, name: &str) -> Ty {
    self.report_at(expr, format!("'{name}' is not defined"));
    Ty::Error
  }

  fn enter_scope(&mut self) {
    self.scopes.push(HashMap::new());
  }

  fn exit_scope(&mut self) {
    self.scopes.pop();
  }

  fn cannot_apply(&mut self, expr: &Expr, op: &str, left: &Ty, right: &Ty) {
    let message = format!("cannot apply '{op}' to {} and {}", left.describe(), right.describe());
    self.report_at(expr, message);
  }

  fn report_at(&mut self, expr: &Expr, message: String) {
    self.report(expr.start, expr.end, message);
  }

  fn report(&mut self, start: usize, end: usize, message: impl Into<String>) {
    self.diagnostics.push(Diagnostic::new(start, end, message));
  }
}

/// The type two operands share, letting a constant take the other side's type.
fn common(left: &Ty, right: &Ty) -> Option<Ty> {
  if left == right {
    return Some(left.clone());
  }
  match (left.is_constant(), right.is_constant()) {
    (true, true) => Some(Ty::FloatConstant), // one of each
    (true, false) => takes_constant(right, left).then(|| right.clone()),
    (false, true) => takes_constant(left, right).then(|| left.clone()),
    (false, false) => None,
  }
}

fn takes_constant(ty: &Ty, constant: &Ty) -> bool {
  if *constant == Ty::IntegerConstant { ty.is_integer() || ty.is_float() } else { ty.is_float() }
}

fn literal_value(expr: &Expr) -> Option<i64> {
  match &expr.kind {
    ExprKind::IntLiteral(value) => Some(*value),
    ExprKind::Group(inner) => literal_value(inner),
    ExprKind::Unary { op: UnaryOp::Neg, operand } => literal_value(operand).map(i64::wrapping_neg),
    _ => None,
  }
}

fn resolve(ty: &Type) -> Ty {
  match ty.kind {
    TypeKind::Primitive(kind) => Ty::Primitive(kind),
    TypeKind::Error => Ty::Error,
  }
}

fn signature(f: &Function) -> FunctionSymbol {
  FunctionSymbol {
    name: f.name.clone(),
    parameter_names: f.parameters.iter().map(|p| p.name.clone()).collect(),
    parameter_types: f.parameters.iter().map(|p| resolve(&p.ty)).collect(),
    return_types: f.return_types.iter().map(resolve).collect(),
  }
}

fn count(n: usize, noun: &str) -> String {
  format!("{n} {noun}{}", if n == 1 { "" } else { "s" })
}
