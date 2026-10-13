# Rust vs C++ for the LLVM Backend

_Decided: 2026-10-13._

## Decision

**Stay with Rust.** The project already has about 10k lines of lexer, parser, AST and visitor code
plus YAML-driven tests. Rewriting all of that in C++ would cost weeks and give very little back for
a toy language.

## Is C++'s LLVM integration better?

Technically yes. LLVM is written in C++, so the C++ API is the real one:

- Every LLVM feature is available, including the newest ones (custom passes, the ORC JIT, full debug
  info).
- The official Kaleidoscope tutorial and almost every blog post, Stack Overflow answer and real
  compiler (Clang, Swift, Zig's old backend) use C++.
- No wrapper layer, so nothing lags behind LLVM releases.

## Is it easier?

No, not for this project. C++ would also bring:

- CMake, `llvm-config` and linking problems, plus slow builds because of LLVM's heavy headers.
- Manual memory management in the frontend, and no `enum` + `match` for the AST. That combination is
  where Rust is strongest for compilers.
- Throwing away code that already works.

## What Rust gives you

- **[inkwell](https://github.com/TheDan64/inkwell)** is a safe, idiomatic wrapper over LLVM's C API.
  It covers everything a toy language needs: modules, functions, basic blocks, phi nodes, structs,
  GEP, calls, running optimization passes, and emitting object files or assembly through
  `TargetMachine`.
- What the C API doesn't expose is mostly advanced material (writing custom passes, some JIT
  internals, detailed debug info).
- The main annoyance is that inkwell is tied to specific LLVM versions. On macOS: install a version
  it supports (for example `brew install llvm@18`), enable the matching cargo feature
  (`features = ["llvm18-0"]`), and point `LLVM_SYS_180_PREFIX` at `$(brew --prefix llvm@18)`. Check
  which versions the current inkwell release supports before installing.

## Simplest way to start

Skip bindings at first. Write a code generation pass that outputs **textual LLVM IR** (`.ll` files)
as strings, then run `clang out.ll -o out`.

- No dependency or linking problems at all.
- You learn what IR actually looks like, which helps a lot when debugging later.
- It fits the existing AST pass structure.
- Move to inkwell later for in-memory IR, a JIT, or running passes from inside the compiler. The
  concepts carry over directly.

Pick C++ only if the goal is specifically to learn LLVM's internal C++ API itself.
