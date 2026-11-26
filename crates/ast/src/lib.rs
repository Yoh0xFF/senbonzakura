//! The syntax tree the parser builds: [`Stmt`] nodes holding [`Expr`] and [`Type`] nodes.
//!
//! - Every node covers the source byte offsets `start` up to (not including) `end`, like a lexer
//!   token. The lexer's `SourceFile` maps them to lines and columns.
//! - A part that failed to parse becomes an [`ExprKind::Error`] or [`TypeKind::Error`] node rather
//!   than being dropped, so the tree keeps its shape after an error.
//! - Each [`Expr`] has an [`ExprId`] of its own, which the checker keys the types it works out by.
//!   Two expressions compare equal by kind and range, whatever their ids.

pub mod float_text;
pub mod sexpr;

use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

/// Tells one expression node from every other, even across separately parsed entries of a REPL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(u32);

impl ExprId {
  fn fresh() -> ExprId {
    static NEXT: AtomicU32 = AtomicU32::new(0);
    ExprId(NEXT.fetch_add(1, Ordering::Relaxed))
  }
}

/// An expression node. Literal nodes hold decoded values: `0x1F` becomes `31` and `"a\n"` becomes
/// a two-character string.
#[derive(Debug, Clone)]
pub struct Expr {
  pub id: ExprId,
  pub kind: ExprKind,
  pub start: usize,
  pub end: usize,
}

impl Expr {
  pub fn new(kind: ExprKind, start: usize, end: usize) -> Expr {
    debug_assert!(start <= end, "invalid node range {start}..{end}");
    Expr { id: ExprId::fresh(), kind, start, end }
  }
}

impl PartialEq for Expr {
  fn eq(&self, other: &Expr) -> bool {
    self.kind == other.kind && self.start == other.start && self.end == other.end
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
  IntLiteral(i64),
  FloatLiteral(f64),
  StringLiteral(String),
  CharLiteral(char),
  BoolLiteral(bool),
  Identifier(String),
  /// `callee(a, b)` with positional arguments, or `callee(x: a, y: b)` with named ones. A call's
  /// arguments are all one kind or all the other; the parser reports a mix, but keeps each
  /// argument as written.
  Call {
    callee: Box<Expr>,
    arguments: Vec<Argument>,
  },
  Index {
    target: Box<Expr>,
    index: Box<Expr>,
  },
  /// `target.name`. The name is empty when it was missing, which the parser has already
  /// reported.
  Field {
    target: Box<Expr>,
    name: String,
  },
  Unary {
    op: UnaryOp,
    operand: Box<Expr>,
  },
  Binary {
    op: BinaryOp,
    left: Box<Expr>,
    right: Box<Expr>,
  },
  /// `lower..upper`, which excludes `upper`.
  Range {
    lower: Box<Expr>,
    upper: Box<Expr>,
  },
  /// `target = value`, or a compound form like `target += value`. The parser has reported a
  /// target that cannot be assigned to, but still builds the node.
  Assign {
    op: AssignOp,
    target: Box<Expr>,
    value: Box<Expr>,
  },
  /// `target++` or `target--`. There is no prefix form. Like `Assign`, the parser has reported a
  /// target that cannot be assigned to, but still builds the node.
  Update {
    op: UpdateOp,
    target: Box<Expr>,
  },
  /// `value as type`, which converts `value` to `type`.
  Cast {
    value: Box<Expr>,
    ty: Type,
  },
  /// An expression in parentheses. Kept as its own node so the tree still says where the
  /// parentheses were, for diagnostics and formatting.
  Group(Box<Expr>),
  /// Stands in for an expression that could not be parsed. A diagnostic has already been
  /// reported, so later passes skip it without reporting again.
  Error,
}

/// One argument of a call: `value`, or `name: value`. `name` is `None` for a positional argument,
/// and empty when what came before the `:` was not a name, which the parser has already reported.
#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
  pub name: Option<String>,
  pub value: Expr,
  pub start: usize,
  pub end: usize,
}

impl Argument {
  pub fn is_named(&self) -> bool {
    self.name.is_some()
  }
}

