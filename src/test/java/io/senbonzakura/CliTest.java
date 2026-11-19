package io.senbonzakura;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class CliTest {

  @TempDir Path dir;

  private final ByteArrayOutputStream out = new ByteArrayOutputStream();
  private final ByteArrayOutputStream err = new ByteArrayOutputStream();
  private final Cli cli =
      new Cli(
          new PrintStream(out, true, StandardCharsets.UTF_8),
          new PrintStream(err, true, StandardCharsets.UTF_8));

  @Test
  void cleanFilePrintsNothingAndSucceeds() throws IOException {
    var file = write("ok.sz", "var x: int = 1;\nfn f(a: int): int { return a; }\n");

    assertEquals(Cli.OK, cli.run(file));
    assertEquals("", out());
    assertEquals("", err());
  }

  @Test
  void treeOptionPrintsEachTopLevelStatement() throws IOException {
    var file = write("ok.sz", "var x = 1; // one\nfn f() { x++; }\n");

    assertEquals(Cli.OK, cli.run("--tree", file));
    assertEquals("(var x 1)\n(fn f () {(++ x);})\n", out());
  }

  @Test
  void problemIsPrintedWithItsLineAndACaret() throws IOException {
    var file = write("bad.sz", "var x = 1;\nvar y = x +;\n");

    assertEquals(Cli.PROBLEMS, cli.run(file));
    assertEquals(
        file
            + ":2:12: error: expected expression, found ';'\n"
            + "    var y = x +;\n"
            + "               ^\n",
        err());
  }

  @Test
  void caretCoversTheWholeProblem() throws IOException {
    var file = write("bad.sz", "f(1, x: 2);");

    assertEquals(Cli.PROBLEMS, cli.run(file));
    assertEquals(
        file
            + ":1:6: error: positional and named arguments cannot be mixed\n"
            + "    f(1, x: 2);\n"
            + "         ^^^^\n",
        err());
  }

  @Test
  void lexerAndParserProblemsArePrintedInSourceOrder() throws IOException {
    var file = write("bad.sz", "var a = ;\nvar b = '\\q';\nvar c = ;\n");

    assertEquals(Cli.PROBLEMS, cli.run(file));
    assertEquals(
        List.of(
            file + ":1:9: error: expected expression, found ';'",
            file + ":2:10: error: unknown escape sequence",
            file + ":3:9: error: expected expression, found ';'"),
        err().lines().filter(l -> l.startsWith(file)).toList());
  }

  @Test
  void treeIsStillPrintedWhenThereAreProblems() throws IOException {
    var file = write("bad.sz", "var x = ;");

    assertEquals(Cli.PROBLEMS, cli.run("--tree", file));
    assertEquals("(var x <error>)\n", out());
  }

  @Test
  void missingFileIsAUsageError() {
    var file = dir.resolve("missing.sz").toString();

    assertEquals(Cli.USAGE, cli.run(file));
    assertEquals("senbonzakura: cannot read " + file + ": no such file\n", err());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {"''", "'--tree'", "'--bogus a.sz'", "'a.sz b.sz'"})
  void badArgumentsPrintUsage(String args) {
    String[] argv = args.isEmpty() ? new String[0] : args.split(" ");

    assertEquals(Cli.USAGE, cli.run(argv));
    assertEquals("usage: senbonzakura [--tree] <file>\n", err());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "'abc'      => 2 => 1 => ' ^'",
        "'abc'      => 1 => 3 => '^^^'",
        "'abc'      => 2 => 9 => ' ^^'",
        "'abc'      => 4 => 0 => '   ^'",
        "'\tx = ;'  => 6 => 1 => '\t    ^'",
      })
  void caretLinesUpUnderTheLine(String line, int column, int length, String caret) {
    assertEquals(caret, Cli.caret(line, column, length));
  }

  private String write(String name, String text) throws IOException {
    var file = dir.resolve(name);
    Files.writeString(file, text);
    return file.toString();
  }

  private String out() {
    return out.toString(StandardCharsets.UTF_8).replace(System.lineSeparator(), "\n");
  }

  private String err() {
    return err.toString(StandardCharsets.UTF_8).replace(System.lineSeparator(), "\n");
  }
}
