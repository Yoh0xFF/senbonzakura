package io.senbonzakura.interpret;

import static io.senbonzakura.check.Ty.Special.FLOAT_CONSTANT;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Type.PrimitiveKind;
import io.senbonzakura.check.Ty;

/**
 * How values are stored at run time, and the operations on them. The static type of each operation
 * comes from the checker, so a value carries no type of its own:
 *
 * <ul>
 *   <li>Integers of every width and {@code char} are {@link Long}s, kept within their type's range:
 *       arithmetic wraps around, as in Go. {@code ulong} and {@code usize} use all 64 bits, and
 *       compare and divide as unsigned.
 *   <li>{@code float} is a {@link Float}, {@code double} a {@link Double}, {@code bool} a {@link
 *       Boolean} and {@code string} a {@link String}.
 *   <li>A range is a {@link RangeValue}.
 *   <li>An integer constant nothing settled, like those in {@code 1 < 2}, is computed as a {@code
 *       long}, and a float constant as a {@code double}.
 * </ul>
 */
final class Values {

  private Values() {}

  /** What {@code lower..upper} evaluates to. */
  record RangeValue(long lower, long upper) {}

  /** The value an integer literal stands for, as its settled type stores it. */
  static Object integerLiteral(long value, Ty type) {
    if (isFloat(type)) {
      return toFloat(value, type);
    }
    return type instanceof Ty.Primitive(var kind) ? wrap(value, kind) : value;
  }

  static Object floatLiteral(double value, Ty type) {
    return isFloat32(type) ? (Object) (float) value : (Object) value;
  }

  /** The value a variable declared without an initializer starts with. */
  static Object zero(Ty type) {
    return switch (((Ty.Primitive) type).kind()) {
      case FLOAT -> 0.0f;
      case DOUBLE -> 0.0;
      case BOOL -> false;
      case STRING -> "";
      default -> 0L;
    };
  }

  /** One, for {@code ++} and {@code --}. */
  static Object one(Ty type) {
    return isFloat(type) ? toFloat(1, type) : 1L;
  }

  /** {@code + - * / %} on two values of {@code type}; {@code +} also joins strings. */
  static Object arithmetic(Expr.BinaryOp op, Ty type, Object left, Object right, Expr at) {
    if (left instanceof String a) {
      return a + right;
    }
    if (isFloat32(type)) {
      float a = (Float) left;
      float b = (Float) right;
      return switch (op) {
        case ADD -> a + b;
        case SUB -> a - b;
        case MUL -> a * b;
        case DIV -> a / b;
        case REM -> a % b;
        default -> throw notArithmetic(op);
      };
    }
    if (isFloat(type)) {
      double a = (Double) left;
      double b = (Double) right;
      return switch (op) {
        case ADD -> a + b;
        case SUB -> a - b;
        case MUL -> a * b;
        case DIV -> a / b;
        case REM -> a % b;
        default -> throw notArithmetic(op);
      };
    }

    long a = (Long) left;
    long b = (Long) right;
    if ((op == Expr.BinaryOp.DIV || op == Expr.BinaryOp.REM) && b == 0) {
      throw new RuntimeError(at.start(), at.end(), "division by zero");
    }
    boolean unsigned = isUnsigned64(type);
    long result =
        switch (op) {
          case ADD -> a + b;
          case SUB -> a - b;
          case MUL -> a * b;
          case DIV -> unsigned ? Long.divideUnsigned(a, b) : a / b;
          case REM -> unsigned ? Long.remainderUnsigned(a, b) : a % b;
          default -> throw notArithmetic(op);
        };
    return type instanceof Ty.Primitive(var kind) ? wrap(result, kind) : result;
  }

  /** {@code == != < <= > >=} on two values of {@code type}. */
  static boolean compare(Expr.BinaryOp op, Ty type, Object left, Object right) {
    if (left instanceof Long a && right instanceof Long b) {
      int order = isUnsigned64(type) ? Long.compareUnsigned(a, b) : Long.compare(a, b);
      return ordered(op, order);
    }
    if (left instanceof Number a && right instanceof Number b) {
      // Primitive comparisons, so NaN is unequal and unordered.
      double x = a.doubleValue();
      double y = b.doubleValue();
      return switch (op) {
        case EQ -> x == y;
        case NE -> x != y;
        case LT -> x < y;
        case LE -> x <= y;
        case GT -> x > y;
        case GE -> x >= y;
        default -> throw notComparison(op);
      };
    }
    if (left instanceof String a && right instanceof String b) {
      return ordered(op, a.compareTo(b));
    }
    return switch (op) {
      case EQ -> left.equals(right);
      case NE -> !left.equals(right);
      default -> throw notComparison(op);
    };
  }

  static Object negate(Ty type, Object value) {
    return switch (value) {
      case Float f -> -f;
      case Double d -> -d;
      case Long l -> type instanceof Ty.Primitive(var kind) ? wrap(-l, kind) : -l;
      default -> throw new IllegalStateException("cannot negate " + value);
    };
  }

