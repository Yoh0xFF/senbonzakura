use crate::ast::{parse_sexpr, statement_to_sexpr, SExpr};
use crate::parser::{parse_root_statement, Parser};
use std::fs;

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
#[derive(Debug)]
pub(super) struct SexpTestCase {
    description: String,
    source: String,
    expected_ast: SExpr,
}

const HEADER_PREFIX: &str = "===";

fn is_header(line: &str) -> bool {
    line.starts_with(HEADER_PREFIX)
}

fn is_separator(line: &str) -> bool {
    let line = line.trim_end();
    line.len() >= 3 && line.chars().all(|c| c == '-')
}

pub(super) fn execute_sexp_test(test_case: &SexpTestCase) {
    let Ok(mut parser) = Parser::new(&test_case.source) else {
        panic!(
            "Failed to tokenize the test case source:\n{}\n",
            test_case.source
        )
    };

    match parse_root_statement(&mut parser) {
        Ok(ast) => {
            let actual_ast = statement_to_sexpr(&ast);
            assert_eq!(
                actual_ast, test_case.expected_ast,
                "AST mismatch for test case: {}\n\nSource:\n{}\nExpected:\n{}\n\nActual:\n{}\n",
                test_case.description, test_case.source, test_case.expected_ast, actual_ast
            );
        }
        Err(error) => panic!(
            "Failed to parse the test case source: {}\n{}\n",
            error, test_case.source
        ),
    }
}

pub(super) fn parse_sexp_test_cases(content: &str) -> Result<Vec<SexpTestCase>, String> {
    let mut lines = content.lines().enumerate().peekable();
    let mut test_cases = Vec::new();

    // Skip anything before the first header
    while lines.next_if(|(_, line)| !is_header(line)).is_some() {}

    while let Some(&(header_line, _)) = lines.peek() {
        let mut description = Vec::new();
        while let Some((_, line)) = lines.next_if(|(_, line)| is_header(line)) {
            let part = line[HEADER_PREFIX.len()..].trim();
            if !part.is_empty() {
                description.push(part);
            }
        }

        let mut source = String::new();
        loop {
            match lines.next() {
                Some((_, line)) if is_separator(line) => break,
                Some((_, line)) => {
                    source.push_str(line);
                    source.push('\n');
                }
                None => {
                    return Err(format!(
                        "Missing '---' separator in test case at line {}",
                        header_line + 1
                    ))
                }
            }
        }

        let mut expected = String::new();
        while let Some((_, line)) = lines.next_if(|(_, line)| !is_header(line)) {
            expected.push_str(line);
            expected.push('\n');
        }

        let expected_ast = parse_sexpr(&expected).map_err(|error| {
            format!(
                "Invalid expected AST in test case at line {}: {}",
                header_line + 1,
                error
            )
        })?;

        test_cases.push(SexpTestCase {
            description: description.join(" "),
            source,
            expected_ast,
        });
    }

    Ok(test_cases)
}

pub(super) fn load_sexp_test_cases(path: &str) -> Vec<SexpTestCase> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to read the test file: {}", error);
            eprintln!("Path: {}", path);
            panic!("Could not read test file: {}", error);
        }
    };

    let test_cases = match parse_sexp_test_cases(&content) {
        Ok(test_cases) => test_cases,
        Err(error) => {
            eprintln!("Path: {}", path);
            panic!("Could not parse test file: {}", error);
        }
    };

    // Verify that we found some test cases
    if test_cases.is_empty() {
        eprintln!("No test cases found in file: {}", path);
        panic!("File contained no valid test cases");
    }

    test_cases
}

pub(super) fn execute_sexp_test_cases(path: &str) {
    let test_cases = load_sexp_test_cases(path);

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
        test_cases[0].expected_ast.to_string(),
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
}
