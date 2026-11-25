//! Prints a tree as an S-expression, like `(+ 1 (* 2 3))`, so its shape fits on one line. Calls
//! print as `(call f 1 2)`, or `(call f x: 1 y: 2)` with named arguments. Statements print as
//! `{(= x 1); y;}`, declarations as `(var a: int 1, b)`, and functions as
//! `(fn f (a: int): bool {...})`. Parts that failed to parse print as `<error>`.

use super::{Argument, Block, Declarator, Expr, ExprKind, Stmt, Type, TypeKind, VarStmt};
use crate::float_text;

pub fn expr(e: &Expr) -> String {
    match &e.kind {
        ExprKind::IntLiteral(value) => value.to_string(),
        ExprKind::FloatLiteral(value) => float_text::double(*value),
        ExprKind::StringLiteral(value) => format!("\"{value}\""),
        ExprKind::CharLiteral(value) => format!("'{value}'"),
        ExprKind::BoolLiteral(value) => value.to_string(),
        ExprKind::Identifier(name) => name.clone(),
        ExprKind::Call { callee, arguments } => {
            let mut parts = vec!["call".to_string(), expr(callee)];
            parts.extend(arguments.iter().map(argument));
            format!("({})", parts.join(" "))
        }
        ExprKind::Index { target, index } => format!("(index {} {})", expr(target), expr(index)),
        ExprKind::Field { target, name } => format!("(. {} {})", expr(target), self::name(name)),
        ExprKind::Unary { op, operand } => format!("({} {})", op.spelling(), expr(operand)),
        ExprKind::Binary { op, left, right } => {
            format!("({} {} {})", op.spelling(), expr(left), expr(right))
        }
        ExprKind::Range { lower, upper } => format!("(.. {} {})", expr(lower), expr(upper)),
        ExprKind::Assign { op, target, value } => {
            format!("({} {} {})", op.spelling(), expr(target), expr(value))
        }
        ExprKind::Update { op, target } => format!("({} {})", op.spelling(), expr(target)),
        ExprKind::Group(inner) => format!("(group {})", expr(inner)),
        ExprKind::Cast { value, ty: t } => format!("(as {} {})", expr(value), ty(t)),
        ExprKind::Error => "<error>".to_string(),
    }
}

pub fn stmt(s: &Stmt) -> String {
    match s {
        Stmt::Expression(s) => format!("{};", expr(&s.expr)),
        Stmt::Var(s) => var(s),
        Stmt::If(s) => {
            let otherwise = s.otherwise.as_ref().map(|o| format!(" else {}", stmt(o)));
            format!(
                "(if {} {}{})",
                expr(&s.condition),
                block(&s.then),
                otherwise.unwrap_or_default()
            )
        }
        Stmt::Loop(s) => {
            let condition = s.condition.as_ref().map(|c| format!("{} ", expr(c)));
            format!("(loop {}{})", condition.unwrap_or_default(), block(&s.body))
        }
        Stmt::LoopIn(s) => format!(
            "(loop var {}{} in {} {})",
            name(&s.name),
            annotation(s.ty.as_ref()),
            expr(&s.iterable),
            block(&s.body)
        ),
        Stmt::LoopStep(s) => format!(
            "(loop {} {} {} {})",
            var(&s.init),
            expr(&s.condition),
            expr(&s.step),
            block(&s.body)
        ),
        Stmt::Continue(s) => jump("continue", s.condition.as_ref()),
        Stmt::Break(s) => jump("break", s.condition.as_ref()),
        Stmt::Return(s) => {
            let values: Vec<_> = s.values.iter().map(|v| format!(" {}", expr(v))).collect();
            let condition = s.condition.as_ref().map(|c| format!(" if {}", expr(c)));
            format!("(return{}{})", values.join(","), condition.unwrap_or_default())
        }
        Stmt::Function(f) => {
            let parameters: Vec<_> = f
                .parameters
                .iter()
                .map(|p| format!("{}{}", name(&p.name), annotation(Some(&p.ty))))
                .collect();
            let returns = if f.return_types.is_empty() {
                String::new()
            } else {
                let types: Vec<_> = f.return_types.iter().map(ty).collect();
                format!(": {}", types.join(", "))
            };
            format!(
                "(fn {} ({}){} {})",
                name(&f.name),
                parameters.join(", "),
                returns,
                block(&f.body)
            )
        }
        Stmt::Block(s) => block(s),
    }
}

pub fn ty(t: &Type) -> String {
    match t.kind {
        TypeKind::Primitive(kind) => kind.spelling().to_string(),
        TypeKind::Error => "<error>".to_string(),
    }
}

fn var(s: &VarStmt) -> String {
    let declarators: Vec<_> = s.declarators.iter().map(declarator).collect();
    format!("(var {})", declarators.join(", "))
}

fn block(b: &Block) -> String {
    let statements: Vec<_> = b.statements.iter().map(stmt).collect();
    format!("{{{}}}", statements.join(" "))
}

fn jump(keyword: &str, condition: Option<&Expr>) -> String {
    match condition {
        None => format!("({keyword})"),
        Some(c) => format!("({keyword} if {})", expr(c)),
    }
}

fn argument(a: &Argument) -> String {
    match &a.name {
        Some(n) => format!("{}: {}", name(n), expr(&a.value)),
        None => expr(&a.value),
    }
}

fn declarator(d: &Declarator) -> String {
    let initializer = d.initializer.as_ref().map(|i| format!(" {}", expr(i)));
    format!("{}{}{}", name(&d.name), annotation(d.ty.as_ref()), initializer.unwrap_or_default())
}

fn name(name: &str) -> &str {
    if name.is_empty() { "<error>" } else { name }
}

fn annotation(t: Option<&Type>) -> String {
    t.map(|t| format!(": {}", ty(t))).unwrap_or_default()
}
