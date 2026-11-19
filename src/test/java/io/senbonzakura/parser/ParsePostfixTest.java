package io.senbonzakura.parser;

import static io.senbonzakura.parser.Parsed.parse;
import static io.senbonzakura.parser.Parsed.parseClean;
import static io.senbonzakura.parser.Parsed.parseFirst;
import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.ast.Expr;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ParsePostfixTest {

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "f()             => (call f)",
        "f(1)            => (call f 1)",
        "f(1, 2)         => (call f 1 2)",
        "f(1, 2,)        => (call f 1 2)",
        "f(a + b, -c)    => (call f (+ a b) (- c))",
        "f(g(x))         => (call f (call g x))",
      })
  void calls(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "a[0]            => (index a 0)",
        "a[i + 1]        => (index a (+ i 1))",
        "a[0..2]         => (index a (.. 0 2))",
        "a[b[0]]         => (index a (index b 0))",
      })
  void indexing(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "a.b             => (. a b)",
        "a.b.c           => (. (. a b) c)",
        "1.foo           => (. 1 foo)",
      })
  void fieldAccess(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "a.b(c)[0].d     => (. (index (call (. a b) c) 0) d)",
        "f()()           => (call (call f))",
        "a[0][1]         => (index (index a 0) 1)",
        "(a + b).c       => (. (group (+ a b)) c)",
        "(f)(x)          => (call (group f) x)",
      })
  void postfixOperatorsChainLeftToRight(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "-a.b            => (- (. a b))",
        "!f()            => (! (call f))",
        "- -a[0]         => (- (- (index a 0)))",
        "-a[0] * b.c     => (* (- (index a 0)) (. b c))",
        "a.b + f(c)      => (+ (. a b) (call f c))",
      })
  void postfixBindsTighterThanPrefixAndInfix(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "a.b = 1         => (= (. a b) 1)",
        "a[0] += 1       => (+= (index a 0) 1)",
        "f().x = 1       => (= (. (call f) x) 1)",
        "a.b[i].c = d    => (= (. (index (. a b) i) c) d)",
      })
  void fieldsAndIndexesCanBeAssigned(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @Test
  void callCannotBeAssigned() {
    var parsed = parse("f() = 1");

    assertEquals(List.of("invalid assignment target"), parsed.parserMessages());
    assertEquals(List.of("f()"), parsed.parserTexts());
  }

  @Test
  void nodesCoverTheirSource() {
    var parsed = parseClean("a.b(c)[0]");
    var index = parsed.as(Expr.Index.class);
    var call = (Expr.Call) index.target();
    var field = (Expr.Field) call.callee();

    assertEquals("a.b(c)[0]", parsed.text(index));
    assertEquals("a.b(c)", parsed.text(call));
    assertEquals("a.b", parsed.text(field));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "f(1 2)      => (call f 1 2)          => expected ',' or ')', found '2'",
        "f(1         => (call f 1)            => expected ')', found end of input",
        "f(          => (call f)              => expected ')', found end of input",
        "f(,)        => (call f <error>)      => expected expression, found ','",
        "f(1;        => (call f 1)            => expected ')', found ';'",
        "a[]         => (index a <error>)     => expected expression, found ']'",
        "f(1 {       => (call f 1)            => expected ')', found '{'",
        "f(1 if      => (call f 1)            => expected ')', found 'if'",
        "a[0         => (index a 0)           => expected ']', found end of input",
        "a.          => (. a <error>)         => expected field name, found end of input",
      })
  void malformedPostfixIsReported(String input, String tree, String message) {
    var parsed = parseFirst(input);

    assertEquals(tree, Sexpr.print(parsed.expr()));
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "i++             => (++ i)",
        "i--             => (-- i)",
        "a.b++           => (++ (. a b))",
        "a[0]--          => (-- (index a 0))",
        "-i++            => (- (++ i))",
        "i++ + 1         => (+ (++ i) 1)",
        "i++ < n         => (< (++ i) n)",
        "a+++b           => (+ (++ a) b)",
        "a - -b          => (- a (- b))",
      })
  void incrementAndDecrement(String input, String tree) {
    assertEquals(tree, Sexpr.print(parseClean(input).expr()));
  }

  @Test
  void updateCoversItsSource() {
    var parsed = parseClean("a.b[0]++");

    assertEquals("a.b[0]++", parsed.text(parsed.expr()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "f()++           => (++ (call f))          => invalid target for '++'",
        "1--             => (-- 1)                 => invalid target for '--'",
        "i++++           => (++ (++ i))            => invalid target for '++'",
        "(i)++           => (++ (group i))         => invalid target for '++'",
      })
  void updateOfSomethingNotAssignableIsReported(String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, Sexpr.print(parsed.expr()));
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @Test
  void thereIsNoPrefixIncrement() {
    var parsed = parseFirst("--i");

    assertEquals(List.of("expected expression, found '--'"), parsed.parserMessages());
  }

  @Test
  void unclosedCallEndsAtItsLastArgument() {
    var parsed = parse("f(1, 2");

    assertEquals("f(1, 2", parsed.text(parsed.expr()));
  }

  @Test
  void keywordIsNotAFieldName() {
    var parsed = parseFirst("a.fn");

    assertEquals("(. a <error>)", Sexpr.print(parsed.expr()));
    assertEquals(List.of("expected field name, found 'fn'"), parsed.parserMessages());
    assertEquals("a.", parsed.text(parsed.expr()));
  }
}
