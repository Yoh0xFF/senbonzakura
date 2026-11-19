package io.senbonzakura.parser;

import io.senbonzakura.ast.Type;
import io.senbonzakura.ast.Type.PrimitiveKind;
import io.senbonzakura.lexer.Token;

/** Parses types. So far only the primitive types, each spelled by a keyword of its own. */
final class ParseType {

  private ParseType() {}

  /**
   * Parses the type at the cursor. Anything else is reported, and consumed unless it can follow a
   * type, so {@code var x: foo = 1} reports {@code foo} once and still parses the initializer.
   */
  static Type type(TokenCursor c) {
    PrimitiveKind kind = primitive(c.peek().kind());
    if (kind != null) {
      Token token = c.advance();
      return new Type.Primitive(kind, token.start(), token.end());
    }

    Token token = c.peek();
    c.report(token, "expected type, found " + c.describe(token));
    if (canFollowType(c)) {
      return new Type.Error(token.start(), token.start());
    }
    c.advance();
    return new Type.Error(token.start(), token.end());
  }

  /** Whether the cursor is on a token that starts a type. */
  static boolean atType(TokenCursor c) {
    return primitive(c.peek().kind()) != null;
  }

  /** Returns {@code null} if the token is not a primitive type. */
  private static PrimitiveKind primitive(Token.Kind kind) {
    return switch (kind) {
      case BYTE -> PrimitiveKind.BYTE;
      case SHORT -> PrimitiveKind.SHORT;
      case INT -> PrimitiveKind.INT;
      case SIZE -> PrimitiveKind.SIZE;
      case LONG -> PrimitiveKind.LONG;
      case UBYTE -> PrimitiveKind.UBYTE;
      case USHORT -> PrimitiveKind.USHORT;
      case UINT -> PrimitiveKind.UINT;
      case USIZE -> PrimitiveKind.USIZE;
      case ULONG -> PrimitiveKind.ULONG;
      case FLOAT -> PrimitiveKind.FLOAT;
      case DOUBLE -> PrimitiveKind.DOUBLE;
      case CHAR -> PrimitiveKind.CHAR;
      case BOOL -> PrimitiveKind.BOOL;
      case STRING -> PrimitiveKind.STRING;
      default -> null;
    };
  }

  private static boolean canFollowType(TokenCursor c) {
    return switch (c.peek().kind()) {
      case EQ, COMMA, SEMICOLON, IN, LEFT_BRACE, RIGHT_BRACE, RIGHT_PAREN, EOF -> true;
      default -> false;
    };
  }
}
