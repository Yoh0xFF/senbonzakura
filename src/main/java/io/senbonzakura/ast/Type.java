package io.senbonzakura.ast;

import java.util.Objects;

/**
 * A type written in the source, like the {@code int} in {@code var i: int}. Like {@link Expr},
 * every node covers the source offsets {@code start} up to (not including) {@code end}.
 */
public sealed interface Type {

  int start();

  int end();

  record Primitive(PrimitiveKind kind, int start, int end) implements Type {
    public Primitive {
      Objects.requireNonNull(kind, "kind");
      NodeRange.check(start, end);
    }
  }

  /**
   * Stands in for a type that could not be parsed. A diagnostic has already been reported, so later
   * passes skip it without reporting again.
   */
  record Error(int start, int end) implements Type {
    public Error {
      NodeRange.check(start, end);
    }
  }

  enum PrimitiveKind {
    // Signed integers
    BYTE("byte"),
    SHORT("short"),
    INT("int"),
    SIZE("size"),
    LONG("long"),

    // Unsigned integers
    UBYTE("ubyte"),
    USHORT("ushort"),
    UINT("uint"),
    USIZE("usize"),
    ULONG("ulong"),

    // Floating point
    FLOAT("float"),
    DOUBLE("double"),

    CHAR("char"),
    BOOL("bool"),
    STRING("string");

    private final String spelling;

    PrimitiveKind(String spelling) {
      this.spelling = spelling;
    }

    public String spelling() {
      return spelling;
    }
  }
}
