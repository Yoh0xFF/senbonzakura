//! Turns tokens into a syntax tree of [`ast`](crate::ast) nodes.
//!
//! - Errors never panic. They are collected in [`Parser::diagnostics`], and the part that failed
//!   becomes an [`ExprKind::Error`] node. Problems the lexer already reported are not reported
//!   again.
//! - [`Parser`] picks a parser from the token that starts each statement, operand and primary.
//!   `TokenCursor` holds all mutable state, and the other modules are stateless.
//! - Parsers call back into [`statement`], [`expression`], [`operand`], [`primary`] and
//!   [`types::ty`] for nested parts, so they never depend on one another directly. The
//!   `operator` module alone knows operator precedence.

mod cursor;
mod decode;
mod group;
mod literal;
mod operator;
mod postfix;
mod statement;
mod types;

#[cfg(test)]
mod tests;

use crate::ast::{Expr, ExprKind, Stmt};
use crate::lexer::{Diagnostic, Kind, Lexer};
use cursor::TokenCursor;

/// Builds [`Stmt`] and [`Expr`] trees from the tokens of a [`Lexer`], one
/// [`parse_statement`](Self::parse_statement) or [`parse_expression`](Self::parse_expression) call
/// at a time.
///
/// Like the lexer, the parser never panics on bad input. Problems go to
/// [`diagnostics`](Self::diagnostics) and the part that failed becomes an [`ExprKind::Error`]
/// node. Problems the lexer found stay in the lexer's own diagnostics and are not reported again.
pub struct Parser<'a> {
    cursor: TokenCursor<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser { cursor: TokenCursor::new(lexer) }
    }

    /// Returns the problems found so far, in the order found.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.cursor.diagnostics()
    }

    /// The lexer the tokens come from, which holds the lexical problems.
    pub fn lexer(&self) -> &Lexer<'a> {
        self.cursor.lexer()
    }

    pub fn at_end(&self) -> bool {
        self.cursor.at_end()
    }

    /// Parses one statement. Always consumes at least one token, unless the input is used up, so a
    /// loop calling it until [`at_end`](Self::at_end) always finishes.
    pub fn parse_statement(&mut self) -> Stmt {
        let first = self.cursor.peek();
        let stmt = statement(&mut self.cursor);
        self.cursor.skip_if_stuck(first);
        stmt
    }

    /// Parses statements until the input is used up, and returns them in source order. A
    /// statement with problems is still returned, as a tree with error nodes in it.
    pub fn parse_file(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.parse_statement());
        }
        statements
    }

    /// Like [`parse_statement`](Self::parse_statement), always consumes at least one token unless
    /// at the end.
    pub fn parse_expression(&mut self) -> Expr {
        let first = self.cursor.peek();
        let expr = expression(&mut self.cursor);
        self.cursor.skip_if_stuck(first);
        expr
    }
}

/// Picks a parser from the token that starts the statement.
pub(crate) fn statement(c: &mut TokenCursor) -> Stmt {
    match c.peek().kind {
        Kind::LeftBrace => Stmt::Block(statement::block(c)),
        Kind::Var => statement::var_statement(c),
        Kind::If => statement::if_statement(c),
        Kind::Loop => statement::loop_statement(c),
        Kind::Continue => statement::continue_statement(c),
        Kind::Break => statement::break_statement(c),
        Kind::Return => statement::return_statement(c),
        Kind::Fn => statement::function(c),
        _ => statement::expression(c),
    }
}

pub(crate) fn expression(c: &mut TokenCursor) -> Expr {
    operator::expression(c)
}

/// Parses the part of an expression between infix operators: prefix operators, then a postfix.
pub(crate) fn operand(c: &mut TokenCursor) -> Expr {
    match c.peek().kind {
        Kind::Minus | Kind::Bang => operator::prefix(c),
        _ => postfix::postfix(c),
    }
}

/// Picks a parser from the token that starts the primary: a literal, a name or a group.
pub(crate) fn primary(c: &mut TokenCursor) -> Expr {
    match c.peek().kind {
        Kind::IntLiteral => literal::integer(c),
        Kind::FloatLiteral => literal::floating(c),
        Kind::StringLiteral => literal::string(c),
        Kind::CharLiteral => literal::character(c),
        Kind::True | Kind::False => literal::boolean(c),
        Kind::Identifier => {
            let token = c.advance();
            Expr::new(ExprKind::Identifier(c.text(token).to_string()), token.start, token.end)
        }
        Kind::LeftParen => group::parenthesized(c),
        Kind::Error => {
            // The lexer already reported this character.
            let token = c.advance();
            Expr::new(ExprKind::Error, token.start, token.end)
        }
        Kind::RightParen
        | Kind::RightBracket
        | Kind::LeftBrace
        | Kind::RightBrace
        | Kind::Comma
        | Kind::Semicolon
        | Kind::If
        | Kind::Eof => missing_operand(c),
        _ => unexpected_token(c),
    }
}

/// Whether `=`, `+=`, `++` and the like can write to `target`: a name, a field or an index. An
/// error node counts, since it was already reported.
pub(crate) fn is_assignable(target: &Expr) -> bool {
    matches!(
        target.kind,
        ExprKind::Identifier(_) | ExprKind::Field { .. } | ExprKind::Index { .. } | ExprKind::Error
    )
}

/// Leaves the token in place, so an enclosing group, argument list or statement can match it. A
/// `{` is left for the statement it belongs to, like the body after an if condition, and an `if`
/// for the condition that ends a return, break or continue.
fn missing_operand(c: &mut TokenCursor) -> Expr {
    let token = c.peek();
    c.report_found("expected expression");
    Expr::new(ExprKind::Error, token.start, token.start)
}

/// Consumes a token nothing else recognized, so parsing always makes progress.
fn unexpected_token(c: &mut TokenCursor) -> Expr {
    debug_assert!(!c.at_end());
    c.report_found("expected expression");
    let token = c.advance();
    Expr::new(ExprKind::Error, token.start, token.end)
}
