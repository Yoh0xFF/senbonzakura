package io.senbonzakura.lexer;

import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

// Chooses a scanner from the first character of each token. The scanners do the actual work;
// all state lives in the Cursor.
public class Lexer {

  private final SourceFile source;
  private final Cursor cursor;

  public Lexer(SourceFile source) {
    this.source = Objects.requireNonNull(source, "source");
    this.cursor = new Cursor(source.text());
  }

  public SourceFile source() {
    return source;
  }

  public List<Diagnostic> diagnostics() {
    return Collections.unmodifiableList(cursor.diagnostics());
  }

  public Token nextToken() {
    skipWhitespace();

    return switch (cursor.peek()) {
      case Cursor.EOF -> Token.eof(cursor.pos(), 0);
      case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' -> NumberScanner.scan(cursor);
      case '"' -> QuotedScanner.string(cursor);
      case '`' -> QuotedScanner.multilineString(cursor);
      case '\'' -> QuotedScanner.character(cursor);
      default -> unknownCharacter();
    };
  }

  private Token unknownCharacter() {
    // Consumes one character nothing else recognized, so lexing always makes progress.
    int start = cursor.pos();
    cursor.advance();
    cursor.report(start, "unexpected character");
    return cursor.token(Token.Kind.ERROR, start, 0);
  }

  private void skipWhitespace() {
    while (true) {
      switch (cursor.peek()) {
        case ' ', '\t', '\n', '\r' -> cursor.bump();
        default -> {
          return;
        }
      }
    }
  }
}