macro_rules! spelled {
    ($(#[$meta:meta])* $name:ident { $($variant:ident = $spelling:literal,)* }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$variant,)*];

            pub fn spelling(self) -> &'static str {
                match self {
                    $($name::$variant => $spelling,)*
                }
            }
        }
    };
}

spelled!(UnaryOp {
    Neg = "-",
    Not = "!",
});

spelled!(
    /// `And` and `Or` skip the right operand when the left one settles the result.
    BinaryOp {
        Add = "+",
        Sub = "-",
        Mul = "*",
        Div = "/",
        Rem = "%",
        Eq = "==",
        Ne = "!=",
        Lt = "<",
        Le = "<=",
        Gt = ">",
        Ge = ">=",
        And = "&&",
        Or = "||",
    }
);

spelled!(AssignOp {
    Assign = "=",
    Add = "+=",
    Sub = "-=",
    Mul = "*=",
    Div = "/=",
    Rem = "%=",
});

spelled!(UpdateOp {
    Increment = "++",
    Decrement = "--",
});

/// A type written in the source, like the `int` in `var i: int`.
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
  pub kind: TypeKind,
  pub start: usize,
  pub end: usize,
}

impl Type {
  pub fn new(kind: TypeKind, start: usize, end: usize) -> Type {
    debug_assert!(start <= end, "invalid node range {start}..{end}");
    Type { kind, start, end }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
  Primitive(PrimitiveKind),
  /// Stands in for a type that could not be parsed. A diagnostic has already been reported, so
  /// later passes skip it without reporting again.
  Error,
}

spelled!(PrimitiveKind {
    // Signed integers
    Byte = "byte",
    Short = "short",
    Int = "int",
    Size = "size",
    Long = "long",

    // Unsigned integers
    Ubyte = "ubyte",
    Ushort = "ushort",
    Uint = "uint",
    Usize = "usize",
    Ulong = "ulong",

    // Floating point
    Float = "float",
    Double = "double",

    Char = "char",
    Bool = "bool",
    String = "string",
});

impl PrimitiveKind {
  /// The smallest value of an integer type, and 0 for the others. `size` and `usize` are taken
  /// to be 64 bits wide.
  pub fn min_value(self) -> i64 {
    match self {
      PrimitiveKind::Byte => i8::MIN.into(),
      PrimitiveKind::Short => i16::MIN.into(),
      PrimitiveKind::Int => i32::MIN.into(),
      PrimitiveKind::Size | PrimitiveKind::Long => i64::MIN,
      _ => 0,
    }
  }

  /// The largest value of an integer type that fits in an `i64`: that is all of them but
  /// `ulong` and `usize`, which no literal can go past anyway.
  pub fn max_value(self) -> i64 {
    match self {
      PrimitiveKind::Byte => i8::MAX.into(),
      PrimitiveKind::Short => i16::MAX.into(),
      PrimitiveKind::Int => i32::MAX.into(),
      PrimitiveKind::Ubyte => u8::MAX.into(),
      PrimitiveKind::Ushort => u16::MAX.into(),
      PrimitiveKind::Uint => u32::MAX.into(),
      _ => i64::MAX,
    }
  }
}

/// A statement node.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
  Expression(ExpressionStmt),
  Var(VarStmt),
  If(IfStmt),
  Loop(LoopStmt),
  LoopIn(LoopInStmt),
  LoopStep(LoopStepStmt),
  Continue(JumpStmt),
  Break(JumpStmt),
  Return(ReturnStmt),
  /// Shared, so a function value can outlive the statements it was declared among.
  Function(Rc<Function>),
  Block(Block),
}

impl Stmt {
  pub fn start(&self) -> usize {
    self.range().0
  }

  pub fn end(&self) -> usize {
    self.range().1
  }

