package io.senbonzakura.lexer;

import java.util.Arrays;
import java.util.Map;
import java.util.stream.Collectors;

/**
 * Scans identifiers and keywords. An identifier starts with a letter or {@code _} and continues
 * with letters, digits and {@code _}, from any script. A keyword is an identifier whose text is the
 * {@link Token.Kind#spelling() spelling} of a keyword kind; matching is case-sensitive.
 */
final class ScanIdentifier {

  // Operators will have spellings too, but they never start with a letter.
  private static final Map<String, Token.Kind> KEYWORDS =
      Arrays.stream(Token.Kind.values())
          .filter(kind -> kind.spelling() != null && Character.isLetter(kind.spelling().charAt(0)))
          .collect(Collectors.toUnmodifiableMap(Token.Kind::spelling, kind -> kind));

  private ScanIdentifier() {}

  /**
   * Scans the word at the cursor, which must be {@link Cursor#atIdentifierStart() on its start}.
   */
  static Token identifierOrKeyword(Cursor c) {
    int start = c.pos();
    c.skipIdentifierChars();
    var kind = KEYWORDS.getOrDefault(c.text(start, c.pos()), Token.Kind.IDENTIFIER);
    return c.token(kind, start, 0);
  }
}
