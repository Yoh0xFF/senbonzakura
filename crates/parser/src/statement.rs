//! Parses statements: an expression followed by `;`, a `var`, an `if` with an optional `else`, a
//! `loop`, a `continue`, a `break`, a `return`, a `fn`, or a block of statements in `{}`.
//! Statements inside a block come from [`statement`](super::statement).
//!
//! A missing `;` or body is reported only when the rest of the statement had no problems, so one
//! mistake does not produce two diagnostics. A missing body becomes an empty block, so the tokens
//! after it start the next statement.

use std::rc::Rc;

use super::cursor::TokenCursor;
use super::types;
use senbonzakura_ast::{
    Block, Declarator, Expr, ExprKind, ExpressionStmt, Function, IfStmt, JumpStmt, LoopInStmt,
    LoopStepStmt, LoopStmt, Parameter, ReturnStmt, Stmt, Type, TypeKind, VarStmt,
};
use senbonzakura_lexer::{Kind, Token};

pub(crate) fn expression(c: &mut TokenCursor) -> Stmt {
    let problems_before = c.problem_count();
    let expr = super::expression(c);
    let end = semicolon(c, problems_before, expr.end, "expected ';' after expression");
    let start = expr.start;
    Stmt::Expression(ExpressionStmt { expr, start, end })
}

/// The cursor must be on `var`.
pub(crate) fn var_statement(c: &mut TokenCursor) -> Stmt {
    debug_assert!(c.at(Kind::Var));
    let problems_before = c.problem_count();
    let keyword = c.advance();
    let first = binding(c, false);
    Stmt::Var(var_rest(c, problems_before, keyword, first))
}

/// `name: type`, the part of a declarator before its initializer, and all of a `loop var x in`
/// variable.
struct Binding {
    name: String,
    ty: Option<Type>,
    start: usize,
    end: usize,
}

/// Parses a name and an optional `: type`. A keyword or literal where the name should be is
/// consumed, unless it can follow the name; `in_loop` adds `in` to those.
fn binding(c: &mut TokenCursor, in_loop: bool) -> Binding {
    let start = c.peek().start;
    let mut name = String::new();
    let mut end = start;
    if c.at(Kind::Identifier) {
        let token = c.advance();
        name = c.text(token).to_string();
        end = token.end;
    } else {
        c.report_found("expected variable name");
        if !can_follow_name(c) && !(in_loop && c.at(Kind::In)) {
            end = c.advance().end;
        }
    }

    let mut ty = None;
    if c.at(Kind::Colon) {
        c.advance();
        let t = super::types::ty(c);
        end = t.end;
        ty = Some(t);
    }
    Binding { name, ty, start, end }
}

fn can_follow_name(c: &TokenCursor) -> bool {
    matches!(
        c.peek().kind,
        Kind::Colon | Kind::Eq | Kind::Comma | Kind::Semicolon | Kind::RightBrace | Kind::Eof
    )
}

/// Parses the rest of a `var` after its first name and type: that declarator's optional
/// `= initializer`, any more declarators after commas, then `;`.
fn var_rest(
    c: &mut TokenCursor,
    problems_before: usize,
    keyword: Token,
    first: Binding,
) -> VarStmt {
    let mut declarators = Vec::new();
    let mut binding = first;
    loop {
        let mut initializer = None;
        let mut end = binding.end;
        if c.at(Kind::Eq) {
            c.advance();
            let value = super::expression(c);
            end = value.end;
            initializer = Some(value);
        }
        declarators.push(Declarator {
            name: binding.name,
            ty: binding.ty,
            initializer,
            start: binding.start,
            end,
        });

        if !c.at(Kind::Comma) {
            break;
        }
        c.advance();
        binding = self::binding(c, false);
    }

    let last_end = declarators.last().expect("at least one declarator").end;
    let end = semicolon(c, problems_before, last_end, "expected ';' after var statement");
    VarStmt { declarators, start: keyword.start, end }
}

/// Consumes the `;` that ends a statement and returns the statement's end: past the `;`, or `end`
/// if it is missing.
fn semicolon(c: &mut TokenCursor, problems_before: usize, end: usize, message: &str) -> usize {
    if c.at(Kind::Semicolon) {
        return c.advance().end;
    }
    if c.problem_count() == problems_before {
        c.report(end, end, message);
    }
    end
}

