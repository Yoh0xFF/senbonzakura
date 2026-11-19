package io.senbonzakura.interpret;

/**
 * A problem found while running a program that checked cleanly, like a division by zero. Covers the
 * source offsets {@code start} up to {@code end} of the code that failed, or neither when no place
 * can be named, as for a stack overflow.
 */
public final class RuntimeError extends RuntimeException {

  private final int start;
  private final int end;

  RuntimeError(int start, int end, String message) {
    super(message, null, false, false);
    this.start = start;
    this.end = end;
  }

  RuntimeError(String message) {
    this(-1, -1, message);
  }

  /** Whether the error points at a place in the source. */
  public boolean hasLocation() {
    return start >= 0;
  }

  public int start() {
    return start;
  }

  public int end() {
    return end;
  }
}
