package io.senbonzakura.parser;

import static io.senbonzakura.parser.ParsedStmt.parse;
import static io.senbonzakura.parser.ParsedStmt.parseClean;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;

import io.senbonzakura.ast.Stmt;
import io.senbonzakura.ast.Type;
import java.util.List;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.EnumSource;

class ParseTypeTest {

  @ParameterizedTest
  @EnumSource(Type.PrimitiveKind.class)
  void everyPrimitiveTypeParses(Type.PrimitiveKind kind) {
    var parsed = parseClean("var x: " + kind.spelling() + ";");
    var type = parsed.as(Stmt.Var.class).declarators().getFirst().type();

    var primitive = assertInstanceOf(Type.Primitive.class, type);
    assertEquals(kind, primitive.kind());
    assertEquals(kind.spelling(), parsed.text(type));
  }

  @ParameterizedTest
  @EnumSource(Type.PrimitiveKind.class)
  void primitiveTypeNamesAreNotVariableNames(Type.PrimitiveKind kind) {
    var parsed = parse("var " + kind.spelling() + " = 1;");

    assertEquals("(var <error> 1)", parsed.tree());
    assertEquals(
        List.of("expected variable name, found '" + kind.spelling() + "'"),
        parsed.parserMessages());
  }
}
