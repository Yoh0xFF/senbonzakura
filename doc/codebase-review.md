# Codebase Review and Roadmap

_Reviewed: 2026-10-13, at commit `cf26f66`._

## Summary

The frontend is in good shape, with two caveats: the parser has three real bugs, and the AST is
missing a few things LLVM code generation will need. The biggest gap is structural. Nothing sits
between the parser and code generation yet, so no pass resolves names or checks types.

## What's working well

- **Clean layout.** Lexer, parser and AST are separate modules, and each grammar rule has its own
  file with the production written in a doc comment.
- **Lexer.** It works on `&str` slices, tracks line, column and byte offset, handles nested comments
  correctly, and uses `thiserror` errors.
- **Golden tests.** The YAML files compare source against the expected AST and are easy to extend.
  All 22 tests pass and run in CI.
- **Typed operators.** Operator enums are kept separate from token types, so the AST doesn't depend
  on lexer details.

## Confirmed bugs

Each input was run through the parser to confirm the behavior.

| Input | Result | Where |
|---|---|---|
| `x = ;` | **Stack overflow crash** | `src/parser/parsers/expression_parse_primary.rs:29`: an unknown token falls back to `parse_left_hand_side_expression`, which calls back into primary forever. Return an "expected expression" error instead. |
| `let x: number = 1.5;` | **Panic** | `src/parser/parsers/expression_parse_literals.rs:57`: `.parse::<i32>().unwrap()`. The lexer accepts floats and `1e10`, but the AST stores `i32`. |
| `let x: number = 99999999999;` | **Panic** | Same line, integer overflow. |
| `foo().bar;`, `a.b().c();` | Parse error | `src/parser/parsers/expression_parse_left_hand_side.rs:29`: member access is parsed, then calls, but never member access after a call. Use one loop that handles `.`, `[` and `(` in any order. |
| `"a\nb"` | Stored as the raw text `a\\nb` | Escape sequences are checked in the lexer but never decoded. Code generation needs the decoded bytes. |

## Design issues that will block LLVM code generation

1. **The AST has no source locations.** The type checker can't report where an error is. Add a
   `span: (start, end)` to every node, or wrap nodes as `Spanned<T>`.
2. **Parse errors have no position, and they're mislabeled.** Every `eat_token` failure becomes a
   `LexicalError` with no line or column. Include `lookahead.start` in the error and use the correct
   variant.
3. **`number` is undefined at the machine level.** LLVM needs to know whether it's `i64` or
   `double`. Suggestion: make `number` a `double`, the way JavaScript does, or split it into `int`
   and `float`. Decide this before writing code generation.
4. **Names are stored as `Box<Expression>`.** Function, class and variable names, and parameters,
   should be plain `String`s (or an `Ident { name, span }`). Otherwise every later pass has to
   unwrap the expression and handle cases that can't happen.
5. **A variable declaration is an expression.** `Expression::Variable` should be its own declaration
   node, not something that can appear inside arbitrary expressions.
6. **Class bodies accept any statement, and fields aren't declared.** `class A { 1 + 2; }` parses
   without error. `this.x = x` implies fields that are created at runtime, but LLVM needs a fixed
   struct layout. Add `ClassMember::Field { name, type }` and `ClassMember::Method`.
7. **The visitor adds little value.** `AstVisitor` returns one `Output` type for both statements and
   expressions, and the dispatcher traits are just boilerplate. In Rust, a plain function per pass
   that does `match stmt { ... }` is simpler and is the usual approach.
8. **The language has a lot of runtime features.** Strings, `nil`, classes, inheritance and generics
   each need a runtime and a memory strategy. Don't try to compile all of it at once.

## Roadmap and action items

### Phase 0: Fix the bugs and cleanup

- [ ] Fix the stack overflow, the literal-parsing panic, and member access after a call.
- [ ] Decode string escape sequences in the parser.
- [ ] Add **error-case tests**, for example an `expected_error:` field in the YAML. Right now every
      test checks only successful parses.
- [ ] Fix the 16 `cargo clippy` warnings, then add `cargo clippy -- -D warnings` and
      `cargo fmt --check` to CI.
- [ ] Remove the unused `TokenType` entries marked TODO, or implement them (`%`, `++`, bitwise
      operators).

### Phase 1: Prepare the frontend for later passes

- [ ] Add spans to AST nodes and to all errors.
- [ ] Add a diagnostics crate (`ariadne` or `codespan-reporting`) so errors show the source line with
      the problem underlined.
- [ ] Replace `number` with `int` / `float`, add class field declarations, `break` / `continue` and
      array literals.
- [ ] Replace the hard-coded program in `main.rs` with a CLI:
      `senbonzakura build file.sb -o file`, plus `--emit=tokens|ast|ll`.

### Phase 2 onward

The open questions this review raised (what `number` means, class fields, `nil`, memory, the runtime
language) were settled on 2026-10-17. See `language-design.md`, whose **Milestones** section replaces
the rest of this roadmap: the checker skeleton, then codegen M1 (`fib(30)`), strings, arrays,
classes, and inheritance.

## Where to start

Finish Phase 0 and Phase 1, then build the checker skeleton described in `language-design.md`.
