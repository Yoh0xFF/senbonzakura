mod parse_function;
mod parse_group;
mod parse_identifier;
mod parse_literal;
mod parse_operator;
mod parse_postfix;
mod parse_statement;
mod parse_type;
mod parser;

use super::Parser;
use senbonzakura_ast::{Argument, Block, Declarator, Expr, Parameter, Stmt, Type, VarStmt, sexpr};
use senbonzakura_lexer::{Diagnostic, Lexer, SourceFile};

/// Anything that covers a range of the source.
pub(super) trait Spanned {
  fn span(&self) -> (usize, usize);
}

macro_rules! spanned {
    ($($node:ty),*) => {
        $(impl Spanned for $node {
            fn span(&self) -> (usize, usize) {
                (self.start, self.end)
            }
        })*
    };
}

spanned!(Expr, Type, Declarator, Parameter, Block, Argument, VarStmt);

impl Spanned for Stmt {
  fn span(&self) -> (usize, usize) {
    (self.start(), self.end())
  }
}

/// A whole input parsed as one expression, with everything the tests look at.
pub(super) struct Parsed {
  pub source: SourceFile,
  pub expr: Expr,
  pub lexer_diagnostics: Vec<Diagnostic>,
  pub parser_diagnostics: Vec<Diagnostic>,
}

pub(super) fn parse(text: &str) -> Parsed {
  parse_expression(text, true)
}

/// Parses the first expression and ignores anything left after it.
pub(super) fn parse_first(text: &str) -> Parsed {
  parse_expression(text, false)
}

pub(super) fn parse_clean(text: &str) -> Parsed {
  let parsed = parse(text);
  assert_eq!(parsed.lexer_diagnostics, [], "lexer diagnostics for {text:?}");
  assert_eq!(parsed.parser_diagnostics, [], "parser diagnostics for {text:?}");
  parsed
}

fn parse_expression(text: &str, whole_input: bool) -> Parsed {
  let source = SourceFile::new("test", text);
  let (expr, lexer_diagnostics, parser_diagnostics) = {
    let mut parser = Parser::new(Lexer::new(&source));
    let expr = parser.parse_expression();
    if whole_input {
      assert!(parser.at_end(), "input left after the expression in: {text:?}");
    }
    (expr, parser.lexer().diagnostics().to_vec(), parser.diagnostics().to_vec())
  };
  Parsed { source, expr, lexer_diagnostics, parser_diagnostics }
}

impl Parsed {
  pub fn tree(&self) -> String {
    sexpr::expr(&self.expr)
  }

  pub fn expr_text(&self) -> &str {
    self.text(&self.expr)
  }

  pub fn text(&self, node: &impl Spanned) -> &str {
    let (start, end) = node.span();
    &self.source.text()[start..end]
  }

  pub fn parser_messages(&self) -> Vec<&str> {
    self.parser_diagnostics.iter().map(|d| d.message.as_str()).collect()
  }

  /// The source text each parser diagnostic points at.
  pub fn parser_texts(&self) -> Vec<&str> {
    self.parser_diagnostics.iter().map(|d| &self.source.text()[d.start..d.end]).collect()
  }
}

/// Like [`Parsed`], for a whole input that is one statement.
pub(super) struct ParsedStmt {
  pub source: SourceFile,
  pub stmt: Stmt,
  pub parser_diagnostics: Vec<Diagnostic>,
}

pub(super) fn parse_stmt(text: &str) -> ParsedStmt {
  let source = SourceFile::new("test", text);
  let (stmt, parser_diagnostics) = {
    let mut parser = Parser::new(Lexer::new(&source));
    let stmt = parser.parse_statement();
    assert!(parser.at_end(), "input left after the statement in: {text:?}");
    assert_eq!(parser.lexer().diagnostics(), [], "lexer diagnostics for {text:?}");
    (stmt, parser.diagnostics().to_vec())
  };
  ParsedStmt { source, stmt, parser_diagnostics }
}

pub(super) fn parse_stmt_clean(text: &str) -> ParsedStmt {
  let parsed = parse_stmt(text);
  assert_eq!(parsed.parser_diagnostics, [], "parser diagnostics for {text:?}");
  parsed
}

/// Statements parsed until the input is used up, for inputs that recovery splits into several.
pub(super) struct All {
  pub trees: Vec<String>,
  pub messages: Vec<String>,
}

pub(super) fn parse_all(text: &str) -> All {
  let source = SourceFile::new("test", text);
  let mut parser = Parser::new(Lexer::new(&source));
  let trees = parser.parse_file().iter().map(sexpr::stmt).collect();
  let messages = parser.diagnostics().iter().map(|d| d.message.clone()).collect();
  All { trees, messages }
}

impl ParsedStmt {
  pub fn tree(&self) -> String {
    sexpr::stmt(&self.stmt)
  }

  pub fn text(&self, node: &impl Spanned) -> &str {
    let (start, end) = node.span();
    &self.source.text()[start..end]
  }

  pub fn parser_messages(&self) -> Vec<&str> {
    self.parser_diagnostics.iter().map(|d| d.message.as_str()).collect()
  }
}
