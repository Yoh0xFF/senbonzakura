package io.senbonzakura.parser;

import static io.senbonzakura.parser.Parsed.parse;
import static io.senbonzakura.parser.Parsed.parseClean;
import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Expr.BinaryOp;
import io.senbonzakura.ast.Expr.UnaryOp;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ParseOperatorTest {

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "1 + 2       => (+ 1 2)",
        "1 - 2       => (- 1 2)",
        "1 * 2       => (* 1 2)",
        "1 / 2       => (/ 1 2)",
        "1 % 2       => (% 1 2)",
        "1 == 2      => (== 1 2)",
        "1 != 2      => (!= 1 2)",
        "1 < 2       => (< 1 2)",
        "1 <= 2      => (<= 1 2)",
        "1 > 2       => (> 1 2)",
        "1 >= 2      => (>= 1 2)",
        "true && false => (&& true false)",
        "true || false => (|| true false)",
        "-1          => (- 1)",
        "!true       => (! true)",
      })
  void eachOperator(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        // Tighter levels group first.
        "1 + 2 * 3            => (+ 1 (* 2 3))",
        "1 * 2 + 3            => (+ (* 1 2) 3)",
        "1 + 2 < 3 * 4        => (< (+ 1 2) (* 3 4))",
        "1 < 2 && 3 < 4       => (&& (< 1 2) (< 3 4))",
        "true || false && true => (|| true (&& false true))",
        "true && false || true => (|| (&& true false) true)",
        // Prefix operators bind tightest.
        "-1 * 2               => (* (- 1) 2)",
        "!true && false       => (&& (! true) false)",
        "--1                  => (- (- 1))",
        "!-1 == 2             => (== (! (- 1)) 2)",
        "1 - -2               => (- 1 (- 2))",
      })
  void precedence(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "1 - 2 - 3            => (- (- 1 2) 3)",
        "8 / 4 / 2            => (/ (/ 8 4) 2)",
        "1 + 2 - 3 + 4        => (+ (- (+ 1 2) 3) 4)",
        "true && true && true => (&& (&& true true) true)",
        "true || true || true => (|| (|| true true) true)",
      })
  void infixOperatorsAreLeftAssociative(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @Test
  void nodesCoverTheirSource() {
    var parsed = parseClean("-(1 + 2) * 3");
    var mul = parsed.as(Expr.Binary.class);
    var neg = (Expr.Unary) mul.left();
    var group = (Expr.Group) neg.operand();
    var add = (Expr.Binary) group.inner();

    assertEquals(BinaryOp.MUL, mul.op());
    assertEquals(UnaryOp.NEG, neg.op());
    assertEquals(BinaryOp.ADD, add.op());
    assertEquals("-(1 + 2) * 3", parsed.text(mul));
    assertEquals("-(1 + 2)", parsed.text(neg));
    assertEquals("(1 + 2)", parsed.text(group));
    assertEquals("1 + 2", parsed.text(add));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "1 < 2 < 3      => (< (< 1 2) 3)       => <",
        "1 == 2 != 3    => (!= (== 1 2) 3)     => !=",
        "1 < 2 + 3 >= 4 => (>= (< 1 (+ 2 3)) 4) => >=",
      })
  void chainedComparisonsAreReported(String input, String tree, String reported) {
    var parsed = parse(input);

    assertEquals(tree, Sexpr.print(parsed.expr()));
    assertEquals(
        List.of("comparison operators cannot be chained; join them with '&&'"),
        parsed.parserMessages());
    assertEquals(List.of(reported), parsed.parserTexts());
  }

  @Test
  void comparisonsJoinedByLogicalOperatorsAreFine() {
    parseClean("1 < 2 && 2 < 3 || 3 == 3");
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "1 +   => (+ 1 <error>)     => expected expression, found end of input",
        "-     => (- <error>)       => expected expression, found end of input",
        "1 * * 2 => (* 1 <error>)   => expected expression, found '*'",
      })
  void missingOperandIsReported(String input, String tree, String message) {
    var parsed = Parsed.parseFirst(input);

    assertEquals(tree, Sexpr.print(parsed.expr()));
    assertEquals(message, parsed.parserMessages().getFirst());
  }
}
