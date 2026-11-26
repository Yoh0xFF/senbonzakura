use super::{parse, parse_clean, parse_first};
use senbonzakura_ast::{AssignOp, BinaryOp, ExprKind, UnaryOp};

fn assert_trees(cases: &[(&str, &str)]) {
  for &(input, tree) in cases {
    assert_eq!(parse_clean(input).tree(), tree, "{input:?}");
  }
}

#[test]
fn each_operator() {
  assert_trees(&[
    ("1 + 2", "(+ 1 2)"),
    ("1 - 2", "(- 1 2)"),
    ("1 * 2", "(* 1 2)"),
    ("1 / 2", "(/ 1 2)"),
    ("1 % 2", "(% 1 2)"),
    ("1 == 2", "(== 1 2)"),
    ("1 != 2", "(!= 1 2)"),
    ("1 < 2", "(< 1 2)"),
    ("1 <= 2", "(<= 1 2)"),
    ("1 > 2", "(> 1 2)"),
    ("1 >= 2", "(>= 1 2)"),
    ("true && false", "(&& true false)"),
    ("true || false", "(|| true false)"),
    ("-1", "(- 1)"),
    ("!true", "(! true)"),
  ]);
}

#[test]
fn casts() {
  assert_trees(&[
    ("x as long", "(as x long)"),
    ("-x as long", "(as (- x) long)"),
    ("!b as int", "(as (! b) int)"),
    ("a + b as int", "(+ a (as b int))"),
    ("a * b as int", "(* a (as b int))"),
    ("x as int as long", "(as (as x int) long)"),
    ("f(x).y[0] as double", "(as (index (. (call f x) y) 0) double)"),
    ("x as int < y", "(< (as x int) y)"),
    ("(a + b) as ulong", "(as (group (+ a b)) ulong)"),
    ("x = y as byte", "(= x (as y byte))"),
  ]);
}

#[test]
fn malformed_cast_is_reported_once() {
  let cases = [
    ("x as", "(as x <error>)", "expected type, found end of input"),
    ("x as foo", "(as x <error>)", "expected type, found 'foo'"),
    ("x as 1 + 2", "(+ (as x <error>) 2)", "expected type, found '1'"),
  ];
  for (input, tree, message) in cases {
    let parsed = parse(input);

    assert_eq!(parsed.tree(), tree, "{input:?}");
    assert_eq!(parsed.parser_messages(), [message]);
  }
}

#[test]
fn precedence() {
  assert_trees(&[
    // Tighter levels group first.
    ("1 + 2 * 3", "(+ 1 (* 2 3))"),
    ("1 * 2 + 3", "(+ (* 1 2) 3)"),
    ("1 + 2 < 3 * 4", "(< (+ 1 2) (* 3 4))"),
    ("1 < 2 && 3 < 4", "(&& (< 1 2) (< 3 4))"),
    ("true || false && true", "(|| true (&& false true))"),
    ("true && false || true", "(|| (&& true false) true)"),
    // Prefix operators bind tightest.
    ("-1 * 2", "(* (- 1) 2)"),
    ("!true && false", "(&& (! true) false)"),
    ("- -1", "(- (- 1))"),
    ("!-1 == 2", "(== (! (- 1)) 2)"),
    ("1 - -2", "(- 1 (- 2))"),
  ]);
}

#[test]
fn infix_operators_are_left_associative() {
  assert_trees(&[
    ("1 - 2 - 3", "(- (- 1 2) 3)"),
    ("8 / 4 / 2", "(/ (/ 8 4) 2)"),
    ("1 + 2 - 3 + 4", "(+ (- (+ 1 2) 3) 4)"),
    ("true && true && true", "(&& (&& true true) true)"),
    ("true || true || true", "(|| (|| true true) true)"),
  ]);
}

#[test]
fn nodes_cover_their_source() {
  let parsed = parse_clean("-(1 + 2) * 3");
  let ExprKind::Binary { op: mul_op, left: neg, .. } = &parsed.expr.kind else { panic!() };
  let ExprKind::Unary { op: neg_op, operand: group } = &neg.kind else { panic!() };
  let ExprKind::Group(add) = &group.kind else { panic!() };
  let ExprKind::Binary { op: add_op, .. } = &add.kind else { panic!() };

  assert_eq!(*mul_op, BinaryOp::Mul);
  assert_eq!(*neg_op, UnaryOp::Neg);
  assert_eq!(*add_op, BinaryOp::Add);
  assert_eq!(parsed.expr_text(), "-(1 + 2) * 3");
  assert_eq!(parsed.text(&**neg), "-(1 + 2)");
  assert_eq!(parsed.text(&**group), "(1 + 2)");
  assert_eq!(parsed.text(&**add), "1 + 2");
}

