//! How values are stored at run time, and the operations on them. The static type of each
//! operation comes from the checker, so a value carries no more type than it needs to be stored:
//!
//! - Integers of every width and `char` are [`Value::Int`]s, kept within their type's range:
//!   arithmetic wraps around, as in Go. `ulong` and `usize` use all 64 bits, and compare and
//!   divide as unsigned.
//! - `float` is a [`Value::Float`], `double` a [`Value::Double`], `bool` a [`Value::Bool`] and
//!   `string` a [`Value::Str`].
//! - A range is a [`Value::Range`].
//! - An integer constant nothing settled, like those in `1 < 2`, is computed as a `long`, and a
//!   float constant as a `double`.

use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use super::RuntimeError;
use super::env::Function;
use senbonzakura_ast::float_text;
use senbonzakura_ast::{BinaryOp, Expr, PrimitiveKind};
use senbonzakura_check::Ty;

#[derive(Debug, Clone)]
pub(crate) enum Value {
  Int(i64),
  Float(f32),
  Double(f64),
  Bool(bool),
  Str(Rc<str>),
  /// What `lower..upper` evaluates to.
  Range(i64, i64),
  Function(Rc<Function>),
  Print,
  /// What calling a function with several return types gives; only a statement receives it.
  Multiple(Vec<Value>),
  /// What calling a function without return types gives.
  Nothing,
}

impl Value {
  pub fn int(&self) -> i64 {
    match self {
      Value::Int(i) => *i,
      other => unreachable!("the checker lets no {other:?} through as an integer"),
    }
  }

  pub fn bool(&self) -> bool {
    match self {
      Value::Bool(b) => *b,
      other => unreachable!("the checker lets no {other:?} through as a bool"),
    }
  }

  fn number(&self) -> f64 {
    match self {
      Value::Int(i) => *i as f64,
      Value::Float(f) => f64::from(*f),
      Value::Double(d) => *d,
      other => unreachable!("not a number: {other:?}"),
    }
  }
}

/// The value an integer literal stands for, as its settled type stores it.
pub(crate) fn integer_literal(value: i64, ty: &Ty) -> Value {
  match ty {
    Ty::FloatConstant | Ty::Primitive(PrimitiveKind::Double) => Value::Double(value as f64),
    Ty::Primitive(PrimitiveKind::Float) => Value::Float(value as f32),
    Ty::Primitive(kind) => Value::Int(wrap(value, *kind)),
    _ => Value::Int(value),
  }
}

pub(crate) fn float_literal(value: f64, ty: &Ty) -> Value {
  if is_float32(ty) { Value::Float(value as f32) } else { Value::Double(value) }
}

/// The value a variable declared without an initializer starts with.
pub(crate) fn zero(kind: PrimitiveKind) -> Value {
  match kind {
    PrimitiveKind::Float => Value::Float(0.0),
    PrimitiveKind::Double => Value::Double(0.0),
    PrimitiveKind::Bool => Value::Bool(false),
    PrimitiveKind::String => Value::Str("".into()),
    _ => Value::Int(0),
  }
}

/// One, for `++` and `--`.
pub(crate) fn one(ty: &Ty) -> Value {
  integer_literal(1, ty)
}

