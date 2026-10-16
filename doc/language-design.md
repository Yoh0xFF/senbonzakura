# Language Design

_Decided: 2026-10-17._

## Goal

Senbonzakura is a **learning vehicle**, not a language meant for real programs. Every feature has to
earn its place by teaching a compiler or LLVM concept. The plan is to get a thin vertical slice
compiling to native code first, then add one feature at a time.

## Pipeline

```
source → lexer → parser → AST (with spans)
       → checker → typed AST
       → codegen → textual .ll
       → clang (+ embedded C runtime) → native binary
```

- The compiler stays in Rust and emits textual LLVM IR (see `rust-vs-cpp-for-llvm.md`). `inkwell`
  comes later, if ever.
- Spans live on the untyped AST (`Spanned<T>`). The typed AST can drop them at first, except where
  codegen needs a location for a runtime panic.
- The checker turns `ast::Expression` into a separate `typed::Expr` tree. Every node carries its
  resolved type and every identifier its resolved binding, so codegen never has to `unwrap()`
  something that "should have been checked".
- The checker **collects all errors**. An expression that failed to check gets the `Error` type,
  which is compatible with everything, so one mistake doesn't cascade. Codegen runs only when there
  are zero errors.
- CLI: `senbonzakura build foo.sb -o foo`, plus `--emit=tokens|ast|ll` to stop early.

## Types

| Type | LLVM | Notes |
|---|---|---|
| `int` | `i64` | Literals without `.` or `e`. Must fit in i64, or it's a compile error. |
| `float` | `double` | Literals with `.` or `e`. |
| `boolean` | `i1` | |
| `string` | `{ ptr, i64 }` by value | Immutable. |
| `[T]` | `ptr` to `{ i64, [0 x T] }` | Reference type, nullable. |
| class `C` | `ptr` to `{ ptr vtable, fields… }` | Reference type, nullable. |
| `void` | `void` | Return type only. |

`number` is removed. Generics (`Name[T]`) still parse, but the checker rejects them as "not
supported yet".

### Checking rules

- **No implicit conversions anywhere.** Converting between `int` and `float` is explicit, with
  `int(x)` / `float(x)` or `x as float`.
- `if`, `while`, `do-while`, `for` conditions, `!`, `&&` and `||` accept **only `boolean`**. There is
  no truthiness.
- Binary operators need operands of identical types. `==` and `!=` likewise, with exceptions listed
  under each type below.
- Unary `-` works on `int` and `float`. `!` works only on `boolean`. Unary `+` is removed.
- `%` works only on `int`.

### Annotations and inference

- Parameters and return types are always annotated. A missing return type means `void`.
- `let x = 5;` infers the type from the initializer. `let x: int;` without an initializer must be
  annotated. `let x = nil;` is an error, since there's nothing to infer from.
- An empty array literal `[]` needs an expected type from context (`let a: [int] = [];`). This is the
  only place the checker uses an expected type.

### Scopes and initialization

- Every `{}` block opens a scope, and the `for` initializer gets its own scope.
- Shadowing in an inner scope is allowed. Redeclaring in the same scope is an error.
- An uninitialized `let` gets a **zero value**: `0`, `0.0`, `false`, `""`, or `nil` for class and
  array types. Class fields start at zero values too.

### Functions

- Top-level only. No nested functions, no closures, no first-class function values, no overloading.
- Functions can be called before they're declared. The checker first collects every signature, then
  checks bodies.
- A function with a non-void return type must **definitely return**. Its body has to end in a
  statement that always returns: a `return`, a block whose last statement always returns, or an
  `if`/`else` where both branches always return. Loops never count, so `while (true) { return 1; }`
  needs a trailing `return`. Code after a `return` is allowed and not reported.

### Program structure

C style. The top level holds only declarations (functions and classes), with **no global variables**,
not even `const`. Every program needs `def main(): int`.

## Semantics

### Statements and expressions

- Assignment, including `+=`, `-=`, `*=`, `/=`, is an **expression** that evaluates to the assigned
  value, so `a = b = 3` works. The target must be an identifier, a field (`this.x`, `p.x`) or an
  index (`a[i]`).
- A compound assignment evaluates its target's subexpressions **once**: `a[i()] += 1` calls `i()`
  once. Codegen computes the address once, then loads, operates and stores.
- `&&` and `||` short-circuit.
- `break` and `continue` have no labels and are an error outside a loop. `continue` in a `for` jumps
  to the increment, not the condition.

### Integer and float arithmetic

