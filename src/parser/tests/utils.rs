use crate::ast::Statement;
use crate::parser::{parse_root_statement, Parser};
use crate::ToSExpression;
// use anyhow::Result;
use serde::Deserialize;
use std::fs;

pub(super) fn execute(source: &str, expected_s_expression: &str) {
    let mut parser = Parser::new(source);

    let ast = parse_root_statement(&mut parser);
    let actual_s_expression = ast
        .to_s_expression()
        .expect("Failed to convert AST to s-expression");

    // Normalize expected s-expression by removing indentation
    let mut normalized_expected_s_expression = expected_s_expression
        .replace('\n', " ") // Replace newlines with spaces
        .split_whitespace() // Split by whitespace
        .collect::<Vec<&str>>()
        .join(" "); // Join with single spaces
    normalized_expected_s_expression = normalized_expected_s_expression.replace(" )", ")");

    assert_eq!(actual_s_expression, normalized_expected_s_expression);
}

#[derive(Debug, Deserialize)]
pub(super) struct YamlTestCase {
    description: String,
    source: String,
    expected_ast: Statement,
}

pub(super) fn execute_yaml_test(test_case: &YamlTestCase) {
    println!("Testing: {}", test_case.description);

    let mut parser = Parser::new(&test_case.source);
    let actual_ast = parse_root_statement(&mut parser);

    // Convert Box<Statement> to Statement for comparison
    let actual_statement = *actual_ast;

    assert_eq!(actual_statement, test_case.expected_ast);
}

pub(super) fn load_yaml_test(path: &str) -> YamlTestCase {
    let result = fs::read_to_string(path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to read the test yaml file: {}", error);
            eprintln!("Path: {}", path);

            panic!("Could not read test file: {}", error);
        }
    };

    let result: serde_yaml::Result<YamlTestCase> = serde_yaml::from_str(&content);
    let test_case: YamlTestCase = match result {
        Ok(test_case) => test_case,
        Err(error) => {
            eprintln!("Failed to parse the test yaml file: {}", error);
            eprintln!("Path: {}", path);
            eprintln!("Content:\n{}", content);

            panic!("Could not parse YAML content: {}", error);
        }
    };

    test_case
}

pub(super) fn execute_yaml_test_file(path: &str) {
    let test_case = load_yaml_test(path);
    execute_yaml_test(&test_case);
}
