use crate::ast::{parse_sexpr, statement_to_sexpr, SExpr};
use crate::parser::{parse_root_statement, Parser, ParserError};
use crate::test_files::{load_test_cases, split_test_cases, RawTestCase};

///
/// A single parser test case loaded from a `.sexp` test file
///
/// Test files contain one or more cases in the following layout:
///
/// ```text
/// === Description of the test
/// === that may continue on further `===` lines
/// source code to parse;
/// ---
/// (Program
///   (Expression ...))
/// ```
///
/// A case starts with one or more lines prefixed with `===`, which are joined into its
/// description. The source runs until a line of three or more `-`, and the expected AST
/// runs until the next header. Anything before the first header is ignored.
/// Inside the expected AST, `;` starts a comment that runs to the end of the line.
///
/// A case that should fail to parse gives the expected error instead of an AST:
///
/// ```text
/// === Missing expression
/// x = ;
/// ---
/// (Error ParserError "Unexpected token: StatementEnd, expected an expression")
/// ```
///
#[derive(Debug)]
pub(super) struct SexpTestCase {
    description: String,
    source: String,
    expected: SExpr,
}

///
/// Convert a parser error into the `(Error <Variant> "<message>")` form used by test files
///
fn parser_error_to_sexpr(error: &ParserError) -> SExpr {
    let (variant, message) = match error {
        ParserError::LexicalError { message } => ("LexicalError", message),
        ParserError::ParserError { message } => ("ParserError", message),
        ParserError::TypeError { message } => ("TypeError", message),
        ParserError::SemanticError { message } => ("SemanticError", message),
    };

    SExpr::List(vec![
        SExpr::Atom("Error".to_string()),
        SExpr::Atom(variant.to_string()),
        SExpr::Str(message.clone()),
    ])
}

pub(super) fn execute_sexp_test(test_case: &SexpTestCase) {
    let Ok(mut parser) = Parser::new(&test_case.source) else {
        panic!(
            "Failed to tokenize the test case source:\n{}\n",
            test_case.source
        )
    };

    let actual = match parse_root_statement(&mut parser) {
        Ok(ast) => statement_to_sexpr(&ast),
        Err(error) => parser_error_to_sexpr(&error),
    };

    assert_eq!(
        actual, test_case.expected,
        "Mismatch for test case: {}\n\nSource:\n{}\nExpected:\n{}\n\nActual:\n{}\n",
        test_case.description, test_case.source, test_case.expected, actual
    );
}

fn parse_sexp_test_case(test_case: RawTestCase) -> Result<SexpTestCase, String> {
    let expected = parse_sexpr(&test_case.expected).map_err(|error| {
        format!(
            "Invalid expected s-expression in test case at line {}: {}",
            test_case.line, error
        )
    })?;

    Ok(SexpTestCase {
        description: test_case.description,
        source: test_case.source,
        expected,
    })
}

pub(super) fn parse_sexp_test_cases(content: &str) -> Result<Vec<SexpTestCase>, String> {
    split_test_cases(content)?
        .into_iter()
        .map(parse_sexp_test_case)
        .collect()
}

pub(super) fn execute_sexp_test_cases(path: &str) {
    let test_cases = load_test_cases(path, parse_sexp_test_case);

    for (index, test_case) in test_cases.iter().enumerate() {
        println!("Testing case #{}:\n{}\n", index + 1, test_case.description);
        execute_sexp_test(test_case);
    }

    println!("All {} test cases passed!", test_cases.len());
}

#[test]
fn test_sexp_test_file_format() {
    let content = "\
leading notes are ignored
=== First
1;
---
(Program
  ; a comment
  (Expression (NumericLiteral 1)))

=== Second case
===   spanning two lines
s = \"a  b\";
---
(Program (Expression (Assignment Assign (Identifier s) (StringLiteral \"a  b\"))))
";
    let test_cases = parse_sexp_test_cases(content).unwrap();
    assert_eq!(test_cases.len(), 2);
    assert_eq!(test_cases[0].description, "First");
    assert_eq!(test_cases[0].source, "1;\n");
    assert_eq!(test_cases[1].description, "Second case spanning two lines");
    assert_eq!(
        test_cases[0].expected.to_string(),
        "(Program\n  (Expression (NumericLiteral 1)))"
    );
    for test_case in &test_cases {
        execute_sexp_test(test_case);
    }
}

#[test]
fn test_sexp_string_escapes_round_trip() {
    let expected = SExpr::List(vec![
        SExpr::Atom("StringLiteral".to_string()),
        SExpr::Str("quote \" backslash \\ newline \n tab \t".to_string()),
    ]);
    assert_eq!(parse_sexpr(&expected.to_string()), Ok(expected));
}

#[test]
fn test_sexp_test_file_format_errors() {
    assert!(parse_sexp_test_cases("=== A\n1;\n").is_err());
    assert!(parse_sexp_test_cases("=== A\n1;\n---\n").is_err());
    assert!(parse_sexp_test_cases("=== A\n1;\n---\n(Program").is_err());
    assert!(parse_sexp_test_cases("=== A\n1;\n---\n(Program))").is_err());
    assert!(parse_sexp_test_cases(
        "=== A\n1;\n---\n(Program (Expression (NumericLiteral 1)) ; trailing\n)"
    )
    .is_ok());
}