/// `+ - * / %` on two values of `ty`; `+` also joins strings.
pub(crate) fn arithmetic(
  op: BinaryOp,
  ty: &Ty,
  left: &Value,
  right: &Value,
  at: &Expr,
) -> Result<Value, RuntimeError> {
  Ok(match (left, right) {
    (Value::Str(a), Value::Str(b)) => Value::Str(format!("{a}{b}").into()),
    (Value::Float(a), Value::Float(b)) => Value::Float(float_op(op, *a, *b)),
    (Value::Double(a), Value::Double(b)) => Value::Double(float_op(op, *a, *b)),
    (Value::Int(a), Value::Int(b)) => {
      let (a, b) = (*a, *b);
      if matches!(op, BinaryOp::Div | BinaryOp::Rem) && b == 0 {
        return Err(RuntimeError::at(at.start, at.end, "division by zero"));
      }
      let unsigned = is_unsigned64(ty);
      let result = match op {
        BinaryOp::Add => a.wrapping_add(b),
        BinaryOp::Sub => a.wrapping_sub(b),
        BinaryOp::Mul => a.wrapping_mul(b),
        BinaryOp::Div if unsigned => ((a as u64) / (b as u64)) as i64,
        BinaryOp::Div => a.wrapping_div(b),
        BinaryOp::Rem if unsigned => ((a as u64) % (b as u64)) as i64,
        BinaryOp::Rem => a.wrapping_rem(b),
        _ => unreachable!("not arithmetic: {op:?}"),
      };
      match ty {
        Ty::Primitive(kind) => Value::Int(wrap(result, *kind)),
        _ => Value::Int(result),
      }
    }
    _ => unreachable!("the checker lets no {left:?} {op:?} {right:?} through"),
  })
}

fn float_op<F>(op: BinaryOp, a: F, b: F) -> F
where
  F: std::ops::Add<Output = F>
    + std::ops::Sub<Output = F>
    + std::ops::Mul<Output = F>
    + std::ops::Div<Output = F>
    + std::ops::Rem<Output = F>,
{
  match op {
    BinaryOp::Add => a + b,
    BinaryOp::Sub => a - b,
    BinaryOp::Mul => a * b,
    BinaryOp::Div => a / b,
    BinaryOp::Rem => a % b,
    _ => unreachable!("not arithmetic: {op:?}"),
  }
}

/// `== != < <= > >=` on two values of `ty`.
pub(crate) fn compare(op: BinaryOp, ty: &Ty, left: &Value, right: &Value) -> bool {
  match (left, right) {
    (Value::Int(a), Value::Int(b)) => {
      let order = if is_unsigned64(ty) { (*a as u64).cmp(&(*b as u64)) } else { a.cmp(b) };
      ordered(op, order)
    }
    (Value::Float(_) | Value::Double(_), Value::Float(_) | Value::Double(_)) => {
      // Comparisons as numbers, so NaN is unequal and unordered.
      let (x, y) = (left.number(), right.number());
      match op {
        BinaryOp::Eq => x == y,
        BinaryOp::Ne => x != y,
        BinaryOp::Lt => x < y,
        BinaryOp::Le => x <= y,
        BinaryOp::Gt => x > y,
        BinaryOp::Ge => x >= y,
        _ => unreachable!("not a comparison: {op:?}"),
      }
    }
    // By UTF-16 units, as the language has always ordered strings.
    (Value::Str(a), Value::Str(b)) => ordered(op, a.encode_utf16().cmp(b.encode_utf16())),
    (Value::Bool(a), Value::Bool(b)) => ordered(op, a.cmp(b)),
    _ => unreachable!("the checker lets no {left:?} {op:?} {right:?} through"),
  }
}

pub(crate) fn negate(ty: &Ty, value: &Value) -> Value {
  match value {
    Value::Float(f) => Value::Float(-f),
    Value::Double(d) => Value::Double(-d),
    Value::Int(i) => match ty {
      Ty::Primitive(kind) => Value::Int(wrap(i.wrapping_neg(), *kind)),
      _ => Value::Int(i.wrapping_neg()),
    },
    other => unreachable!("cannot negate {other:?}"),
  }
}

