use super::{Parser, parse};
use crate::ast::{Expr, ExprKind, sexpr};
use crate::lexer::{Lexer, SourceFile};

#[test]
fn parse_file_returns_every_statement_in_order() {
    let source = SourceFile::new("test", "var x = 1;\nfn f() {}\nf(x);");
    let mut parser = Parser::new(Lexer::new(&source));

    let statements = parser.parse_file();

    let trees: Vec<_> = statements.iter().map(sexpr::stmt).collect();
    assert_eq!(trees, ["(var x 1)", "(fn f () {})", "(call f x);"]);
    assert!(parser.diagnostics().is_empty());
    assert!(parser.at_end());
}

#[test]
fn parse_file_of_an_empty_or_comment_only_file_is_empty() {
    for text in ["", "// x\n/* y */"] {
        let source = SourceFile::new("test", text);
        assert_eq!(Parser::new(Lexer::new(&source)).parse_file(), []);
    }
}

#[test]
fn parse_file_keeps_going_after_problems() {
    let source = SourceFile::new("test", ") x = ; } y;");
    let mut parser = Parser::new(Lexer::new(&source));

    let statements = parser.parse_file();

    assert_eq!(sexpr::stmt(statements.last().unwrap()), "y;");
    let messages: Vec<_> = parser.diagnostics().iter().map(|d| d.message.as_str()).collect();
    assert_eq!(
        messages,
        [
            "expected expression, found ')'",
            "expected expression, found ';'",
            "expected expression, found '}'",
        ]
    );
}

#[test]
fn non_literal_is_reported_and_becomes_an_error_node() {
    let parsed = parse(":");

    assert_eq!(parsed.expr, Expr::new(ExprKind::Error, 0, 1));
    assert_eq!(parsed.parser_messages(), ["expected expression, found ':'"]);
}

#[test]
fn empty_input_is_reported_at_the_end() {
    let parsed = parse("  ");

    assert_eq!(parsed.expr, Expr::new(ExprKind::Error, 2, 2));
    assert_eq!(parsed.parser_messages(), ["expected expression, found end of input"]);
}

#[test]
fn unknown_character_is_left_to_the_lexer() {
    let parsed = parse("#");

    assert_eq!(parsed.expr, Expr::new(ExprKind::Error, 0, 1));
    let messages: Vec<_> = parsed.lexer_diagnostics.iter().map(|d| d.message.as_str()).collect();
    assert_eq!(messages, ["unexpected character"]);
    assert_eq!(parsed.parser_diagnostics, []);
}

#[test]
fn stray_close_paren_is_reported_and_skipped() {
    let parsed = parse(")");

    assert_eq!(parsed.expr, Expr::new(ExprKind::Error, 0, 0));
    assert_eq!(parsed.parser_messages(), ["expected expression, found ')'"]);
}

#[test]
fn parses_one_expression_per_call() {
    let source = SourceFile::new("test", "1 \"two\" 3.0 true");
    let mut parser = Parser::new(Lexer::new(&source));

    let exprs: Vec<_> = (0..4).map(|_| parser.parse_expression()).collect();

    assert!(parser.at_end());
    assert_eq!(
        exprs,
        [
            Expr::new(ExprKind::IntLiteral(1), 0, 1),
            Expr::new(ExprKind::StringLiteral("two".to_string()), 2, 7),
            Expr::new(ExprKind::FloatLiteral(3.0), 8, 11),
            Expr::new(ExprKind::BoolLiteral(true), 12, 16),
        ]
    );
}
