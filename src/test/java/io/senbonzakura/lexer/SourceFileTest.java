package io.senbonzakura.lexer;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import io.senbonzakura.lexer.SourceFile.LineCol;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class SourceFileTest {

  @Test
  void emptyTextHasOneLine() {
    var file = new SourceFile("test", "");

    assertEquals(1, file.lineCount());
    assertEquals(new LineCol(1, 1), file.locate(0));
  }

  @ParameterizedTest
  @CsvSource({
    "0, 1, 1", // 'a'
    "1, 1, 2", // 'b'
    "2, 1, 3", // '\n'
    "3, 2, 1", // 'c'
    "4, 2, 2", // '\r' of \r\n
    "5, 2, 3", // '\n' of \r\n
    "6, 3, 1", // 'd'
    "7, 3, 2", // lone '\r'
    "8, 4, 1", // 'e'
    "9, 4, 2", // end of text
  })
  void locateHandlesEveryLineBreakStyle(int offset, int line, int column) {
    var file = new SourceFile("test", "ab\nc\r\nd\re");

    assertEquals(new LineCol(line, column), file.locate(offset));
  }

  @Test
  void crlfCountsAsOneLineBreak() {
    assertEquals(3, new SourceFile("test", "a\r\nb\r\nc").lineCount());
  }

  @Test
  void trailingLineBreakStartsAnEmptyLastLine() {
    var file = new SourceFile("test", "a\n");

    assertEquals(2, file.lineCount());
    assertEquals(new LineCol(2, 1), file.locate(2));
  }

  @Test
  void columnsCountUtf16Units() {
    var file = new SourceFile("test", "😀x");

    assertEquals(new LineCol(1, 3), file.locate(2));
  }

  @Test
  void locateRejectsOffsetsOutsideTheText() {
    var file = new SourceFile("test", "abc");

    assertThrows(IndexOutOfBoundsException.class, () -> file.locate(-1));
    assertThrows(IndexOutOfBoundsException.class, () -> file.locate(4));
  }

  @Test
  void textOfToken() {
    var file = new SourceFile("test", "let x = 42");

    assertEquals("42", file.text(new Token(Token.Kind.INT_LITERAL, 8, 10, 0)));
  }

  @Test
  void accessors() {
    var file = new SourceFile("main.sz", "abc");

    assertEquals("main.sz", file.path());
    assertEquals("abc", file.text());
    assertEquals(3, file.length());
    assertEquals("main.sz", file.toString());
  }

  @Test
  void rejectsNulls() {
    assertThrows(NullPointerException.class, () -> new SourceFile(null, ""));
    assertThrows(NullPointerException.class, () -> new SourceFile("test", null));
  }

  @Test
  void lineColIsOneBased() {
    assertEquals("3:7", new LineCol(3, 7).toString());
    assertThrows(IllegalArgumentException.class, () -> new LineCol(0, 1));
    assertThrows(IllegalArgumentException.class, () -> new LineCol(1, 0));
  }
}
