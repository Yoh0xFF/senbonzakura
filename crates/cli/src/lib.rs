//! The command line: `senbonzakura [--tree] [--run] <file>`, or `senbonzakura --repl` for the
//! interactive [`Repl`]. Parses and [checks](senbonzakura_check) the file, and prints
//! each problem found to the error stream, with the line it is on and a caret under the part it
//! covers. The checks run only on a file that lexed and parsed cleanly, so one syntax error does
//! not show up again as a list of undefined names. With `--tree` it also prints each top-level
//! statement as an [S-expression](senbonzakura_ast::sexpr), and with `--run` it then runs a file that has
//! no problems, its output going to the output stream.
//!
//! Exits with [`OK`] when the file has no problems and ran without failing, [`PROBLEMS`] when it
//! has problems, [`USAGE`] when the arguments are wrong or the file cannot be read, and
//! [`RUNTIME_ERROR`] when running it failed.

mod problems;
pub mod repl;
mod repl_input;

use std::io::{self, BufRead, Write};

use crate::repl::{Plain, Repl, Terminal, history_file};
use senbonzakura_ast::sexpr;
use senbonzakura_check as check;
use senbonzakura_interpret::Interpreter;
use senbonzakura_lexer::{Diagnostic, Lexer, SourceFile};
use senbonzakura_parser::Parser;

pub const OK: i32 = 0;
pub const PROBLEMS: i32 = 1;
pub const USAGE: i32 = 2;
pub const RUNTIME_ERROR: i32 = 3;

const USAGE_TEXT: &str = "usage: senbonzakura [--tree] [--run] <file>\n       senbonzakura --repl";

pub struct Cli<'a> {
  input: &'a mut dyn BufRead,
  out: &'a mut dyn Write,
  err: &'a mut dyn Write,
  terminal: bool,
}

impl<'a> Cli<'a> {
  /// `terminal` says whether the program runs in a terminal, so the REPL reads with line editing
  /// from it instead of plain lines from `input`.
  pub fn new(
    input: &'a mut dyn BufRead,
    out: &'a mut dyn Write,
    err: &'a mut dyn Write,
    terminal: bool,
  ) -> Cli<'a> {
    Cli { input, out, err, terminal }
  }

  pub fn run<S: AsRef<str>>(&mut self, args: &[S]) -> i32 {
    let code = self.dispatch(args);
    let _ = self.out.flush();
    let _ = self.err.flush();
    code
  }

  fn dispatch<S: AsRef<str>>(&mut self, args: &[S]) -> i32 {
    let mut tree = false;
    let mut execute = false;
    let mut repl = false;
    let mut path: Option<&str> = None;
    for arg in args.iter().map(AsRef::as_ref) {
      match arg {
        "--tree" => tree = true,
        "--run" => execute = true,
        "--repl" => repl = true,
        _ if arg.starts_with('-') || path.is_some() => return self.usage(),
        _ => path = Some(arg),
      }
    }
    if repl {
      if path.is_some() || tree || execute {
        return self.usage();
      }
      return self.repl();
    }
    let Some(path) = path else {
      return self.usage();
    };

    let text = match read(path) {
      Ok(text) => text,
      Err(reason) => {
        let _ = writeln!(self.err, "senbonzakura: cannot read {path}: {reason}");
        return USAGE;
      }
    };
    self.file(&SourceFile::new(path, text), tree, execute)
  }

  fn file(&mut self, source: &SourceFile, tree: bool, execute: bool) -> i32 {
    let mut parser = Parser::new(Lexer::new(source));
    let statements = parser.parse_file();

    if tree {
      for stmt in &statements {
        let _ = writeln!(self.out, "{}", sexpr::stmt(stmt));
      }
    }

    // The lexer's problems come in as the parser pulls tokens, so the two lists interleave.
    let mut diagnostics: Vec<Diagnostic> =
      parser.lexer().diagnostics().iter().chain(parser.diagnostics()).cloned().collect();
    let mut analysis = None;
    if diagnostics.is_empty() {
      let checked = check::analyze(&statements);
      diagnostics = checked.diagnostics().to_vec();
      analysis = Some(checked);
    }
    diagnostics.sort_by_key(|d| d.start);
    for d in &diagnostics {
      problems::print(self.err, source, d.start, d.end, "error", &d.message);
    }
    if !diagnostics.is_empty() {
      return PROBLEMS;
    }

    if execute {
      let analysis = analysis.expect("checked");
      if let Err(error) = Interpreter::new(&analysis, self.out).run(&statements) {
        let _ = self.out.flush();
        match error.location {
          Some((start, end)) => {
            problems::print(self.err, source, start, end, "runtime error", &error.message)
          }
          None => {
            let _ = writeln!(self.err, "{}: runtime error: {}", source.path(), error.message);
          }
        }
        return RUNTIME_ERROR;
      }
    }
    OK
  }

  fn repl(&mut self) -> i32 {
    if !self.terminal {
      let input = Box::new(Plain::new(&mut *self.input));
      return Repl::new(input, self.out, self.err).run();
    }
    match Terminal::new(history_file()) {
      Ok(terminal) => Repl::new(Box::new(terminal), self.out, self.err).run(),
      Err(e) => {
        let _ = writeln!(self.err, "senbonzakura: cannot use the terminal: {e}");
        USAGE
      }
    }
  }

  fn usage(&mut self) -> i32 {
    let _ = writeln!(self.err, "{USAGE_TEXT}");
    USAGE
  }
}

/// Reads a source file, or says why it cannot be read.
fn read(path: &str) -> Result<String, String> {
  let bytes = std::fs::read(path).map_err(|e| match e.kind() {
    io::ErrorKind::NotFound => "no such file".to_string(),
    io::ErrorKind::PermissionDenied => "permission denied".to_string(),
    _ => e.to_string(),
  })?;
  String::from_utf8(bytes).map_err(|_| "not valid UTF-8".to_string())
}

#[cfg(test)]
mod tests;
