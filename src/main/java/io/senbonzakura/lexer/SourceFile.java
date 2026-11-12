package io.senbonzakura.lexer;

import java.util.Arrays;
import java.util.Objects;

public final class SourceFile {

  private final String path;
  private final String text;
  private final int[] lineStarts;

  public SourceFile(String path, String text) {
    this.path = Objects.requireNonNull(path, "path");
    this.text = Objects.requireNonNull(text, "text");
    this.lineStarts = computeLineStarts(text);
  }

  public String path() {
    return path;
  }

  public String text() {
    return text;
  }

  public int length() {
    return text.length();
  }

  public int lineCount() {
    return lineStarts.length;
  }

  public String text(Token token) {
    return text.substring(token.start(), token.end());
  }

  public LineCol locate(int offset) {
    if (offset < 0 || offset > text.length()) {
      throw new IndexOutOfBoundsException(
          "offset " + offset + " outside 0.." + text.length() + " in " + path);
    }
    int index = Arrays.binarySearch(lineStarts, offset);
    // Not found: binarySearch returns -(insertionPoint) - 1; the line is the one before it.
    int line = index >= 0 ? index : -index - 2;
    return new LineCol(line + 1, offset - lineStarts[line] + 1);
  }

  private static int[] computeLineStarts(String text) {
    int[] starts = new int[16];
    int count = 0;
    starts[count++] = 0;

    int length = text.length();
    for (int i = 0; i < length; i++) {
      char c = text.charAt(i);
      if (c == '\r' && i + 1 < length && text.charAt(i + 1) == '\n') {
        i++; // treat \r\n as a single line break
      } else if (c != '\n' && c != '\r') {
        continue;
      }
      if (count == starts.length) {
        starts = Arrays.copyOf(starts, count * 2);
      }
      starts[count++] = i + 1;
    }
    return Arrays.copyOf(starts, count);
  }

  @Override
  public String toString() {
    return path;
  }

  public record LineCol(int line, int column) {

    public LineCol {
      if (line < 1 || column < 1) {
        throw new IllegalArgumentException("line and column are 1-based: " + line + ":" + column);
      }
    }

    @Override
    public String toString() {
      return line + ":" + column;
    }
  }
}