/// The cursor must be on `if`. There are no parentheses around the condition, and the body must
/// be a block, optionally followed by `else` and a block or another `if`. An `else` after a
/// missing body is still parsed.
pub(crate) fn if_statement(c: &mut TokenCursor) -> Stmt {
    debug_assert!(c.at(Kind::If));
    let problems_before = c.problem_count();
    let if_token = c.advance();
    let condition = super::expression(c);
    let then = body(c, problems_before, condition.end, "after if condition");

    if !c.at(Kind::Else) {
        let end = then.end;
        return Stmt::If(IfStmt { condition, then, otherwise: None, start: if_token.start, end });
    }
    let otherwise = else_branch(c);
    let end = otherwise.end();
    Stmt::If(IfStmt {
        condition,
        then,
        otherwise: Some(Box::new(otherwise)),
        start: if_token.start,
        end,
    })
}

fn else_branch(c: &mut TokenCursor) -> Stmt {
    debug_assert!(c.at(Kind::Else));
    let else_token = c.advance();
    match c.peek().kind {
        Kind::LeftBrace => Stmt::Block(block(c)),
        Kind::If => if_statement(c),
        _ => {
            c.report_found("expected '{' or 'if' after 'else'");
            Stmt::Block(empty_block_at(else_token.end))
        }
    }
}

/// The cursor must be on `loop`. What follows it picks the form, and none has parentheses:
///
/// - `loop { ... }` runs until something leaves it.
/// - `loop var x in xs { ... }` runs once for each item of `xs`.
/// - `loop var i = 0; i < n; i++ { ... }` runs the `var` once, then the body while the condition
///   holds, running the step after each pass.
/// - `loop condition { ... }` runs while the condition holds.
pub(crate) fn loop_statement(c: &mut TokenCursor) -> Stmt {
    debug_assert!(c.at(Kind::Loop));
    let problems_before = c.problem_count();
    let loop_token = c.advance();

    if c.at(Kind::LeftBrace) {
        let body = block(c);
        let end = body.end;
        return Stmt::Loop(LoopStmt { condition: None, body, start: loop_token.start, end });
    }
    if c.at(Kind::Var) {
        return var_loop(c, loop_token, problems_before);
    }
    let condition = super::expression(c);
    let body = body(c, problems_before, condition.end, "after loop condition");
    let end = body.end;
    Stmt::Loop(LoopStmt { condition: Some(condition), body, start: loop_token.start, end })
}

/// Tells `loop var x in` from `loop var i = 0;` by what follows the name and type. Anything else
/// is reported as a missing `in`, and the loop is parsed as one over what follows.
fn var_loop(c: &mut TokenCursor, loop_token: Token, problems_before: usize) -> Stmt {
    debug_assert!(c.at(Kind::Var));
    let keyword = c.advance();
    let binding = binding(c, true);
    if c.at(Kind::Eq) || c.at(Kind::Comma) || c.at(Kind::Semicolon) {
        let init = var_rest(c, problems_before, keyword, binding);
        return step_loop(c, loop_token, problems_before, init);
    }

    if c.at(Kind::In) {
        c.advance();
    } else if c.problem_count() == problems_before {
        c.report_found("expected 'in' or '=' after loop variable");
    }
    // A '{' here is the body, so the missing 'in' or name is the only problem reported.
    let iterable = if c.at(Kind::LeftBrace) && c.problem_count() > problems_before {
        let at = c.peek().start;
        Expr::new(ExprKind::Error, at, at)
    } else {
        super::expression(c)
    };
    let body = body(c, problems_before, iterable.end, "after loop iterable");
    let end = body.end;
    Stmt::LoopIn(LoopInStmt {
        name: binding.name,
        ty: binding.ty,
        iterable,
        body,
        start: loop_token.start,
        end,
    })
}

fn step_loop(
    c: &mut TokenCursor,
    loop_token: Token,
    problems_before: usize,
    init: VarStmt,
) -> Stmt {
    let condition = super::expression(c);
    semicolon(c, problems_before, condition.end, "expected ';' after loop condition");
    let step = super::expression(c);
    let body = body(c, problems_before, step.end, "after loop step");
    let end = body.end;
    Stmt::LoopStep(LoopStepStmt { init, condition, step, body, start: loop_token.start, end })
}