/// `value as to`. Integers wrap into a narrower integer type. A float truncates toward zero and
/// saturates at the integer type's range, as in Rust, with NaN becoming zero. An integer that is
/// not a code point fails as a `char`.
pub(crate) fn cast(value: &Value, from: &Ty, to: &Ty, at: &Expr) -> Result<Value, RuntimeError> {
  let Ty::Primitive(kind) = *to else { unreachable!("a cast is to a primitive type: {to:?}") };
  if let Value::Int(integer) = *value {
    return Ok(match kind {
      PrimitiveKind::Char => {
        if u32::try_from(integer).ok().and_then(char::from_u32).is_none() {
          let message = format!("{integer} is not a valid char");
          return Err(RuntimeError::at(at.start, at.end, message));
        }
        Value::Int(integer)
      }
      PrimitiveKind::Float | PrimitiveKind::Double => {
        let d = if is_unsigned64(from) { integer as u64 as f64 } else { integer as f64 };
        if kind == PrimitiveKind::Float { Value::Float(d as f32) } else { Value::Double(d) }
      }
      _ => Value::Int(wrap(integer, kind)),
    });
  }

  let d = value.number();
  Ok(match kind {
    PrimitiveKind::Float => Value::Float(d as f32),
    PrimitiveKind::Double => Value::Double(d),
    // `as` saturates at the range and turns NaN into zero.
    PrimitiveKind::Ulong | PrimitiveKind::Usize => Value::Int(d as u64 as i64),
    _ => Value::Int((d as i64).clamp(kind.min_value(), kind.max_value())),
  })
}

/// How `print` shows a value of `ty`.
pub(crate) fn format(ty: &Ty, value: &Value) -> String {
  match (ty, value) {
    (Ty::Range(element), Value::Range(lower, upper)) => {
      format!("{}..{}", format(element, &Value::Int(*lower)), format(element, &Value::Int(*upper)))
    }
    (Ty::Primitive(PrimitiveKind::Char), Value::Int(code)) => u32::try_from(*code)
      .ok()
      .and_then(char::from_u32)
      .expect("a char holds a code point")
      .to_string(),
    (_, Value::Int(i)) if is_unsigned64(ty) => (*i as u64).to_string(),
    _ => value.to_string(),
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Int(i) => write!(f, "{i}"),
      Value::Float(x) => f.write_str(&float_text::float(*x)),
      Value::Double(x) => f.write_str(&float_text::double(*x)),
      Value::Bool(b) => write!(f, "{b}"),
      Value::Str(s) => f.write_str(s),
      Value::Range(lower, upper) => write!(f, "{lower}..{upper}"),
      Value::Function(function) => write!(f, "<fn {}>", function.declaration.name),
      Value::Print => f.write_str("<fn print>"),
      Value::Multiple(values) => {
        let values: Vec<_> = values.iter().map(Value::to_string).collect();
        f.write_str(&values.join(", "))
      }
      Value::Nothing => f.write_str("<nothing>"),
    }
  }
}

/// Keeps `value` within the range of `kind`, wrapping around as two's complement.
pub(crate) fn wrap(value: i64, kind: PrimitiveKind) -> i64 {
  match kind {
    PrimitiveKind::Byte => (value as i8).into(),
    PrimitiveKind::Short => (value as i16).into(),
    PrimitiveKind::Int => (value as i32).into(),
    PrimitiveKind::Ubyte => (value as u8).into(),
    PrimitiveKind::Ushort => (value as u16).into(),
    PrimitiveKind::Uint => (value as u32).into(),
    _ => value,
  }
}

fn ordered(op: BinaryOp, order: Ordering) -> bool {
  match op {
    BinaryOp::Eq => order.is_eq(),
    BinaryOp::Ne => order.is_ne(),
    BinaryOp::Lt => order.is_lt(),
    BinaryOp::Le => order.is_le(),
    BinaryOp::Gt => order.is_gt(),
    BinaryOp::Ge => order.is_ge(),
    _ => unreachable!("not a comparison: {op:?}"),
  }
}

fn is_float32(ty: &Ty) -> bool {
  *ty == Ty::Primitive(PrimitiveKind::Float)
}

fn is_unsigned64(ty: &Ty) -> bool {
  matches!(ty, Ty::Primitive(PrimitiveKind::Ulong | PrimitiveKind::Usize))
}
