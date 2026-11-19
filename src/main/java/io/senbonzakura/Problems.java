package io.senbonzakura;

import io.senbonzakura.lexer.SourceFile;
import java.io.PrintStream;

/** Prints problems the way the command line and the REPL both show them. */
final class Problems {

  private Problems() {}

  /**
   * Prints {@code path:line:column: kind: message}, then the line and a caret under the part the
   * problem covers. A source with an empty path, like a REPL entry, starts at the line instead.
   */
  static void print(
      PrintStream err, SourceFile source, int start, int end, String kind, String message) {
    var at = source.locate(start);
    String where = source.path().isEmpty() ? at.toString() : source.path() + ":" + at;
    err.println(where + ": " + kind + ": " + message);
    String line = source.lineText(at.line());
    err.println("    " + line);
    err.println("    " + caret(line, at.column(), end - start));
  }

  /**
   * Pads up to {@code column} and marks {@code length} characters with {@code ^}, at least one and
   * no further than the end of the line. The padding keeps the line's tabs, so the caret lines up
   * under tab-indented code.
   */
  static String caret(String line, int column, int length) {
    int from = column - 1;
    var caret = new StringBuilder();
    for (int i = 0; i < from && i < line.length(); i++) {
      caret.append(line.charAt(i) == '\t' ? '\t' : ' ');
    }
    int width = Math.max(1, Math.min(length, line.length() - from));
    return caret.append("^".repeat(width)).toString();
  }
}