#[test]
fn chained_comparisons_are_reported() {
  let cases = [
    ("1 < 2 < 3", "(< (< 1 2) 3)", "<"),
    ("1 == 2 != 3", "(!= (== 1 2) 3)", "!="),
    ("1 < 2 + 3 >= 4", "(>= (< 1 (+ 2 3)) 4)", ">="),
  ];
  for (input, tree, reported) in cases {
    let parsed = parse(input);

    assert_eq!(parsed.tree(), tree, "{input:?}");
    assert_eq!(
      parsed.parser_messages(),
      ["comparison operators cannot be chained; join them with '&&'"]
    );
    assert_eq!(parsed.parser_texts(), [reported]);
  }
}

#[test]
fn comparisons_joined_by_logical_operators_are_fine() {
  parse_clean("1 < 2 && 2 < 3 || 3 == 3");
}

#[test]
fn missing_operand_is_reported() {
  let cases = [
    ("1 +", "(+ 1 <error>)", "expected expression, found end of input"),
    ("-", "(- <error>)", "expected expression, found end of input"),
    ("1 * * 2", "(* 1 <error>)", "expected expression, found '*'"),
  ];
  for (input, tree, message) in cases {
    let parsed = parse_first(input);

    assert_eq!(parsed.tree(), tree, "{input:?}");
    assert_eq!(parsed.parser_messages()[0], message);
  }
}

#[test]
fn each_assignment_operator() {
  assert_trees(&[
    ("x = 1", "(= x 1)"),
    ("x += 1", "(+= x 1)"),
    ("x -= 1", "(-= x 1)"),
    ("x *= 1", "(*= x 1)"),
    ("x /= 1", "(/= x 1)"),
    ("x %= 1", "(%= x 1)"),
  ]);
}

#[test]
fn assignment_is_loosest_and_right_associative() {
  assert_trees(&[
    ("a = b = 1", "(= a (= b 1))"),
    ("a += b -= 1", "(+= a (-= b 1))"),
    ("x = 1 + 2 * 3", "(= x (+ 1 (* 2 3)))"),
    ("x = a || b", "(= x (|| a b))"),
    ("x = 0..10", "(= x (.. 0 10))"),
    ("ok = a < b && c", "(= ok (&& (< a b) c))"),
    ("x = -y", "(= x (- y))"),
  ]);
}

#[test]
fn invalid_assignment_target_is_reported() {
  let cases = [
    ("1 = 2", "(= 1 2)", "1"),
    ("a + b = c", "(= (+ a b) c)", "a + b"),
    ("-x = 1", "(= (- x) 1)", "-x"),
    ("(x) = 1", "(= (group x) 1)", "(x)"),
    ("a..b += 1", "(+= (.. a b) 1)", "a..b"),
    ("x = 1 = 2", "(= x (= 1 2))", "1"),
  ];
  for (input, tree, reported) in cases {
    let parsed = parse(input);

    assert_eq!(parsed.tree(), tree, "{input:?}");
    assert_eq!(parsed.parser_messages(), ["invalid assignment target"]);
    assert_eq!(parsed.parser_texts(), [reported]);
  }
}

#[test]
fn error_target_is_not_reported_again() {
  let parsed = parse(": = 1");

  assert_eq!(parsed.tree(), "(= <error> 1)");
  assert_eq!(parsed.parser_messages(), ["expected expression, found ':'"]);
}

#[test]
fn range_is_looser_than_logical_operators() {
  assert_trees(&[
    ("0..10", "(.. 0 10)"),
    ("a + 1..b * 2", "(.. (+ a 1) (* b 2))"),
    ("a < b..c", "(.. (< a b) c)"),
    ("a || b..c", "(.. (|| a b) c)"),
    ("-1..1", "(.. (- 1) 1)"),
    ("(0..1)..2", "(.. (group (.. 0 1)) 2)"),
  ]);
}

#[test]
fn chained_ranges_are_reported() {
  let parsed = parse("0..1..2");

  assert_eq!(parsed.tree(), "(.. (.. 0 1) 2)");
  assert_eq!(parsed.parser_messages(), ["range operators cannot be chained"]);
  assert_eq!(parsed.parser_texts(), [".."]);
  assert_eq!(parsed.parser_diagnostics[0].start, 4);
}

#[test]
fn missing_right_side_is_reported() {
  for (input, tree) in
    [("x =", "(= x <error>)"), ("x +=", "(+= x <error>)"), ("0..", "(.. 0 <error>)")]
  {
    let parsed = parse(input);

    assert_eq!(parsed.tree(), tree, "{input:?}");
    assert_eq!(parsed.parser_messages(), ["expected expression, found end of input"]);
  }
}

#[test]
fn assignment_and_range_cover_their_source() {
  let parsed = parse_clean("x = 0..10");
  let ExprKind::Assign { op, target, value } = &parsed.expr.kind else { panic!() };

  assert_eq!(*op, AssignOp::Assign);
  assert_eq!(parsed.expr_text(), "x = 0..10");
  assert_eq!(parsed.text(&**target), "x");
  assert_eq!(parsed.text(&**value), "0..10");
}
