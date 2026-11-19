package io.senbonzakura;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.file.Path;
import org.jline.reader.EOFError;
import org.jline.reader.EndOfFileException;
import org.jline.reader.LineReader;
import org.jline.reader.LineReaderBuilder;
import org.jline.reader.ParsedLine;
import org.jline.reader.UserInterruptException;
import org.jline.reader.impl.DefaultParser;

/** Where the {@link Repl} reads its entries from: a plain stream, or a terminal through JLine. */
sealed interface ReplInput extends AutoCloseable {

  /**
   * Reads the next entry, which may span lines, or returns {@code null} when the input has ended.
   * An entry the user abandoned comes back blank.
   */
  String next() throws IOException;

  @Override
  default void close() throws IOException {}

  /**
   * Reads lines from a stream, printing the prompts itself, and joins the lines of an entry that is
   * still {@link Repl#unfinished}. A blank line ends an entry anyway.
   */
  final class Plain implements ReplInput {

    private final BufferedReader in;
    private final PrintStream out;

    Plain(BufferedReader in, PrintStream out) {
      this.in = in;
      this.out = out;
    }

    @Override
    public String next() throws IOException {
      out.print(Repl.PROMPT);
      out.flush();
      var text = new StringBuilder();
      while (true) {
        String line = in.readLine();
        if (line == null) {
          return text.isEmpty() ? null : text.toString();
        }
        if (!text.isEmpty() && line.isBlank()) {
          return text.toString();
        }
        text.append(line).append('\n');
        if (!Repl.unfinished(text.toString())) {
          return text.toString();
        }
        out.print(Repl.MORE);
        out.flush();
      }
    }
  }

  /**
   * Reads from a terminal with JLine: the arrow keys move through the entry, which can be edited
   * anywhere, and up and down recall earlier entries. Enter on an unfinished entry starts a new
   * line of it instead of submitting it, so a whole function stays one entry to edit and recall.
   * Ctrl-C abandons the entry, and Ctrl-D on an empty one ends the input.
   */
  final class Terminal implements ReplInput {

    private final org.jline.terminal.Terminal terminal;
    private final LineReader reader;

    /**
     * @param history the file earlier entries are kept in across sessions, or {@code null} to keep
     *     them only while this session lasts
     */
    Terminal(org.jline.terminal.Terminal terminal, Path history) {
      this.terminal = terminal;
      var builder =
          LineReaderBuilder.builder()
              .terminal(terminal)
              .appName("senbonzakura")
              .parser(new EntryParser())
              .variable(LineReader.SECONDARY_PROMPT_PATTERN, Repl.MORE)
              // `!` is the not operator, not a history event to expand.
              .option(LineReader.Option.DISABLE_EVENT_EXPANSION, true);
      if (history != null) {
        builder.variable(LineReader.HISTORY_FILE, history);
      }
      this.reader = builder.build();
    }

    @Override
    public String next() {
      try {
        return reader.readLine(Repl.PROMPT);
      } catch (UserInterruptException e) {
        return "";
      } catch (EndOfFileException e) {
        return null;
      }
    }

    @Override
    public void close() throws IOException {
      terminal.close();
    }
  }

  /**
   * Tells JLine whether Enter submits the entry. It does unless the entry is {@link
   * Repl#unfinished}, and a blank last line submits it anyway. Quotes and backslashes mean nothing
   * special here: the lexer reads the entry, and an apostrophe in a comment must not hold it open.
   */
  final class EntryParser extends DefaultParser {

    EntryParser() {
      setQuoteChars(new char[0]);
      setEscapeChars(new char[0]);
    }

    @Override
    public ParsedLine parse(String line, int cursor, ParseContext context) {
      if (context == ParseContext.ACCEPT_LINE && Repl.unfinished(line) && !endsBlank(line)) {
        throw new EOFError(-1, -1, "the entry is not finished");
      }
      return super.parse(line, cursor, context);
    }

    private static boolean endsBlank(String line) {
      int lastBreak = line.lastIndexOf('\n');
      return lastBreak >= 0 && line.substring(lastBreak + 1).isBlank();
    }
  }
}
