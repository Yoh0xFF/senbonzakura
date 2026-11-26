//! Parses calls `f(a, b)` or `f(x: a, y: b)`, indexing `a[i]`, field access `a.b` and `a++` and
//! `a--` after a primary, from [`primary`](super::primary). They bind tighter than prefix
//! operators, so `-f()` is `-(f())`, and chain left to right, so `a.b(c)` calls `a.b`.

use std::collections::HashSet;

use super::cursor::TokenCursor;
use senbonzakura_ast::{Argument, Expr, ExprKind, UpdateOp};
use senbonzakura_lexer::Kind;

pub(crate) fn postfix(c: &mut TokenCursor) -> Expr {
    let mut expr = super::primary(c);
    loop {
        expr = match c.peek().kind {
            Kind::LeftParen => call(c, expr),
            Kind::LeftBracket => index(c, expr),
            Kind::Dot => field(c, expr),
            Kind::PlusPlus | Kind::MinusMinus => update(c, expr),
            _ => return expr,
        };
    }
}

/// Allows a trailing comma, and treats a missing comma between arguments as one problem. A mix of
/// positional and named arguments is reported once, at the first argument of the other kind.
fn call(c: &mut TokenCursor, callee: Expr) -> Expr {
    debug_assert!(c.at(Kind::LeftParen));
    let open = c.advance();
    let mut arguments: Vec<Argument> = Vec::new();
    let mut names = HashSet::new();
    let mut mix_reported = false;
    while !c.at(Kind::RightParen) && !at_closer(c) {
        let argument = argument(c);
        if !mix_reported
            && arguments.first().is_some_and(|first| argument.is_named() != first.is_named())
        {
            c.report(
                argument.start,
                argument.end,
                "positional and named arguments cannot be mixed",
            );
            mix_reported = true;
        }
        if let Some(name) = argument.name.as_ref().filter(|name| !name.is_empty())
            && !names.insert(name.clone())
        {
            c.report(argument.start, argument.end, format!("argument '{name}' is given twice"));
        }
        arguments.push(argument);

        if c.at(Kind::Comma) {
            c.advance();
        } else if !c.at(Kind::RightParen) && !at_closer(c) {
            c.report_found("expected ',' or ')'");
        }
    }

    let start = callee.start;
    let end = if c.at(Kind::RightParen) {
        c.advance().end
    } else {
        c.report_found("expected ')'");
        arguments.last().map_or(open.end, |last| last.end)
    };
    Expr::new(ExprKind::Call { callee: Box::new(callee), arguments }, start, end)
}

/// Parses `value` or `name: value`. The name is parsed as an expression and only then known to be
/// a name, by the `:` after it, so the cursor never has to look past one token.
fn argument(c: &mut TokenCursor) -> Argument {
    let first = super::expression(c);
    if !c.at(Kind::Colon) {
        let (start, end) = (first.start, first.end);
        return Argument { name: None, value: first, start, end };
    }

    let name = match &first.kind {
        ExprKind::Identifier(name) => name.clone(),
        ExprKind::Error => String::new(),
        _ => {
            c.report(first.start, first.end, "argument name must be an identifier");
            String::new()
        }
    };
    c.advance();
    let value = super::expression(c);
    let end = value.end;
    Argument { name: Some(name), value, start: first.start, end }
}

fn index(c: &mut TokenCursor, target: Expr) -> Expr {
    debug_assert!(c.at(Kind::LeftBracket));
    c.advance();
    let index = super::expression(c);
    let end = if c.at(Kind::RightBracket) {
        c.advance().end
    } else {
        c.report_found("expected ']'");
        index.end
    };
    let start = target.start;
    Expr::new(ExprKind::Index { target: Box::new(target), index: Box::new(index) }, start, end)
}

fn field(c: &mut TokenCursor, target: Expr) -> Expr {
    debug_assert!(c.at(Kind::Dot));
    let dot = c.advance();
    let start = target.start;
    if !c.at(Kind::Identifier) {
        c.report_found("expected field name");
        return Expr::new(
            ExprKind::Field { target: Box::new(target), name: String::new() },
            start,
            dot.end,
        );
    }
    let name = c.advance();
    let field = ExprKind::Field { target: Box::new(target), name: c.text(name).to_string() };
    Expr::new(field, start, name.end)
}

/// Like an assignment, reports a target that cannot be written to but still builds the node.
fn update(c: &mut TokenCursor, target: Expr) -> Expr {
    let token = c.advance();
    let op = if token.kind == Kind::PlusPlus { UpdateOp::Increment } else { UpdateOp::Decrement };
    if !super::is_assignable(&target) {
        c.report(target.start, target.end, format!("invalid target for '{}'", op.spelling()));
    }
    let start = target.start;
    Expr::new(ExprKind::Update { op, target: Box::new(target) }, start, token.end)
}

/// A token an argument list cannot go past: one that ends an enclosing construct, a `{` that
/// starts a block, or the `if` of a trailing condition. It must cover every token
/// [`primary`](super::primary) leaves in place, or the argument loop would never move.
fn at_closer(c: &TokenCursor) -> bool {
    matches!(
        c.peek().kind,
        Kind::RightBracket
            | Kind::LeftBrace
            | Kind::RightBrace
            | Kind::Semicolon
            | Kind::If
            | Kind::Eof
    )
}
