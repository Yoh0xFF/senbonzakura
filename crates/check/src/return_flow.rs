//! Works out whether a function body cannot finish without returning, for functions that must
//! return values. It looks only at the shape of the code, not at values: `loop true { }` counts as
//! a loop that may end.

use senbonzakura_ast::Stmt;

/// Whether running these statements always ends in a `return`, or never ends.
pub(crate) fn always_returns(statements: &[Stmt]) -> bool {
    statements.iter().any(stmt_always_returns)
}

fn stmt_always_returns(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Return(s) => s.condition.is_none(),
        Stmt::Block(s) => always_returns(&s.statements),
        Stmt::If(s) => s.otherwise.as_deref().is_some_and(|otherwise| {
            always_returns(&s.then.statements) && stmt_always_returns(otherwise)
        }),
        // A loop with no condition ends only by a break, and never ending counts as returning.
        Stmt::Loop(s) => s.condition.is_none() && !breaks(&s.body.statements),
        _ => false,
    }
}

/// Whether these statements hold a `break` out of the loop whose body they are.
fn breaks(statements: &[Stmt]) -> bool {
    statements.iter().any(stmt_breaks)
}

fn stmt_breaks(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Break(_) => true,
        Stmt::Block(s) => breaks(&s.statements),
        Stmt::If(s) => {
            breaks(&s.then.statements) || s.otherwise.as_deref().is_some_and(stmt_breaks)
        }
        // A break inside a nested loop or function leaves that, not this loop.
        _ => false,
    }
}
