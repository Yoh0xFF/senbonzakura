//! Parses prefix and infix operators, and owns their precedence. Infix operators, loosest first:
//!
//! 1. `= += -= *= /= %=`, which are right-associative and need a name, field or index on the left
//! 2. `..`, which cannot be chained: `a..b..c` is an error
//! 3. `||`
//! 4. `&&`
//! 5. `== != < <= > >=`, which cannot be chained: `a < b < c` is an error
//! 6. `+ -`
//! 7. `* / %`
//! 8. `as`, which converts the value on its left to the type on its right
//!
//! Prefix `-` and `!` bind tighter than all of them, and postfix calls, indexing, field access,
//! `++` and `--` tighter still. The infix operators from `||` down are left-associative. Operands
//! come from [`operand`](super::operand).

use super::cursor::TokenCursor;
use senbonzakura_ast::{AssignOp, BinaryOp, Expr, ExprKind, UnaryOp};
use senbonzakura_lexer::Kind;

const LOOSEST: u8 = 1;

pub(crate) fn expression(c: &mut TokenCursor) -> Expr {
  assignment(c)
}

pub(crate) fn prefix(c: &mut TokenCursor) -> Expr {
  let token = c.advance();
  let op = match token.kind {
    Kind::Minus => UnaryOp::Neg,
    Kind::Bang => UnaryOp::Not,
    kind => unreachable!("not a prefix operator: {kind:?}"),
  };
  let operand = super::operand(c);
  let end = operand.end;
  Expr::new(ExprKind::Unary { op, operand: Box::new(operand) }, token.start, end)
}

fn assignment(c: &mut TokenCursor) -> Expr {
  let target = range(c);
  let Some(op) = assign_op(c.peek().kind) else {
    return target;
  };
  c.advance();
  if !super::is_assignable(&target) {
    c.report(target.start, target.end, "invalid assignment target");
  }
  let value = assignment(c);
  let (start, end) = (target.start, value.end);
  Expr::new(ExprKind::Assign { op, target: Box::new(target), value: Box::new(value) }, start, end)
}

fn range(c: &mut TokenCursor) -> Expr {
  let mut left = infix(c, LOOSEST);
  let mut chained = false;
  while c.at(Kind::DotDot) {
    let op_token = c.advance();
    if chained {
      c.report_token(op_token, "range operators cannot be chained");
    }
    chained = true;

    let right = infix(c, LOOSEST);
    let (start, end) = (left.start, right.end);
    left = Expr::new(ExprKind::Range { lower: Box::new(left), upper: Box::new(right) }, start, end);
  }
  left
}

/// Parses operands joined by infix operators of precedence `min_precedence` or higher, by
/// precedence climbing. Each right operand is parsed one level tighter, which makes the operators
/// left-associative.
fn infix(c: &mut TokenCursor, min_precedence: u8) -> Expr {
  let mut left = cast(c);
  let mut after_comparison = false;
  loop {
    let Some(op) = binary_op(c.peek().kind).filter(|&op| precedence(op) >= min_precedence) else {
      return left;
    };
    let op_token = c.advance();
    if after_comparison && is_comparison(op) {
      c.report_token(op_token, "comparison operators cannot be chained; join them with '&&'");
    }
    after_comparison = is_comparison(op);

    let right = infix(c, precedence(op) + 1);
    let (start, end) = (left.start, right.end);
    left =
      Expr::new(ExprKind::Binary { op, left: Box::new(left), right: Box::new(right) }, start, end);
  }
}

/// Parses an operand followed by any number of `as type`. A cast binds tighter than every infix
/// operator but looser than prefix ones, so `-x as long` converts `-x`, as in Rust.
fn cast(c: &mut TokenCursor) -> Expr {
  let mut expr = super::operand(c);
  while c.at(Kind::As) {
    c.advance();
    let ty = super::parse_type::ty(c);
    let (start, end) = (expr.start, expr.end.max(ty.end));
    expr = Expr::new(ExprKind::Cast { value: Box::new(expr), ty }, start, end);
  }
  expr
}

/// Higher binds tighter.
fn precedence(op: BinaryOp) -> u8 {
  match op {
    BinaryOp::Or => 1,
    BinaryOp::And => 2,
    BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 3,
    BinaryOp::Add | BinaryOp::Sub => 4,
    BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => 5,
  }
}

fn is_comparison(op: BinaryOp) -> bool {
  precedence(op) == 3
}

fn assign_op(kind: Kind) -> Option<AssignOp> {
  Some(match kind {
    Kind::Eq => AssignOp::Assign,
    Kind::PlusEq => AssignOp::Add,
    Kind::MinusEq => AssignOp::Sub,
    Kind::StarEq => AssignOp::Mul,
    Kind::SlashEq => AssignOp::Div,
    Kind::PercentEq => AssignOp::Rem,
    _ => return None,
  })
}

fn binary_op(kind: Kind) -> Option<BinaryOp> {
  Some(match kind {
    Kind::Plus => BinaryOp::Add,
    Kind::Minus => BinaryOp::Sub,
    Kind::Star => BinaryOp::Mul,
    Kind::Slash => BinaryOp::Div,
    Kind::Percent => BinaryOp::Rem,
    Kind::EqEq => BinaryOp::Eq,
    Kind::BangEq => BinaryOp::Ne,
    Kind::Less => BinaryOp::Lt,
    Kind::LessEq => BinaryOp::Le,
    Kind::Greater => BinaryOp::Gt,
    Kind::GreaterEq => BinaryOp::Ge,
    Kind::AndAnd => BinaryOp::And,
    Kind::OrOr => BinaryOp::Or,
    _ => return None,
  })
}