  fn range(&self) -> (usize, usize) {
    match self {
      Stmt::Expression(s) => (s.start, s.end),
      Stmt::Var(s) => (s.start, s.end),
      Stmt::If(s) => (s.start, s.end),
      Stmt::Loop(s) => (s.start, s.end),
      Stmt::LoopIn(s) => (s.start, s.end),
      Stmt::LoopStep(s) => (s.start, s.end),
      Stmt::Continue(s) | Stmt::Break(s) => (s.start, s.end),
      Stmt::Return(s) => (s.start, s.end),
      Stmt::Function(s) => (s.start, s.end),
      Stmt::Block(s) => (s.start, s.end),
    }
  }
}

/// `expr;`. Covers the semicolon too, unless it was missing.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStmt {
  pub expr: Expr,
  pub start: usize,
  pub end: usize,
}

/// `var a: int = 1, b = 2;`, which declares one or more variables. Covers the semicolon too,
/// unless it was missing.
#[derive(Debug, Clone, PartialEq)]
pub struct VarStmt {
  /// Never empty.
  pub declarators: Vec<Declarator>,
  pub start: usize,
  pub end: usize,
}

/// One variable of a `var`: `name: type = initializer`, where `: type` and `= initializer` are
/// optional. `name` is empty when it was missing, which the parser has already reported.
#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
  pub name: String,
  pub ty: Option<Type>,
  pub initializer: Option<Expr>,
  pub start: usize,
  pub end: usize,
}

/// `if condition { ... } else { ... }`, where `else` is optional and may be followed by another
/// `if` instead of a block. The braces are required, so `then` is always a block, and `otherwise`
/// is a [`Stmt::Block`], a [`Stmt::If`] for `else if`, or `None` when there is no `else`. A body
/// that was missing, which the parser has already reported, is an empty block where it should
/// have started.
#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
  pub condition: Expr,
  pub then: Block,
  pub otherwise: Option<Box<Stmt>>,
  pub start: usize,
  pub end: usize,
}

/// `loop condition { ... }`, which runs while `condition` holds, or `loop { ... }`, which runs
/// until something leaves it, with no condition.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopStmt {
  pub condition: Option<Expr>,
  pub body: Block,
  pub start: usize,
  pub end: usize,
}

/// `loop var name: type in iterable { ... }`, where `: type` is optional. `name` is empty when it
/// was missing, which the parser has already reported.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopInStmt {
  pub name: String,
  pub ty: Option<Type>,
  pub iterable: Expr,
  pub body: Block,
  pub start: usize,
  pub end: usize,
}

/// `loop var i = 0; condition; step { ... }`. `init` runs once and covers its own `;`, and `step`
/// runs after each pass through the body.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopStepStmt {
  pub init: VarStmt,
  pub condition: Expr,
  pub step: Expr,
  pub body: Block,
  pub start: usize,
  pub end: usize,
}

/// `continue;` or `break;`, optionally `continue if condition;`, which jumps only when
/// `condition` holds. Covers the semicolon too, unless it was missing.
#[derive(Debug, Clone, PartialEq)]
pub struct JumpStmt {
  pub condition: Option<Expr>,
  pub start: usize,
  pub end: usize,
}

/// `return a, b if condition;`, which returns zero or more values; `values` is empty for a plain
/// `return`. With an `if condition` it returns only when the condition holds. Covers the
/// semicolon too, unless it was missing.
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
  pub values: Vec<Expr>,
  pub condition: Option<Expr>,
  pub start: usize,
  pub end: usize,
}

/// `fn name(a: int, b: int): int, bool { ... }`. `return_types` is empty when there is no `:`
/// after the parameters, so the function returns nothing, and holds several types when it
/// returns several values. `name` is empty when it was missing, which the parser has already
/// reported.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  pub name: String,
  pub parameters: Vec<Parameter>,
  pub return_types: Vec<Type>,
  pub body: Block,
  pub start: usize,
  pub end: usize,
}

/// One parameter of a function: `name: type`. The type is required; when it was missing, which
/// the parser has already reported, it is a [`TypeKind::Error`]. `name` is empty when it was
/// missing.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
  pub name: String,
  pub ty: Type,
  pub start: usize,
  pub end: usize,
}

/// `{ ... }`. Covers both braces, unless the closing one was missing.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
  pub statements: Vec<Stmt>,
  pub start: usize,
  pub end: usize,
}
