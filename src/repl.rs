//! The interactive prompt.

use std::io::Write;
use std::path::PathBuf;

use crate::ast::Stmt;
use crate::check;
use crate::cli::{OK, USAGE};
use crate::interpret;
use crate::lexer::{Diagnostic, Kind, Lexer, SourceFile, Token};
use crate::parser::Parser;
use crate::problems;
pub use crate::repl_input::{Plain, ReplInput, Terminal};

pub const PROMPT: &str = "> ";
pub const MORE: &str = "... ";
pub const BANNER: &str =
    "Senbonzakura. End an expression without ';' to see its value. Ctrl-D quits.";

/// The interactive prompt. It reads one entry at a time, then checks and runs it, keeping the
/// variables and functions of earlier entries. Each entry gets a scope inside the earlier ones, so
/// declaring a name again shadows the old one.
///
/// - An entry that ends in an expression without a `;` shows that expression's value.
/// - An entry goes on to the next line, at the `...` prompt, while a parenthesis, bracket, brace,
///   multi-line string or block comment is still open. A blank line ends it anyway.
/// - An entry with a problem, or that fails while running, is reported and forgotten.
///
/// The prompt ends when the input does, as with Ctrl-D. On a terminal, [`Terminal`] reads with
/// line editing and history.
pub struct Repl<'a> {
    input: Box<dyn ReplInput + 'a>,
    out: &'a mut dyn Write,
    err: &'a mut dyn Write,
    checker: check::Session,
    interpreter: interpret::Session,
}

impl<'a> Repl<'a> {
    pub fn new(
        input: Box<dyn ReplInput + 'a>,
        out: &'a mut dyn Write,
        err: &'a mut dyn Write,
    ) -> Repl<'a> {
        Repl {
            input,
            out,
            err,
            checker: check::Session::new(),
            interpreter: interpret::Session::new(),
        }
    }

    /// Reads and runs entries until the input ends, and returns the exit code.
    pub fn run(&mut self) -> i32 {
        let _ = writeln!(self.out, "{BANNER}");
        loop {
            match self.input.next(self.out) {
                Ok(None) => {
                    let _ = writeln!(self.out);
                    let _ = self.out.flush();
                    return OK;
                }
                Ok(Some(entry)) => {
                    if !entry.trim().is_empty() {
                        self.evaluate(&entry);
                    }
                }
                Err(e) => {
                    let _ = writeln!(self.err, "senbonzakura: cannot read input: {e}");
                    return USAGE;
                }
            }
        }
    }

    fn evaluate(&mut self, text: &str) {
        let source = SourceFile::new("", text);
        let mut parser = Parser::new(Lexer::new(&source));
        let statements = parser.parse_file();
        let mut problems: Vec<Diagnostic> =
            parser.lexer().diagnostics().iter().chain(parser.diagnostics()).cloned().collect();

        // At the end of an entry, a missing ';' after an expression asks to see its value.
        let show_last = match statements.last() {
            Some(Stmt::Expression(last)) if last.end == last.expr.end => {
                let end = last.expr.end;
                problems
                    .retain(|d| !(d.start == end && d.message == "expected ';' after expression"));
                true
            }
            _ => false,
        };
        if self.report(&source, problems) {
            return;
        }

        let analysis = self.checker.check(&statements);
        if self.report(&source, analysis.diagnostics().to_vec()) {
            return;
        }

        match self.interpreter.run(&statements, &analysis, show_last, self.out) {
            Ok(Some(shown)) => {
                let _ = writeln!(self.out, "{shown}");
            }
            Ok(None) => {}
            Err(error) => {
                self.checker.forget_last();
                let _ = self.out.flush();
                match error.location {
                    Some((start, end)) => problems::print(
                        self.err,
                        &source,
                        start,
                        end,
                        "runtime error",
                        &error.message,
                    ),
                    None => {
                        let _ = writeln!(self.err, "runtime error: {}", error.message);
                    }
                }
            }
        }
    }

    /// Prints the problems in source order, and returns whether there were any.
    fn report(&mut self, source: &SourceFile, mut problems: Vec<Diagnostic>) -> bool {
        if problems.is_empty() {
            return false;
        }
        let _ = self.out.flush();
        problems.sort_by_key(|d| d.start);
        for d in &problems {
            problems::print(self.err, source, d.start, d.end, "error", &d.message);
        }
        true
    }
}

/// The file a terminal REPL keeps its history in, `~/.senbonzakura_history`.
pub fn history_file() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".senbonzakura_history"))
}

/// Whether an entry has something open that a later line would close.
pub fn unfinished(text: &str) -> bool {
    let source = SourceFile::new("", text);
    let mut lexer = Lexer::new(&source);
    let mut depth: i64 = 0;
    loop {
        let token = lexer.next_token();
        match token.kind {
            Kind::Eof => break,
            Kind::LeftParen | Kind::LeftBracket | Kind::LeftBrace => depth += 1,
            Kind::RightParen | Kind::RightBracket | Kind::RightBrace => depth -= 1,
            _ => {
                if token.has(Token::UNTERMINATED) && text.as_bytes()[token.start] == b'`' {
                    return true;
                }
            }
        }
    }
    depth > 0 || lexer.diagnostics().iter().any(|d| d.message == "unterminated block comment")
}

#[cfg(test)]
mod tests;
