use std::fmt;

use super::{Expression, Statement, Type};

///
/// Maximum line width the pretty printer aims for
///
const MAX_WIDTH: usize = 80;

///
/// Nodes whose children are always printed on separate lines
///
const ALWAYS_BREAK: &[&str] = &["Program", "Block"];

///
/// Placeholder printed for an absent optional child
///
const ABSENT: &str = "_";

///
/// A minimal s-expression tree, used as a compact textual form of the AST
///
/// Every AST node is printed as `(NodeName child...)`:
///
/// - children are positional and follow the field order of the AST node
/// - operators, primitive types and booleans are bare atoms (`Add`, `Number`, `true`)
/// - string values are quoted (`"Hello"`)
/// - an absent optional child is printed as `_`, a trailing one is omitted
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr {
    Atom(String),
    Str(String),
    List(Vec<SExpr>),
}

impl SExpr {
    fn atom(value: impl ToString) -> SExpr {
        SExpr::Atom(value.to_string())
    }

    fn node(name: &str, children: Vec<SExpr>) -> SExpr {
        let mut items = Vec::with_capacity(children.len() + 1);
        items.push(SExpr::atom(name));
        items.extend(children);
        SExpr::List(items)
    }

    fn head(&self) -> Option<&str> {
        match self {
            SExpr::List(items) => match items.first() {
                Some(SExpr::Atom(name)) => Some(name),
                _ => None,
            },
            _ => None,
        }
    }

    fn flat_len(&self) -> usize {
        let mut out = String::new();
        self.write_flat(&mut out);
        out.len()
    }

    fn write_flat(&self, out: &mut String) {
        match self {
            SExpr::Atom(value) => out.push_str(value),
            SExpr::Str(value) => write_quoted(value, out),
            SExpr::List(items) => {
                out.push('(');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(' ');
                    }
                    item.write_flat(out);
                }
                out.push(')');
            }
        }
    }

    fn write_pretty(&self, out: &mut String, indent: usize) {
        let SExpr::List(items) = self else {
            return self.write_flat(out);
        };

        let forced = self.head().is_some_and(|head| ALWAYS_BREAK.contains(&head));
        if items.len() <= 1 || (!forced && indent + self.flat_len() <= MAX_WIDTH) {
            return self.write_flat(out);
        }

        // The head and any leading atoms (e.g. operators) stay on the first line
        out.push('(');
        let mut rest = items.iter().peekable();
        if let Some(head) = rest.next() {
            head.write_flat(out);
        }
        while let Some(SExpr::Atom(value)) = rest.peek() {
            if value == ABSENT {
                break;
            }
            out.push(' ');
            out.push_str(value);
            rest.next();
        }

        for item in rest {
            out.push('\n');
            out.push_str(&" ".repeat(indent + 2));
            item.write_pretty(out, indent + 2);
        }
        out.push(')');
    }
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_pretty(&mut out, 0);
        f.write_str(&out)
    }
}

fn write_quoted(value: &str, out: &mut String) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
}

