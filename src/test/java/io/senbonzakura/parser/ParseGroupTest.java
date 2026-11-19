package io.senbonzakura.parser;

import static io.senbonzakura.parser.Parsed.parse;
import static io.senbonzakura.parser.Parsed.parseClean;
import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Sexpr;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ParseGroupTest {

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "(1)               => (group 1)",
        "(1 + 2) * 3       => (* (group (+ 1 2)) 3)",
        "1 - (2 - 3)       => (- 1 (group (- 2 3)))",
        "-(1 + 2)          => (- (group (+ 1 2)))",
        "((1))             => (group (group 1))",
        "(1 < 2) == true   => (== (group (< 1 2)) true)",
      })
  void parenthesesOverridePrecedence(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @Test
  void missingOperandBeforeCloseParenLeavesTheParenForTheGroup() {
    var parsed = parse("(1 + )");

    assertEquals("(group (+ 1 <error>))", Sexpr.print(parsed.expr()));
    assertEquals(List.of("expected expression, found ')'"), parsed.parserMessages());
    var group = parsed.as(Expr.Group.class);
    assertEquals("(1 + )", parsed.text(group));
    assertEquals(new Expr.Error(5, 5), ((Expr.Binary) group.inner()).right());
  }

  @Test
  void unclosedParenIsReported() {
    var parsed = parse("(1 + 2");

    assertEquals("(group (+ 1 2))", Sexpr.print(parsed.expr()));
    assertEquals(List.of("expected ')', found end of input"), parsed.parserMessages());
    assertEquals("(1 + 2", parsed.text(parsed.expr()));
  }
}
