package io.senbonzakura.check;

import io.senbonzakura.ast.Type.PrimitiveKind;
import java.util.List;

/**
 * The type of a value, as the {@link Checker} works it out. An {@link io.senbonzakura.ast.Type} is
 * a type as written, with a place in the source; this is what it means. It also covers what no
 * written type can say, like a number literal that has not settled on a type yet.
 */
public sealed interface Ty {

  Ty BOOL = new Primitive(PrimitiveKind.BOOL);
  Ty CHAR = new Primitive(PrimitiveKind.CHAR);
  Ty DOUBLE = new Primitive(PrimitiveKind.DOUBLE);
  Ty INT = new Primitive(PrimitiveKind.INT);
  Ty STRING = new Primitive(PrimitiveKind.STRING);

  record Primitive(PrimitiveKind kind) implements Ty {}

  /**
   * What {@code lower..upper} gives: the integers from {@code lower} up to, but not including,
   * {@code upper}. {@code element} is an integer {@link Primitive}, or {@link
   * Special#INTEGER_CONSTANT} while both bounds are.
   */
  record Range(Ty element) implements Ty {}

  /** What calling a function with several return types gives. Only a statement can use it. */
  record Multiple(List<Ty> types) implements Ty {
    public Multiple {
      types = List.copyOf(types);
    }
  }

  enum Special implements Ty {
    /**
     * An integer literal, or arithmetic on only those. It takes the numeric type it goes into, if
     * the literal fits it, and {@code int} when nothing asks for a type.
     */
    INTEGER_CONSTANT,

    /** Like {@link #INTEGER_CONSTANT} for float literals: {@code float} or {@code double}. */
    FLOAT_CONSTANT,

    /** What calling a function without return types gives. */
    NOTHING,

    /** Accepts any value. Only the parameter of the built-in {@code print} has it. */
    ANY,

    /**
     * The type of something whose problem was already reported. It goes into and accepts anything,
     * so the one mistake is not reported again by everything that uses it.
     */
    ERROR
  }
}
