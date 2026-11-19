package io.senbonzakura;

import io.senbonzakura.ast.Stmt;
import io.senbonzakura.check.Checker;
import io.senbonzakura.interpret.Interpreter;
import io.senbonzakura.interpret.RuntimeError;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token;
import io.senbonzakura.lexer.Token.Diagnostic;
import io.senbonzakura.parser.Parser;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.file.Path;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Stream;
import org.jline.terminal.TerminalBuilder;

/**
 * The interactive prompt. It reads one entry at a time, then checks and runs it, keeping the
 * variables and functions of earlier entries. Each entry gets a scope inside the earlier ones, so
 * declaring a name again shadows the old one.
 *
 * <ul>
 *   <li>An entry that ends in an expression without a {@code ;} shows that expression's value.
 *   <li>An entry goes on to the next line, at the {@code ...} prompt, while a parenthesis, bracket,
 *       brace, multi-line string or block comment is still open. A blank line ends it anyway.
 *   <li>An entry with a problem, or that fails while running, is reported and forgotten.
 * </ul>
 *
 * <p>The prompt ends when the input does, as with Ctrl-D. On a terminal, {@link #onTerminal} reads
 * with line editing and history; see {@link ReplInput.Terminal}.
 */
public final class Repl implements AutoCloseable {

  static final String PROMPT = "> ";
  static final String MORE = "... ";
  static final String BANNER =
      "Senbonzakura. End an expression without ';' to see its value. Ctrl-D quits.";

  private final ReplInput input;
  private final PrintStream out;
  private final PrintStream err;
  private final Checker.Session checker = new Checker.Session();
  private final Interpreter.Session interpreter;

  /** A REPL reading plain lines, as from a pipe. */
  public Repl(BufferedReader in, PrintStream out, PrintStream err) {
    this(new ReplInput.Plain(in, out), out, err);
  }

  Repl(ReplInput input, PrintStream out, PrintStream err) {
    this.input = input;
    this.out = out;
    this.err = err;
    this.interpreter = new Interpreter.Session(out);
  }

  /**
   * A REPL reading from the terminal with line editing, keeping its history in {@code
   * ~/.senbonzakura_history}.
   */
  public static Repl onTerminal(PrintStream out, PrintStream err) throws IOException {
    var terminal = TerminalBuilder.builder().system(true).build();
    var history = Path.of(System.getProperty("user.home"), ".senbonzakura_history");
    return new Repl(new ReplInput.Terminal(terminal, history), out, err);
  }

  @Override
  public void close() throws IOException {
    input.close();
  }

  /** Reads and runs entries until the input ends, and returns the exit code. */
  public int run() {
    out.println(BANNER);
    try {
      while (true) {
        String entry = input.next();
        if (entry == null) {
          out.println();
          return Cli.OK;
        }
        if (!entry.isBlank()) {
          evaluate(entry);
        }
      }
    } catch (IOException e) {
      err.println("senbonzakura: cannot read input: " + e.getMessage());
      return Cli.USAGE;
    }
  }

  /** Whether an entry has something open that a later line would close. */
  static boolean unfinished(String text) {
    var lexer = new Lexer(new SourceFile("", text));
    int depth = 0;
    for (Token token = lexer.nextToken();
        token.kind() != Token.Kind.EOF;
        token = lexer.nextToken()) {
      switch (token.kind()) {
        case LEFT_PAREN, LEFT_BRACKET, LEFT_BRACE -> depth++;
        case RIGHT_PAREN, RIGHT_BRACKET, RIGHT_BRACE -> depth--;
        default -> {
          if (token.has(Token.UNTERMINATED) && text.charAt(token.start()) == '`') {
            return true;
          }
        }
      }
    }
    return depth > 0
        || lexer.diagnostics().stream()
            .anyMatch(d -> d.message().equals("unterminated block comment"));
  }

  private void evaluate(String text) {
    var source = new SourceFile("", text);
    var lexer = new Lexer(source);
    var parser = new Parser(lexer);
    List<Stmt> statements = parser.parseFile();
    var problems =
        Stream.concat(lexer.diagnostics().stream(), parser.diagnostics().stream()).toList();

    // At the end of an entry, a missing ';' after an expression asks to see its value.
    boolean showLast =
        !statements.isEmpty()
            && statements.getLast() instanceof Stmt.Expression last
            && last.end() == last.expr().end();
    if (showLast) {
      int end = ((Stmt.Expression) statements.getLast()).expr().end();
      problems =
          problems.stream()
              .filter(
                  d -> !(d.start() == end && d.message().equals("expected ';' after expression")))
              .toList();
    }
    if (report(source, problems)) {
      return;
    }

    var analysis = checker.check(statements);
    if (report(source, analysis.diagnostics())) {
      return;
    }

    try {
      String shown = interpreter.run(statements, analysis, showLast);
      if (shown != null) {
        out.println(shown);
      }
    } catch (RuntimeError e) {
      checker.forgetLast();
      out.flush();
      if (e.hasLocation()) {
        Problems.print(err, source, e.start(), e.end(), "runtime error", e.getMessage());
      } else {
        err.println("runtime error: " + e.getMessage());
      }
    }
  }

  /** Prints the problems in source order, and returns whether there were any. */
  private boolean report(SourceFile source, List<Diagnostic> problems) {
    if (problems.isEmpty()) {
      return false;
    }
    out.flush();
    problems.stream()
        .sorted(Comparator.comparingInt(Diagnostic::start))
        .forEach(d -> Problems.print(err, source, d.start(), d.end(), "error", d.message()));
    return true;
  }
}