/// The cursor must be on `fn`. Parses `fn name(a: int, b: int): int, bool { ... }`, where the `:`
/// and return types are left out for a function that returns nothing. The parameter list allows a
/// trailing comma. A missing `:` before a parameter's type or the return types is reported, and
/// the types after it still parsed.
pub(crate) fn function(c: &mut TokenCursor) -> Stmt {
    debug_assert!(c.at(Kind::Fn));
    let problems_before = c.problem_count();
    let fn_token = c.advance();
    let mut end = fn_token.end;

    let mut name = String::new();
    if c.at(Kind::Identifier) {
        let token = c.advance();
        name = c.text(token).to_string();
        end = token.end;
    } else {
        c.report_found("expected function name");
        if !can_follow_function_name(c) {
            end = c.advance().end;
        }
    }

    let mut parameters = Vec::new();
    if c.at(Kind::LeftParen) {
        end = self::parameters(c, &mut parameters);
    } else if c.problem_count() == problems_before {
        c.report_found("expected '(' after function name");
    }

    let mut return_types = Vec::new();
    if c.at(Kind::Colon) || types::at_type(c) {
        if c.at(Kind::Colon) {
            c.advance();
        } else if c.problem_count() == problems_before {
            c.report_found("expected ':' before return type");
        }
        return_types.push(types::ty(c));
        while c.at(Kind::Comma) {
            c.advance();
            return_types.push(types::ty(c));
        }
        end = return_types.last().expect("at least one return type").end;
    }

    let body = body(c, problems_before, end, "after function signature");
    let end = body.end;
    Stmt::Function(Rc::new(Function {
        name,
        parameters,
        return_types,
        body,
        start: fn_token.start,
        end,
    }))
}

fn can_follow_function_name(c: &TokenCursor) -> bool {
    matches!(
        c.peek().kind,
        Kind::LeftParen
            | Kind::Colon
            | Kind::LeftBrace
            | Kind::RightBrace
            | Kind::Semicolon
            | Kind::Eof
    )
}

/// The cursor must be on `(`. Adds the parameters to `into` and returns where the list ends: past
/// the `)`, or after the last parameter if it is missing.
fn parameters(c: &mut TokenCursor, into: &mut Vec<Parameter>) -> usize {
    debug_assert!(c.at(Kind::LeftParen));
    let open = c.advance();
    while !c.at(Kind::RightParen) && !at_parameters_end(c) {
        let problems_before = c.problem_count();
        into.push(parameter(c));
        if c.at(Kind::Comma) {
            c.advance();
        } else if !c.at(Kind::RightParen)
            && !at_parameters_end(c)
            && c.problem_count() == problems_before
        {
            c.report_found("expected ',' or ')'");
        }
    }

    if !c.at(Kind::RightParen) {
        c.report_found("expected ')'");
        return into.last().map_or(open.end, |last| last.end);
    }
    c.advance().end
}

/// Parses `name: type`. It consumes a token unless the cursor is on one
/// [`can_follow_parameter_name`] accepts, each of which the parameter loop consumes or stops at,
/// so the loop always moves.
fn parameter(c: &mut TokenCursor) -> Parameter {
    let problems_before = c.problem_count();
    let start = c.peek().start;
    let mut name = String::new();
    let mut end = start;
    if c.at(Kind::Identifier) {
        let token = c.advance();
        name = c.text(token).to_string();
        end = token.end;
    } else {
        c.report_found("expected parameter name");
        if !can_follow_parameter_name(c) {
            end = c.advance().end;
        }
    }

    let ty = if c.at(Kind::Colon) {
        c.advance();
        types::ty(c)
    } else {
        if c.problem_count() == problems_before {
            c.report_found("expected ':' after parameter name");
        }
        if types::at_type(c) { types::ty(c) } else { Type::new(TypeKind::Error, end, end) }
    };
    let end = ty.end;
    Parameter { name, ty, start, end }
}

fn can_follow_parameter_name(c: &TokenCursor) -> bool {
    c.at(Kind::Colon) || c.at(Kind::Comma) || c.at(Kind::RightParen) || at_parameters_end(c)
}

