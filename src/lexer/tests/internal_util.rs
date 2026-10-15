use crate::ast::{parse_sexpr, SExpr};
use crate::test_files::{load_test_cases, split_test_cases, RawTestCase};
use crate::{lexer::TokenType, Lexer, Token};

///
/// A single lexer test case loaded from a `.tokens` test file
///
/// Test files share the case layout of the parser `.sexp` files, see `RawTestCase`.
/// The expected output lists one token per line, without the final `End` token:
///
/// ```text
/// === Number and identifier
/// 12 abc
/// ---
/// Number     1:1-1:3  0..2  "12"
/// Identifier 1:4-1:7  3..6  "abc"
/// ```
///
/// Each line holds the token type, its start and end as `line:column`, its byte offsets
/// as `start..end`, and the source text it covers as a quoted string.
/// Blank lines are ignored, and `;` starts a comment that runs to the end of the line.
///
#[derive(Debug)]
pub(super) struct TokensTestCase {
    description: String,
    source: String,
    expected: Vec<SExpr>,
}

///
/// Convert a token into the `<Type> <line>:<column>-<line>:<column> <start>..<end> "<text>"`
/// form used by test files
///
fn token_to_sexpr(token: &Token, source: &str) -> SExpr {
    SExpr::List(vec![
        SExpr::Atom(token.token_type.to_string()),
        SExpr::Atom(format!(
            "{}:{}-{}:{}",
            token.start.line, token.start.column, token.end.line, token.end.column
        )),
        SExpr::Atom(format!("{}..{}", token.start.offset, token.end.offset)),
        SExpr::Str(token.text(source).to_string()),
    ])
}

///
/// Print token s-expressions one per line, without the surrounding parentheses
///
fn format_token_lines(tokens: &[SExpr]) -> String {
    tokens
        .iter()
        .map(|token| {
            let text = token.to_string();
            text[1..text.len() - 1].to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn execute_tokens_test(test_case: &TokensTestCase) {
    let mut lexer = Lexer::new(&test_case.source);

    let mut actual = vec![];
    loop {
        let token = match lexer.next_token() {
            Ok(token) => token,
            Err(error) => panic!(
                "Failed to get next token in test case: {}\n{:?}",
                test_case.description, error
            ),
        };
        if token.token_type == TokenType::End {
            break;
        }
        actual.push(token_to_sexpr(&token, &test_case.source));
    }

    assert_eq!(
        actual,
        test_case.expected,
        "Mismatch for test case: {}\n\nSource:\n{}\nExpected:\n{}\n\nActual:\n{}\n",
        test_case.description,
        test_case.source,
        format_token_lines(&test_case.expected),
        format_token_lines(&actual)
    );
}

fn parse_tokens_test_case(test_case: RawTestCase) -> Result<TokensTestCase, String> {
    let mut expected = Vec::new();
    for (index, line) in test_case.expected.lines().enumerate() {
        let invalid = |error: String| {
            format!(
                "Invalid expected token at line {}: {}",
                test_case.expected_line + index,
                error
            )
        };

        // Wrap the line in a list, keeping a trailing comment away from the closing parenthesis
        let token = match parse_sexpr(&format!("(\n{}\n)", line)).map_err(invalid)? {
            SExpr::List(items) => items,
            other => return Err(invalid(format!("Expected a token, found {}", other))),
        };

        match token.as_slice() {
            [] => {}
            [SExpr::Atom(_), SExpr::Atom(_), SExpr::Atom(_), SExpr::Str(_)] => {
                expected.push(SExpr::List(token))
            }
            _ => {
                return Err(invalid(format!(
                    "Expected `<Type> <line>:<column>-<line>:<column> <start>..<end> \"<text>\"`, found `{}`",
                    line.trim()
                )))
            }
        }
    }

    Ok(TokensTestCase {
        description: test_case.description,
        source: test_case.source,
        expected,
    })
}

pub(super) fn parse_tokens_test_cases(content: &str) -> Result<Vec<TokensTestCase>, String> {
    split_test_cases(content)?
        .into_iter()
        .map(parse_tokens_test_case)
        .collect()
}

pub(super) fn execute_tokens_test_cases(path: &str) {
    let test_cases = load_test_cases(path, parse_tokens_test_case);

    for (index, test_case) in test_cases.iter().enumerate() {
        println!("Testing case #{}:\n{}\n", index + 1, test_case.description);
        execute_tokens_test(test_case);
    }

    println!("All {} test cases passed!", test_cases.len());
}

#[test]
fn test_tokens_test_file_format() {
    let content = "\
=== Number and identifier
12 abc
---
Number     1:1-1:3  0..2  \"12\"  ; the number
; a comment line

Identifier 1:4-1:7  3..6  \"abc\"
";
    let test_cases = parse_tokens_test_cases(content).unwrap();
    assert_eq!(test_cases.len(), 1);
    assert_eq!(test_cases[0].expected.len(), 2);
    assert_eq!(
        format_token_lines(&test_cases[0].expected),
        "Number 1:1-1:3 0..2 \"12\"\nIdentifier 1:4-1:7 3..6 \"abc\""
    );
    execute_tokens_test(&test_cases[0]);
}

#[test]
fn test_tokens_test_file_format_errors() {
    assert!(parse_tokens_test_cases("=== A\n1\n").is_err());
    assert!(parse_tokens_test_cases("=== A\n1\n---\nNumber 1:1-1:2 0..1\n").is_err());
    assert!(parse_tokens_test_cases("=== A\n1\n---\nNumber 1:1-1:2 0..1 \"1\" extra\n").is_err());
    assert!(parse_tokens_test_cases("=== A\n1\n---\n(Number 1:1-1:2 0..1 \"1\")\n").is_err());
    assert!(parse_tokens_test_cases("=== A\n1\n---\nNumber 1:1-1:2 0..1 \"1\n").is_err());
}