- `+`, `-` and `*` on `int` **wrap** (two's complement). They're emitted without `nsw`.
- `/` and `%` on `int` check for a zero divisor at runtime and panic. `INT_MIN / -1` wraps to
  `INT_MIN`. `%` is C-style truncated remainder (`srem`), so `-7 % 3 == -1`.
- `float` follows IEEE: `1.0 / 0.0` is `inf`, with no check. Comparisons use ordered `fcmp`
  (`oeq`, `olt`, …), except `!=`, which uses `une`, so every comparison with NaN except `!=` is
  false.
- `-9223372036854775808` can't be written as a literal, because it parses as `-(9223372036854775808)`.
  Write `-9223372036854775807 - 1`, as in C.

### Strings

- Immutable, represented as `{ ptr, i64 len }` and passed by value. Literals are `private constant`
  globals with no allocation and no NUL terminator.
- `+` and `+=` concatenate, allocating a new string. `==` and `!=` compare **content**. `len(s)` is
  an intrinsic.
- No `<`/`>` on strings, no indexing, no `char` type.
- Strings are never `nil`. Their zero value is `""`.

### Arrays

- Reference types: `let b = a;` aliases the same array.
- Created with a literal, `[1, 2, 3]`, whose element type comes from the first element (every
  element must match it exactly), or with `new [int](n)`, which gives `n` zero-valued elements. A
  negative `n` panics.
- Indexing is **bounds-checked** and panics when out of range. `len(a)` is an intrinsic.
- Fixed length after creation, with no `push`.
- `==` on arrays is a type error.

### Classes

```
class Point {
    let x: int;
    let y: int;

    def constructor(x: int, y: int) {
        this.x = x;
        this.y = y;
    }

    def norm2(): int {
        return this.x * this.x + this.y * this.y;
    }
}

def main(): int {
    let p = new Point(1, 2);
    println(p.norm2());
    return 0;
}
```

- Reference types. `new` allocates, and `==` compares **identity**.
- Fields are declared with `let name: Type;`, with no initializers. They start at zero values and the
  constructor sets them.
- A class body contains only fields and methods.
- `constructor` is optional. Without one, `new Point()` takes no arguments.
- Fields are always accessed through `this.x`, with no implicit `this`. Everything is public.
- Methods take `this` as their first parameter.
- The layout is `{ vtable_ptr, parent fields…, own fields… }`. The vtable pointer exists from the
  first class milestone, even before inheritance, so adding inheritance doesn't change the layout.

### Inheritance

Built in the milestone after classes:

- Single inheritance with `extends`. **Every method is virtual**, with no `virtual` or `override`
  keywords. An override must have exactly the same parameter and return types.
- Upcasts from subclass to superclass are implicit. There are no downcasts or `instanceof`.
- `super.method(...)` is a static call to the parent's implementation. A subclass constructor must
  call `super.constructor(...)` explicitly when the parent's constructor takes arguments.

### `nil`

- `nil` has a special type that converts to any class or array type, Java style. Every class and
  array reference is nullable. `x == nil` is allowed.
- Field access, method calls and indexing through `nil` hit a runtime null check and panic.
- Optional types (`T?` with narrowing after `if (x != nil)`) are a possible later lesson.

## Runtime and ABI

### Symbols

- User symbols are mangled: function `foo` is `@sb.foo`, method `Point.norm2` is
  `@sb.Point.norm2`. They can't collide with libc.
- The compiler emits a real `@main(): i32` that calls `@sb.main` and truncates its `i64` result into
  the process exit code.
- Runtime functions use the `sb_` prefix: `sb_alloc`, `sb_panic`, `sb_str_concat`, …

### Runtime library

- A small C file, `runtime/sb_runtime.c`, embedded in the compiler with `include_str!`.
- `build` writes the `.ll`, then runs `clang foo.ll sb_runtime.c -o foo`. Only `clang` is needed,
  with no LLVM install.
- The `.ll` has no target triple or datalayout, so clang uses the host defaults.

### Memory

All allocation goes through `sb_alloc(size)`. In v1 it's plain `malloc` and nothing is freed.
Switching to the Boehm GC later means changing only that function.

### Panics

A panic prints `panic: <message> at <line>:<col>` to stderr and exits with code **101**. Codegen
passes the source location to `sb_panic` as constants. Panics cover division by zero, out-of-bounds
indexes, `nil` access and negative array sizes.

### Printing

`print(x)` and `println(x)` are compiler intrinsics, not functions, since there's no overloading.
Each takes exactly one primitive argument (`int`, `float`, `boolean`, `string`):

- `float` prints with `%g` (`0.1`, `3.14159`, `1e+20`).
- `boolean` prints `true` / `false`.
- `string` prints with `printf("%.*s", len, ptr)`.
- Only `println` adds a newline.

### Codegen conventions

- Locals use `alloca`, `load` and `store`, and `mem2reg` turns them into SSA. No phi nodes by hand.
- `&&` and `||` use branches plus an `alloca` for the result.
- A runtime check branches to a block that calls `sb_panic` and ends in `unreachable`.
- A block left without a terminator gets `ret void` in a void function, or `unreachable` otherwise
  (the missing-return check guarantees that's unreachable).
- `break` and `continue` use a stack of `(continue_target, break_target)` blocks.

## Testing

- **Parser:** the existing golden `.sexp` files.
- **Typecheck:** golden files in `tests/typecheck/` with programs and their expected error messages.
- **End-to-end:** `tests/e2e/*.sb`. Each case compiles, runs, and compares stdout and the exit code,
  plus stderr for panics:

  ```
  === fib
  def fib(n: int): int { if (n < 2) { return n; } return fib(n - 1) + fib(n - 2); }
  def main(): int { println(fib(10)); return 0; }
  --- stdout
  55
  --- exit
  0
  ```

  When `clang` isn't on `PATH`, the e2e tests are skipped instead of failing. CI runs on a runner
  that has clang.

## Milestones

Each feature goes through the checker and codegen together, as a vertical slice, apart from the
checker skeleton in step 2.

1. **Frontend prep.** Spans on AST nodes and errors, `int` / `float` keywords and literals (with
   `number` removed), class field and member nodes, `break` / `continue`, array literals, the CLI.
2. **Checker skeleton.** Name resolution, type checking, the typed AST, the missing-return check, the
   typecheck golden suite.
3. **M1: scalars.** `int` / `float` / `boolean`, functions, control flow, short-circuit `&&` / `||`,
   `print` / `println`, checked division, the e2e harness. **Target: recursive `fib(30)`.**
4. **M2: strings.** Literals, concatenation, `==`, `len`, `sb_alloc`.
5. **M3: arrays.** Literals, `new [T](n)`, indexing, bounds checks.
6. **M4: classes.** Fields, constructors, statically dispatched methods, `nil` and null checks.
7. **M5: inheritance.** Vtables, overriding, `super`.
8. **Later, optional.** Boehm GC, `inkwell` with optimization passes, optional types, generics
   through monomorphization, global constants.
