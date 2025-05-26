use std::fs;

use serde::Deserialize;

use crate::{lexer::TokenType, Lexer, Token};

#[derive(Debug, Deserialize)]
pub(super) struct YamlTestCase {
    description: String,
    source: String,
    expected_tokens: Vec<Token>,
}

pub(super) fn execute_yaml_test(test_case: &YamlTestCase) {
    let mut lexer = Lexer::new(&test_case.source);

    let mut actual_tokens: Vec<Token> = vec![];
    let mut token = match lexer.next_token() {
        Ok(token) => token,
        Err(error) => panic!("Failed to get next token: {:?}", error),
    };
    while token.token_type != TokenType::End {
        actual_tokens.push(token);
        token = match lexer.next_token() {
            Ok(token) => token,
            Err(error) => panic!("Failed to get next token: {:?}", error),
        };
    }

    assert_eq!(actual_tokens, test_case.expected_tokens);
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
