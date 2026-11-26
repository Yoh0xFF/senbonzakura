# Senbonzakura (Rust)

Learning compiler theory..
Testing some random ideas..

A Rust rewrite of [senbonzakura](../senbonzakura): the lexer, parser, checker, tree-walking
interpreter and REPL, with the same language, messages and exit codes.

```sh
cargo run -- --run hello.sz     # check a file, then run it
cargo run -- --tree hello.sz    # print each top-level statement as an S-expression
cargo run -- hello.sz           # only check it
cargo run -- --repl             # interactive prompt
```

Exit codes: `0` ok, `1` the file has problems, `2` bad arguments or unreadable file, `3` the program
failed while running.

## Layout

A Cargo workspace, one crate per stage. Each depends only on the stages before it:

| Crate                    | Directory          | What it does                                                         |
|--------------------------|--------------------|----------------------------------------------------------------------|
| `senbonzakura-lexer`     | `crates/lexer`     | Source text to tokens; problems are collected, never thrown          |
| `senbonzakura-ast`       | `crates/ast`       | The syntax tree, and `sexpr` and `float_text` to print it            |
| `senbonzakura-parser`    | `crates/parser`    | Tokens to a tree, with error nodes where parsing failed              |
| `senbonzakura-check`     | `crates/check`     | Names, scopes, control flow and types; records each expression's type |
| `senbonzakura-interpret` | `crates/interpret` | Runs a checked tree; `Session` runs REPL entries                     |
| `senbonzakura-cli`       | `crates/cli`       | The `senbonzakura` binary: command line and interactive prompt       |

```
lexer ──► parser ──► check ──► interpret ──► cli
  ast ──────┴──────────┴──────────┘
```

## Differences from the Java version

- Offsets are byte offsets into the UTF-8 source, and columns count characters, as rustc does. The
  Java version counts UTF-16 units, so the two differ only on lines with non-ASCII text.
- Calls may nest 10,000 deep before a run fails with a stack overflow, a fixed limit rather than
  wherever the JVM's stack runs out. The binary runs on a 1 GiB thread stack to make room for it.
- The REPL edits lines with [reedline](https://github.com/nushell/reedline) instead of JLine. It
  keeps its history in `~/.senbonzakura_history` as before.

## Tasks

`task test`, `task lint`, `task fmt`, `task repl`, `task run -- --run hello.sz`. Run `task hooks`
once to format staged Rust files on commit.
