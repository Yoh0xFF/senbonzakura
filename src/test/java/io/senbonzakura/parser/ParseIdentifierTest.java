package io.senbonzakura.parser;

import static io.senbonzakura.parser.Parsed.parse;
import static io.senbonzakura.parser.Parsed.parseClean;
import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.ast.Expr;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

class ParseIdentifierTest {

  @ParameterizedTest
  @ValueSource(strings = {"x", "_", "_x1", "camelCase", "snake_case", "ünïcödé", "日本語"})
  void identifiers(String input) {
    var parsed = parseClean(input);

    assertEquals(new Expr.Identifier(input, 0, input.length()), parsed.expr());
  }

  @Test
  void keywordIsNotAnIdentifier() {
    var parsed = parse("fn");

    assertEquals(new Expr.Error(0, 2), parsed.expr());
    assertEquals(List.of("expected expression, found 'fn'"), parsed.parserMessages());
  }
}
