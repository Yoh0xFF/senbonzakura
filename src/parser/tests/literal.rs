use super::{Parsed, parse, parse_clean};
use crate::ast::ExprKind;

fn int(parsed: &Parsed) -> i64 {
    match parsed.expr.kind {
        ExprKind::IntLiteral(value) => value,
        ref other => panic!("not an integer literal: {other:?}"),
    }
}

fn float(parsed: &Parsed) -> f64 {
    match parsed.expr.kind {
        ExprKind::FloatLiteral(value) => value,
        ref other => panic!("not a float literal: {other:?}"),
    }
}

fn string(parsed: &Parsed) -> &str {
    match &parsed.expr.kind {
        ExprKind::StringLiteral(value) => value,
        other => panic!("not a string literal: {other:?}"),
    }
}

fn character(parsed: &Parsed) -> char {
    match parsed.expr.kind {
        ExprKind::CharLiteral(value) => value,
        ref other => panic!("not a character literal: {other:?}"),
    }
}

/// The lexer reports what is wrong with a malformed literal, and the parser stays quiet.
fn assert_lexer_reports(parsed: &Parsed, input: &str) {
    assert!(!parsed.lexer_diagnostics.is_empty(), "the lexer reports the problem in {input:?}");
    assert_eq!(parsed.parser_diagnostics, [], "{input:?}");
}

#[test]
fn integers() {
    let cases = [
        ("0", 0),
        ("42", 42),
        ("007", 7),
        ("1_000", 1000),
        ("0x1F", 31),
        ("0XfF", 255),
        ("0xdead_BEEF", 3735928559),
        ("0o17", 15),
        ("0b1010", 10),
        ("0b_1", 1),
        ("9223372036854775807", 9223372036854775807),
        ("0x7FFF_FFFF_FFFF_FFFF", 9223372036854775807),
    ];
    for (input, value) in cases {
        let parsed = parse_clean(input);

        assert_eq!(int(&parsed), value, "{input:?}");
        assert_eq!(parsed.expr_text(), input);
    }
}

#[test]
fn integer_too_large_is_reported() {
    for input in ["9223372036854775808", "0x8000_0000_0000_0000", "99999999999999999999"] {
        let parsed = parse(input);

        assert_eq!(int(&parsed), 0, "{input:?}");
        assert_eq!(parsed.parser_messages(), ["integer literal is too large"]);
    }
}

#[test]
fn floats() {
    let cases = [
        ("1.5", 1.5),
        ("0.0", 0.0),
        ("1_0.5_0", 10.5),
        ("1e10", 1e10),
        ("1E+4", 1e4),
        ("1.5e-3", 0.0015),
    ];
    for (input, value) in cases {
        let parsed = parse_clean(input);

        assert_eq!(float(&parsed), value, "{input:?}");
        assert_eq!(parsed.expr_text(), input);
    }
}

#[test]
fn float_out_of_range_is_reported() {
    let parsed = parse("1e999");

    assert_eq!(float(&parsed), 0.0);
    assert_eq!(parsed.parser_messages(), ["float literal is out of range"]);
}

#[test]
fn booleans() {
    assert_eq!(parse_clean("true").expr.kind, ExprKind::BoolLiteral(true));
    assert_eq!(parse_clean("false").expr.kind, ExprKind::BoolLiteral(false));
}

#[test]
fn strings() {
    let cases = [
        ("\"\"", ""),
        ("\"hello\"", "hello"),
        ("\"a\\tb\"", "a\tb"),
        ("\"q\\\"q\"", "q\"q"),
        ("\"\\\\\"", "\\"),
        ("`multi\nline`", "multi\nline"),
        ("`no \\`break`", "no `break"),
    ];
    for (input, value) in cases {
        let parsed = parse_clean(input);

        assert_eq!(string(&parsed), value, "{input:?}");
        assert_eq!(parsed.expr_text(), input);
    }
}

#[test]
fn all_simple_escapes() {
    let parsed = parse_clean("\"\\n\\t\\r\\0\\\\\\\"\\'\\`\"");

    assert_eq!(string(&parsed), "\n\t\r\0\\\"'`");
}

#[test]
fn unicode_escapes() {
    let parsed = parse_clean("\"\\u{41}\\u{e9}\\u{1F600}\"");

    assert_eq!(string(&parsed), "Aé😀");
}

#[test]
fn backslash_before_line_break_joins_lines() {
    for input in ["`a\\\nb`", "`a\\\r\nb`", "`a\\\rb`"] {
        assert_eq!(string(&parse_clean(input)), "ab", "{input:?}");
    }
}

#[test]
fn characters() {
    let cases = [
        ("'a'", 'a'),
        ("'\\n'", '\n'),
        ("'\\''", '\''),
        ("'é'", 'é'),
        ("'😀'", '😀'),
        ("'\\u{1F600}'", '😀'),
    ];
    for (input, value) in cases {
        let parsed = parse_clean(input);

        assert_eq!(character(&parsed), value, "{input:?}");
        assert_eq!(parsed.expr_text(), input);
    }
}

#[test]
fn malformed_strings_get_best_effort_values() {
    let cases = [
        ("\"abc", "abc"),
        ("\"abc\\", "abc"),
        ("`abc", "abc"),
        ("\"a\\qb\"", "aqb"),
        ("\"\\u41\"", "\u{FFFD}41"),
        ("\"\\u{41\"", "\u{FFFD}"),
        ("\"\\u{D800}\"", "\u{FFFD}"),
        ("\"\\u{110000}\"", "\u{FFFD}"),
        ("\"\\u{}\"", "\u{FFFD}"),
    ];
    for (input, value) in cases {
        let parsed = parse(input);

        assert_eq!(string(&parsed), value, "{input:?}");
        assert_lexer_reports(&parsed, input);
    }
}

#[test]
fn malformed_characters_become_the_replacement_character() {
    for input in ["''", "'ab'", "'"] {
        let parsed = parse(input);

        assert_eq!(character(&parsed), '\u{FFFD}', "{input:?}");
        assert_lexer_reports(&parsed, input);
    }
}

#[test]
fn malformed_integers_get_best_effort_values() {
    for (input, value) in [("0x", 0), ("0b102", 2), ("1abc", 1), ("0x1g", 1)] {
        let parsed = parse(input);

        assert_eq!(int(&parsed), value, "{input:?}");
        assert_lexer_reports(&parsed, input);
    }
}

#[test]
fn malformed_floats_get_best_effort_values() {
    for (input, value) in [("1e", 0.0), ("1.5x", 1.5), ("2e+", 0.0)] {
        let parsed = parse(input);

        assert_eq!(float(&parsed), value, "{input:?}");
        assert_lexer_reports(&parsed, input);
    }
}