///
/// Convert a statement (usually the root `Program`) to its s-expression form
///
pub fn statement_to_sexpr(statement: &Statement) -> SExpr {
    match statement {
        Statement::Program { body } => SExpr::node("Program", statements(body)),
        Statement::Block { body } => SExpr::node("Block", statements(body)),
        Statement::Empty => SExpr::node("Empty", vec![]),
        Statement::Expression { expression } => {
            SExpr::node("Expression", vec![expression_to_sexpr(expression)])
        }
        Statement::VariableDeclaration { variables } => {
            SExpr::node("VariableDeclaration", expressions(variables))
        }
        Statement::If {
            condition,
            consequent,
            alternative,
        } => {
            let mut children = vec![
                expression_to_sexpr(condition),
                statement_to_sexpr(consequent),
            ];
            children.extend(alternative.as_deref().map(statement_to_sexpr));
            SExpr::node("If", children)
        }
        Statement::While { condition, body } => SExpr::node(
            "While",
            vec![expression_to_sexpr(condition), statement_to_sexpr(body)],
        ),
        Statement::DoWhile { body, condition } => SExpr::node(
            "DoWhile",
            vec![statement_to_sexpr(body), expression_to_sexpr(condition)],
        ),
        Statement::For {
            initializer,
            condition,
            increment,
            body,
        } => SExpr::node(
            "For",
            vec![
                optional(initializer.as_deref().map(statement_to_sexpr)),
                optional(condition.as_deref().map(expression_to_sexpr)),
                optional(increment.as_deref().map(expression_to_sexpr)),
                statement_to_sexpr(body),
            ],
        ),
        Statement::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        } => {
            let parameters = parameters
                .iter()
                .map(|parameter| {
                    SExpr::node(
                        "Parameter",
                        vec![
                            identifier(&parameter.name),
                            type_to_sexpr(&parameter.type_annotation),
                        ],
                    )
                })
                .collect();
            SExpr::node(
                "FunctionDeclaration",
                vec![
                    identifier(name),
                    SExpr::node("Parameters", parameters),
                    type_to_sexpr(return_type),
                    statement_to_sexpr(body),
                ],
            )
        }
        Statement::Return { argument } => SExpr::node(
            "Return",
            argument
                .as_deref()
                .map(expression_to_sexpr)
                .into_iter()
                .collect(),
        ),
        Statement::ClassDeclaration {
            name,
            super_class,
            body,
        } => SExpr::node(
            "ClassDeclaration",
            vec![
                identifier(name),
                optional(super_class.as_deref().map(identifier)),
                statement_to_sexpr(body),
            ],
        ),
    }
}

///
/// Convert an expression to its s-expression form
///
pub fn expression_to_sexpr(expression: &Expression) -> SExpr {
    match expression {
        Expression::Variable {
            identifier: name,
            type_annotation,
            initializer,
        } => {
            let mut children = vec![identifier(name), type_to_sexpr(type_annotation)];
            children.extend(initializer.as_deref().map(expression_to_sexpr));
            SExpr::node("Variable", children)
        }
        Expression::Assignment {
            operator,
            left,
            right,
        } => SExpr::node(
            "Assignment",
            vec![
                SExpr::atom(format!("{:?}", operator)),
                expression_to_sexpr(left),
                expression_to_sexpr(right),
            ],
        ),
        Expression::Binary {
            operator,
            left,
            right,
        } => SExpr::node(
            "Binary",
            vec![
                SExpr::atom(format!("{:?}", operator)),
                expression_to_sexpr(left),
                expression_to_sexpr(right),
            ],
        ),
        Expression::Unary { operator, right } => SExpr::node(
            "Unary",
            vec![
                SExpr::atom(format!("{:?}", operator)),
                expression_to_sexpr(right),
            ],
        ),
        Expression::Logical {
            operator,
            left,
            right,
        } => SExpr::node(
            "Logical",
            vec![
                SExpr::atom(format!("{:?}", operator)),
                expression_to_sexpr(left),
                expression_to_sexpr(right),
            ],
        ),
        Expression::BooleanLiteral { value } => {
            SExpr::node("BooleanLiteral", vec![SExpr::atom(value)])
        }
        Expression::NilLiteral => SExpr::node("NilLiteral", vec![]),
        Expression::StringLiteral { value } => {
            SExpr::node("StringLiteral", vec![SExpr::Str(value.clone())])
        }
        Expression::NumericLiteral { value } => {
            SExpr::node("NumericLiteral", vec![SExpr::atom(value)])
        }
        Expression::Identifier { name } => identifier(name),
        Expression::Member {
            computed,
            object,
            property,
        } => SExpr::node(
            if *computed {
                "ComputedMember"
            } else {
                "Member"
            },
            vec![expression_to_sexpr(object), expression_to_sexpr(property)],
        ),
        Expression::Call { callee, arguments } => {
            let mut children = vec![expression_to_sexpr(callee)];
            children.extend(expressions(arguments));
            SExpr::node("Call", children)
        }
        Expression::This {} => SExpr::node("This", vec![]),
        Expression::Super {} => SExpr::node("Super", vec![]),
        Expression::New { callee, arguments } => {
            let mut children = vec![expression_to_sexpr(callee)];
            children.extend(expressions(arguments));
            SExpr::node("New", children)
        }
    }
}

