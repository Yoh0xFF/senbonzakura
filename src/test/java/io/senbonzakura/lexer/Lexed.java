package io.senbonzakura.lexer;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.fail;

import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.ArrayList;
import java.util.List;

// Lexes a whole input up to and including EOF, and keeps everything the tests look at.
record Lexed(SourceFile source, List<Token> tokens, List<Diagnostic> diagnostics) {

  static Lexed lex(String text) {
    var source = new SourceFile("test", text);
    var lexer = new Lexer(source);
    var tokens = new ArrayList<Token>();
    // Every token except EOF consumes at least one char, so more than length + 1 means it's stuck.
    for (int i = 0; i <= text.length() + 1; i++) {
      var token = lexer.nextToken();
      tokens.add(token);
      if (token.kind() == Token.Kind.EOF) {
        return new Lexed(source, List.copyOf(tokens), List.copyOf(lexer.diagnostics()));
      }
    }
    return fail("lexer did not reach EOF, tokens so far: " + tokens);
  }

  // Kinds of all tokens before EOF.
  List<Token.Kind> kinds() {
    return beforeEof().stream().map(Token::kind).toList();
  }

  // Source text of all tokens before EOF.
  List<String> texts() {
    return beforeEof().stream().map(source::text).toList();
  }

  Token eof() {
    return tokens.getLast();
  }

  // The only token before EOF; fails if there are more or none.
  Token single() {
    assertEquals(1, beforeEof().size(), () -> "expected exactly one token, got " + texts());
    return tokens.getFirst();
  }

  String text(Token token) {
    return source.text(token);
  }

  List<String> messages() {
    return diagnostics.stream().map(Diagnostic::message).toList();
  }

  // The source text each diagnostic points at.
  List<String> diagnosticTexts() {
    return diagnostics.stream().map(d -> source.text().substring(d.start(), d.end())).toList();
  }

  private List<Token> beforeEof() {
    return tokens.subList(0, tokens.size() - 1);
  }
}
