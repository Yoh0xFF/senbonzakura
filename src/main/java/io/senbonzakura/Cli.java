package io.senbonzakura;

import io.senbonzakura.ast.Sexpr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.check.Checker;
import io.senbonzakura.interpret.Interpreter;
import io.senbonzakura.interpret.RuntimeError;
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
 * The command line: {@code senbonzakura [--tree] [--run] <file>}. Parses and {@link Checker checks}
 * the file, and prints each problem found to the error stream, with the line it is on and a caret
 * under the part it covers. The checks run only on a file that lexed and parsed cleanly, so one
 * syntax error does not show up again as a list of undefined names. With {@code --tree} it also
 * prints each top-level statement as an {@link Sexpr S-expression}, and with {@code --run} it then
 * runs a file that has no problems, its output going to the output stream.
 *
 * <p>Exits with {@link #OK} when the file has no problems and ran without failing, {@link
 * #PROBLEMS} when it has problems, {@link #USAGE} when the arguments are wrong or the file cannot
 * be read, and {@link #RUNTIME_ERROR} when running it failed.
 */
public final class Cli {

  public static final int OK = 0;
  public static final int PROBLEMS = 1;
  public static final int USAGE = 2;
  public static final int RUNTIME_ERROR = 3;

  private static final String USAGE_TEXT = "usage: senbonzakura [--tree] [--run] <file>";

  private final PrintStream out;
  private final PrintStream err;

  public Cli(PrintStream out, PrintStream err) {
    this.out = out;
    this.err = err;
  }

  public int run(String... args) {
    boolean tree = false;
    boolean execute = false;
    String path = null;
    for (String arg : args) {
      if (arg.equals("--tree")) {
        tree = true;
      } else if (arg.equals("--run")) {
        execute = true;
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
    Checker.Analysis analysis = null;
    if (diagnostics.isEmpty()) {
      analysis = Checker.analyze(statements);
      diagnostics =
          analysis.diagnostics().stream()
              .sorted(Comparator.comparingInt(Diagnostic::start))
              .toList();
    }
    for (var diagnostic : diagnostics) {
      print(source, diagnostic.start(), diagnostic.end(), "error", diagnostic.message());
    }
    if (!diagnostics.isEmpty()) {
      return PROBLEMS;
    }

    if (execute) {
      try {
        new Interpreter(analysis, out).run(statements);
      } catch (RuntimeError e) {
        out.flush();
        if (e.hasLocation()) {
          print(source, e.start(), e.end(), "runtime error", e.getMessage());
        } else {
          err.println(source.path() + ": runtime error: " + e.getMessage());
        }
        return RUNTIME_ERROR;
      }
    }
    return OK;
  }

  /** Prints {@code path:line:column: kind: message}, then the line and a caret under it. */
  private void print(SourceFile source, int start, int end, String kind, String message) {
    var at = source.locate(start);
    err.println(source.path() + ":" + at + ": " + kind + ": " + message);
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

  private static String reason(Exception e) {
    return switch (e) {
      case NoSuchFileException _ -> "no such file";
      case AccessDeniedException _ -> "permission denied";
      case MalformedInputException _ -> "not valid UTF-8";
      default -> e.getMessage();
    };
  }
}
