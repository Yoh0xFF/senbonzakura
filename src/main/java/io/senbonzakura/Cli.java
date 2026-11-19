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
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PrintStream;
import java.nio.charset.MalformedInputException;
import java.nio.charset.StandardCharsets;
import java.nio.file.AccessDeniedException;
import java.nio.file.Files;
import java.nio.file.InvalidPathException;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Stream;

/**
 * The command line: {@code senbonzakura [--tree] [--run] <file>}, or {@code senbonzakura --repl}
 * for the interactive {@link Repl}. Parses and {@link Checker checks} the file, and prints each
 * problem found to the error stream, with the line it is on and a caret under the part it covers.
 * The checks run only on a file that lexed and parsed cleanly, so one syntax error does not show up
 * again as a list of undefined names. With {@code --tree} it also prints each top-level statement
 * as an {@link Sexpr S-expression}, and with {@code --run} it then runs a file that has no
 * problems, its output going to the output stream.
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

  private static final String USAGE_TEXT =
      "usage: senbonzakura [--tree] [--run] <file>\n       senbonzakura --repl";

  private final InputStream in;
  private final PrintStream out;
  private final PrintStream err;

  public Cli(InputStream in, PrintStream out, PrintStream err) {
    this.in = in;
    this.out = out;
    this.err = err;
  }

  /** A command line with no input, for running files. */
  public Cli(PrintStream out, PrintStream err) {
    this(InputStream.nullInputStream(), out, err);
  }

  public int run(String... args) {
    boolean tree = false;
    boolean execute = false;
    boolean repl = false;
    String path = null;
    for (String arg : args) {
      if (arg.equals("--tree")) {
        tree = true;
      } else if (arg.equals("--run")) {
        execute = true;
      } else if (arg.equals("--repl")) {
        repl = true;
      } else if (arg.startsWith("-") || path != null) {
        err.println(USAGE_TEXT);
        return USAGE;
      } else {
        path = arg;
      }
    }
    if (repl) {
      if (path != null || tree || execute) {
        err.println(USAGE_TEXT);
        return USAGE;
      }
      var reader = new BufferedReader(new InputStreamReader(in, StandardCharsets.UTF_8));
      return new Repl(reader, out, err).run();
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
      Problems.print(
          err, source, diagnostic.start(), diagnostic.end(), "error", diagnostic.message());
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
          Problems.print(err, source, e.start(), e.end(), "runtime error", e.getMessage());
        } else {
          err.println(source.path() + ": runtime error: " + e.getMessage());
        }
        return RUNTIME_ERROR;
      }
    }
    return OK;
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