/// A token a parameter list cannot go past, so a missing `)` does not swallow the body.
fn at_parameters_end(c: &TokenCursor) -> bool {
    matches!(c.peek().kind, Kind::LeftBrace | Kind::RightBrace | Kind::Semicolon | Kind::Eof)
}

/// Parses the block a statement requires, or reports it missing, as `expected '{' <where>`, and
/// stands in an empty block at `end`.
fn body(c: &mut TokenCursor, problems_before: usize, end: usize, place: &str) -> Block {
    if c.at(Kind::LeftBrace) {
        return block(c);
    }
    if c.problem_count() == problems_before {
        c.report_found(&format!("expected '{{' {place}"));
    }
    empty_block_at(end)
}

/// The cursor must be on `continue`.
pub(crate) fn continue_statement(c: &mut TokenCursor) -> Stmt {
    debug_assert!(c.at(Kind::Continue));
    let jump = jump(c, "continue", false);
    Stmt::Continue(JumpStmt { condition: jump.condition, start: jump.start, end: jump.end })
}

/// The cursor must be on `break`.
pub(crate) fn break_statement(c: &mut TokenCursor) -> Stmt {
    debug_assert!(c.at(Kind::Break));
    let jump = jump(c, "break", false);
    Stmt::Break(JumpStmt { condition: jump.condition, start: jump.start, end: jump.end })
}

/// The cursor must be on `return`.
pub(crate) fn return_statement(c: &mut TokenCursor) -> Stmt {
    debug_assert!(c.at(Kind::Return));
    let jump = jump(c, "return", true);
    Stmt::Return(ReturnStmt {
        values: jump.values,
        condition: jump.condition,
        start: jump.start,
        end: jump.end,
    })
}

/// What `continue`, `break` and `return` share.
struct Jump {
    values: Vec<Expr>,
    condition: Option<Expr>,
    start: usize,
    end: usize,
}

/// Parses the keyword spelled `word`, values separated by commas if `takes_value`, an optional
/// `if condition`, then `;`. The condition is a plain expression, not an `if` statement, so it
/// takes no block and no `else`.
///
/// Values are optional too. None follow when the next token is `;`, `if`, or one that closes the
/// enclosing block, so `return }` reports the missing `;` rather than a missing expression.
fn jump(c: &mut TokenCursor, word: &str, takes_value: bool) -> Jump {
    let problems_before = c.problem_count();
    let keyword = c.advance();
    let mut end = keyword.end;
    let mut message = format!("expected ';' or 'if' after '{word}'");

    let mut values = Vec::new();
    if takes_value && !ends_jump(c) {
        values.push(super::expression(c));
        while c.at(Kind::Comma) {
            c.advance();
            values.push(super::expression(c));
        }
        end = values.last().expect("at least one value").end;
        message = format!("expected ';' or 'if' after {word} value");
    }

    let mut condition = None;
    if c.at(Kind::If) {
        c.advance();
        let value = super::expression(c);
        end = value.end;
        condition = Some(value);
        message = format!("expected ';' after {word} condition");
    }

    let end = semicolon(c, problems_before, end, &message);
    Jump { values, condition, start: keyword.start, end }
}

fn ends_jump(c: &TokenCursor) -> bool {
    matches!(c.peek().kind, Kind::Semicolon | Kind::If | Kind::RightBrace | Kind::Eof)
}

fn empty_block_at(offset: usize) -> Block {
    Block { statements: Vec::new(), start: offset, end: offset }
}

/// The cursor must be on an opening brace. A missing closing brace is reported at the end of
/// input.
pub(crate) fn block(c: &mut TokenCursor) -> Block {
    debug_assert!(c.at(Kind::LeftBrace));
    let open = c.advance();
    let mut statements = Vec::new();
    while !c.at(Kind::RightBrace) && !c.at_end() {
        let first = c.peek();
        statements.push(super::statement(c));
        c.skip_if_stuck(first);
    }

    if c.at_end() {
        c.report_found("expected '}'");
        let end = statements.last().map_or(open.end, Stmt::end);
        return Block { statements, start: open.start, end };
    }
    let close = c.advance();
    Block { statements, start: open.start, end: close.end }
}