///
/// Convert a type annotation to its s-expression form
///
pub fn type_to_sexpr(type_annotation: &Type) -> SExpr {
    match type_annotation {
        Type::Primitive(primitive) => SExpr::atom(primitive),
        Type::Array(element) => SExpr::node("Array", vec![type_to_sexpr(element)]),
        Type::Function {
            params,
            return_type,
        } => SExpr::node(
            "Function",
            vec![
                SExpr::node("Parameters", params.iter().map(type_to_sexpr).collect()),
                type_to_sexpr(return_type),
            ],
        ),
        Type::Class { name, super_class } => {
            let mut children = vec![SExpr::atom(name)];
            children.extend(super_class.iter().map(SExpr::atom));
            SExpr::node("Class", children)
        }
        Type::Generic { base, type_args } => {
            let mut children = vec![SExpr::atom(base)];
            children.extend(type_args.iter().map(type_to_sexpr));
            SExpr::node("Generic", children)
        }
        Type::Void => SExpr::atom("Void"),
    }
}

fn statements(list: &[Statement]) -> Vec<SExpr> {
    list.iter().map(statement_to_sexpr).collect()
}

fn expressions(list: &[Expression]) -> Vec<SExpr> {
    list.iter().map(expression_to_sexpr).collect()
}

///
/// A name is printed the same way as an `Identifier` expression
///
fn identifier(name: &str) -> SExpr {
    SExpr::node("Identifier", vec![SExpr::atom(name)])
}

fn optional(value: Option<SExpr>) -> SExpr {
    value.unwrap_or_else(|| SExpr::atom(ABSENT))
}

///
/// Parse s-expression text into a tree. `;` starts a comment that runs to the end of the line.
///
#[cfg(test)]
pub fn parse_sexpr(text: &str) -> Result<SExpr, String> {
    let mut chars = text.chars().peekable();
    let expr = parse_next(&mut chars)?.ok_or("Expected an s-expression, found nothing")?;
    match parse_next(&mut chars)? {
        None => Ok(expr),
        Some(extra) => Err(format!(
            "Unexpected content after the s-expression: {}",
            extra
        )),
    }
}

#[cfg(test)]
fn parse_next(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<Option<SExpr>, String> {
    // Skip whitespace and comments
    loop {
        match chars.peek() {
            Some(ch) if ch.is_whitespace() => {
                chars.next();
            }
            Some(';') => while chars.next_if(|&ch| ch != '\n').is_some() {},
            _ => break,
        }
    }

    match chars.next() {
        None => Ok(None),
        Some(')') => Err("Unexpected ')'".to_string()),
        Some('(') => {
            let mut items = Vec::new();
            loop {
                while chars.next_if(|ch| ch.is_whitespace()).is_some() {}
                match chars.peek() {
                    Some(')') => {
                        chars.next();
                        return Ok(Some(SExpr::List(items)));
                    }
                    None => return Err("Unclosed '('".to_string()),
                    _ => match parse_next(chars)? {
                        Some(item) => items.push(item),
                        None => return Err("Unclosed '('".to_string()),
                    },
                }
            }
        }
        Some('"') => {
            let mut value = String::new();
            loop {
                match chars.next() {
                    Some('"') => return Ok(Some(SExpr::Str(value))),
                    Some('\\') => match chars.next() {
                        Some('n') => value.push('\n'),
                        Some('t') => value.push('\t'),
                        Some(ch @ ('"' | '\\')) => value.push(ch),
                        other => return Err(format!("Invalid string escape: {:?}", other)),
                    },
                    Some(ch) => value.push(ch),
                    None => return Err("Unterminated string".to_string()),
                }
            }
        }
        Some(first) => {
            let mut value = first.to_string();
            while let Some(ch) =
                chars.next_if(|&ch| !ch.is_whitespace() && !matches!(ch, '(' | ')' | '"' | ';'))
            {
                value.push(ch);
            }
            Ok(Some(SExpr::Atom(value)))
        }
    }
}
