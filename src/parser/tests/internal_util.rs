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
    let mut parser = Parser::new(&test_case.source);
    let actual_ast = parse_root_statement(&mut parser);

    // Convert Box<Statement> to Statement for comparison
    let actual_statement = *actual_ast;

    assert_eq!(actual_statement, test_case.expected_ast);
}

pub(super) fn load_yaml_test_cases(path: &str) -> Vec<YamlTestCase> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to read the test yaml file: {}", error);
            eprintln!("Path: {}", path);
            panic!("Could not read test file: {}", error);
        }
    };

    // Parse multiple documents
    let mut test_cases = Vec::new();

    // Use the serde_yaml::from_reader function with a stream
    for document in serde_yaml::Deserializer::from_str(&content) {
        match YamlTestCase::deserialize(document) {
            Ok(test_case) => test_cases.push(test_case),
            Err(e) => {
                eprintln!("Failed to parse a YAML document: {}", e);
                eprintln!("Path: {}", path);
                panic!("Could not parse YAML content: {}", e);
            }
        }
    }

    // Verify that we found some test cases
    if test_cases.is_empty() {
        eprintln!("No test cases found in file: {}", path);
        panic!("File contained no valid test cases");
    }

    test_cases
}

pub(super) fn execute_yaml_test_cases(path: &str) {
    let test_cases = load_yaml_test_cases(path);

    for (index, test_case) in test_cases.iter().enumerate() {
        println!("Testing case #{}:\n{}\n", index + 1, test_case.description);
        execute_yaml_test(test_case);
    }

    println!("All {} test cases passed!", test_cases.len());
}
