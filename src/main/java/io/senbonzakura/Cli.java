package io.senbonzakura;

import io.senbonzakura.ast.Sexpr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token.Diagnostic;
import io.senbonzakura.parser.Parser;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.charset.MalformedInputException;
import java.nio.file.AccessDeniedException;
import java.nio.file.Files;
import java.nio.file.InvalidPathException;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Stream;

/**
 * The command line: {@code senbonzakura [--tree] <file>}. Parses the file and prints each problem
 * found to the error stream, with the line it is on and a caret under the part it covers. With
 * {@code --tree} it also prints each top-level statement as an {@link Sexpr S-expression}.
 *
 * <p>Exits with {@link #OK} when the file has no problems, {@link #PROBLEMS} when it has some, and
 * {@link #USAGE} when the arguments are wrong or the file cannot be read.
 */
public final class Cli {

  public static final int OK = 0;
  public static final int PROBLEMS = 1;
  public static final int USAGE = 2;

  private static final String USAGE_TEXT = "usage: senbonzakura [--tree] <file>";

  private final PrintStream out;
  private final PrintStream err;

  public Cli(PrintStream out, PrintStream err) {
    this.out = out;
    this.err = err;
  }

  public int run(String... args) {
    boolean tree = false;
    String path = null;
    for (String arg : args) {
      if (arg.equals("--tree")) {
        tree = true;
      } else if (arg.startsWith("-") || path != null) {
        err.println(USAGE_TEXT);
        return USAGE;
      } else {
        path = arg;
      }
    }
    if (path == null) {
      err.println(USAGE_TEXT);
      return USAGE;
    }

    String text;
    try {
      text = Files.readString(Path.of(path));
    } catch (IOException | InvalidPathException e) {
      err.println("senbonzakura: cannot read " + path + ": " + reason(e));
      return USAGE;
    }

    var source = new SourceFile(path, text);
    var lexer = new Lexer(source);
    var parser = new Parser(lexer);
    List<Stmt> statements = parser.parseFile();

    if (tree) {
      for (Stmt stmt : statements) {
        out.println(Sexpr.print(stmt));
      }
    }

    // The lexer's problems come in as the parser pulls tokens, so the two lists interleave.
    var diagnostics =
        Stream.concat(lexer.diagnostics().stream(), parser.diagnostics().stream())
            .sorted(Comparator.comparingInt(Diagnostic::start))
            .toList();
    for (var diagnostic : diagnostics) {
      print(source, diagnostic);
    }
    return diagnostics.isEmpty() ? OK : PROBLEMS;
  }

  /** Prints {@code path:line:column: error: message}, then the line and a caret under it. */
  private void print(SourceFile source, Diagnostic diagnostic) {
    var at = source.locate(diagnostic.start());
    err.println(source.path() + ":" + at + ": error: " + diagnostic.message());
    String line = source.lineText(at.line());
    err.println("    " + line);
    err.println("    " + caret(line, at.column(), diagnostic.end() - diagnostic.start()));
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

  private static String reason(Exception e) {
    return switch (e) {
      case NoSuchFileException _ -> "no such file";
      case AccessDeniedException _ -> "permission denied";
      case MalformedInputException _ -> "not valid UTF-8";
      default -> e.getMessage();
    };
  }
}
