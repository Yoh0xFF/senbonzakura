package io.senbonzakura;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.io.BufferedReader;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.PrintStream;
import java.io.StringReader;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import org.jline.reader.EOFError;
import org.jline.reader.Parser.ParseContext;
import org.jline.terminal.TerminalBuilder;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

class ReplInputTest {

  private final ReplInput.EntryParser parser = new ReplInput.EntryParser();

  @ParameterizedTest
  @ValueSource(strings = {"fn f() {", "f(1,", "fn f() {\n  return 1;", "`multi\nline", "/* open"})
  void enterOnAnUnfinishedEntryStartsANewLine(String entry) {
    assertThrows(
        EOFError.class, () -> parser.parse(entry, entry.length(), ParseContext.ACCEPT_LINE));
  }

  @ParameterizedTest
  @ValueSource(
      strings = {
        "1 + 2",
        "fn f() {\n  return 1;\n}",
        "// don't stop at an apostrophe",
        "var s = \"it's\"; var c = '\\'';",
        "print(`a \\ b`)",
        "{\n",
        "fn f() {\n   ",
      })
  void enterOnAFinishedOrBlankEndedEntrySubmitsIt(String entry) {
    parser.parse(entry, entry.length(), ParseContext.ACCEPT_LINE);
  }

  @Test
  void unfinishedEntryIsNotHeldOpenWhileCompleting() {
    parser.parse("fn f() {", 8, ParseContext.COMPLETE);
  }

  @Test
  void terminalInputReadsMultiLineEntriesWhole() throws IOException {
    var entries = entries("fn f(): int {\n  return 5;\n}\nf()\n!done && x\n");

    assertEquals(List.of("fn f(): int {\n  return 5;\n}", "f()", "!done && x"), entries);
  }

  @Test
  void terminalInputSubmitsAnUnfinishedEntryOnABlankLine() throws IOException {
    assertEquals(List.of("{\n", "1"), entries("{\n\n1\n"));
  }

  @Test
  void plainInputReadsLinesUntilTheEntryIsFinished() throws IOException {
    var out = new ByteArrayOutputStream();
    var input =
        new ReplInput.Plain(
            new BufferedReader(new StringReader("fn f() {\n}\n1\n")),
            new PrintStream(out, true, StandardCharsets.UTF_8));

    assertEquals("fn f() {\n}\n", input.next());
    assertEquals("1\n", input.next());
    assertNull(input.next());
    assertEquals("> ... > > ", out.toString(StandardCharsets.UTF_8));
  }

  // Types the keys into a virtual terminal and returns the entries the reader makes of them.
  private static List<String> entries(String keys) throws IOException {
    var terminal =
        TerminalBuilder.builder()
            .system(false)
            .streams(
                new ByteArrayInputStream(keys.getBytes(StandardCharsets.UTF_8)),
                new ByteArrayOutputStream())
            .type("dumb")
            .build();
    var entries = new ArrayList<String>();
    try (var input = new ReplInput.Terminal(terminal, null)) {
      for (String entry = input.next(); entry != null; entry = input.next()) {
        entries.add(entry);
      }
    }
    return entries;
  }
}
