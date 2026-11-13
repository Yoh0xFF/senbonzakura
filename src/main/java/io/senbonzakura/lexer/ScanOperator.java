package io.senbonzakura.lexer;

import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.stream.Collectors;

/**
 * Scans operators and punctuation by longest match, so {@code <=} is one token rather than {@code
 * <} then {@code =}. The candidates are the {@link Token.Kind#isOperator() operator} kinds.
 */
final class ScanOperator {

  // Grouped by first character and sorted longest first, so the first spelling that matches is the
  // longest one.
  private static final Map<Character, List<Token.Kind>> BY_FIRST_CHAR =
      Map.copyOf(
          Arrays.stream(Token.Kind.values())
              .filter(Token.Kind::isOperator)
              .sorted(
                  Comparator.comparingInt((Token.Kind kind) -> kind.spelling().length()).reversed())
              .collect(
                  Collectors.groupingBy(
                      kind -> kind.spelling().charAt(0), Collectors.toUnmodifiableList())));

  private ScanOperator() {}

  /**
   * Scans the operator at the cursor, or returns empty and consumes nothing if none starts there.
   */
  static Optional<Token> operator(Cursor c) {
    var candidates = BY_FIRST_CHAR.getOrDefault((char) c.peek(), List.of());
    for (var kind : candidates) {
      if (c.lookingAt(kind.spelling())) {
        int start = c.pos();
        c.bump(kind.spelling().length());
        return Optional.of(c.token(kind, start, 0));
      }
    }
    return Optional.empty();
  }
}