  /**
   * {@code value as to}. Integers wrap into a narrower integer type. A float truncates toward zero
   * and saturates at the integer type's range, as in Rust, with NaN becoming zero. An integer that
   * is not a code point fails as a {@code char}.
   */
  static Object cast(Object value, Ty from, Ty to, Expr at) {
    var target = (Ty.Primitive) to;
    PrimitiveKind kind = target.kind();
    if (value instanceof Long integer) {
      if (kind == PrimitiveKind.CHAR) {
        boolean valid =
            integer >= 0
                && integer <= Character.MAX_CODE_POINT
                && !(integer >= Character.MIN_SURROGATE && integer <= Character.MAX_SURROGATE);
        if (!valid) {
          throw new RuntimeError(at.start(), at.end(), integer + " is not a valid char");
        }
        return integer;
      }
      if (isFloat(to)) {
        double d = isUnsigned64(from) ? unsignedToDouble(integer) : (double) integer;
        return isFloat32(to) ? (Object) (float) d : (Object) d;
      }
      return wrap(integer, kind);
    }

    double d = ((Number) value).doubleValue();
    if (kind == PrimitiveKind.FLOAT) {
      return (float) d;
    }
    if (kind == PrimitiveKind.DOUBLE) {
      return d;
    }
    if (Double.isNaN(d)) {
      return 0L;
    }
    if (kind == PrimitiveKind.ULONG || kind == PrimitiveKind.USIZE) {
      if (d <= 0) {
        return 0L;
      }
      if (d >= 0x1p64) {
        return -1L; // all 64 bits set: the largest ulong
      }
      return d < 0x1p63 ? (long) d : (long) (d - 0x1p63) + Long.MIN_VALUE;
    }
    long truncated = (long) d; // Java saturates at the long range
    return Math.clamp(truncated, min(kind), max(kind));
  }

  /** How {@code print} shows a value of {@code type}. */
  static String format(Ty type, Object value) {
    return switch (type) {
      case Ty.Range range ->
          format(range.element(), ((RangeValue) value).lower())
              + ".."
              + format(range.element(), ((RangeValue) value).upper());
      case Ty.Primitive(var kind) when kind == PrimitiveKind.CHAR ->
          Character.toString(Math.toIntExact((Long) value));
      default ->
          value instanceof Long l && isUnsigned64(type)
              ? Long.toUnsignedString(l)
              : String.valueOf(value);
    };
  }

  /** Keeps {@code value} within the range of {@code kind}, wrapping around as two's complement. */
  static long wrap(long value, PrimitiveKind kind) {
    return switch (kind) {
      case BYTE -> (byte) value;
      case SHORT -> (short) value;
      case INT -> (int) value;
      case UBYTE -> value & 0xFFL;
      case USHORT -> value & 0xFFFFL;
      case UINT -> value & 0xFFFF_FFFFL;
      default -> value;
    };
  }

  private static long min(PrimitiveKind kind) {
    return switch (kind) {
      case BYTE -> Byte.MIN_VALUE;
      case SHORT -> Short.MIN_VALUE;
      case INT -> Integer.MIN_VALUE;
      case SIZE, LONG -> Long.MIN_VALUE;
      default -> 0;
    };
  }

  private static long max(PrimitiveKind kind) {
    return switch (kind) {
      case BYTE -> Byte.MAX_VALUE;
      case SHORT -> Short.MAX_VALUE;
      case INT -> Integer.MAX_VALUE;
      case UBYTE -> 0xFFL;
      case USHORT -> 0xFFFFL;
      case UINT -> 0xFFFF_FFFFL;
      default -> Long.MAX_VALUE;
    };
  }

  private static boolean ordered(Expr.BinaryOp op, int order) {
    return switch (op) {
      case EQ -> order == 0;
      case NE -> order != 0;
      case LT -> order < 0;
      case LE -> order <= 0;
      case GT -> order > 0;
      case GE -> order >= 0;
      default -> throw notComparison(op);
    };
  }

  private static double unsignedToDouble(long value) {
    // Halve, keeping the low bit so rounding stays correct, convert, then double.
    return value >= 0 ? value : ((value >>> 1) | (value & 1)) * 2.0;
  }

  private static Object toFloat(long value, Ty type) {
    return isFloat32(type) ? (Object) (float) value : (Object) (double) value;
  }

  private static boolean isFloat(Ty type) {
    return type == FLOAT_CONSTANT
        || (type instanceof Ty.Primitive(var kind)
            && (kind == PrimitiveKind.FLOAT || kind == PrimitiveKind.DOUBLE));
  }

  private static boolean isFloat32(Ty type) {
    return type instanceof Ty.Primitive(var kind) && kind == PrimitiveKind.FLOAT;
  }

  private static boolean isUnsigned64(Ty type) {
    return type instanceof Ty.Primitive(var kind)
        && (kind == PrimitiveKind.ULONG || kind == PrimitiveKind.USIZE);
  }

  private static IllegalStateException notArithmetic(Expr.BinaryOp op) {
    return new IllegalStateException("not arithmetic: " + op);
  }

  private static IllegalStateException notComparison(Expr.BinaryOp op) {
    return new IllegalStateException("not a comparison: " + op);
  }
}
