use super::internal_util::execute_yaml_test_cases;

#[test]
fn test_lexer_parse_comments() {
    execute_yaml_test_cases("tests/lexer/test_comment_tokens.yaml");
}

#[test]
fn test_keyword_tokens() {
    execute_yaml_test_cases("tests/lexer/test_keyword_tokens.yaml");
}

#[test]
fn test_literal_tokens() {
    execute_yaml_test_cases("tests/lexer/test_literal_tokens.yaml");
}

#[test]
fn test_mixed_tokens() {
    execute_yaml_test_cases("tests/lexer/test_mixed_tokens.yaml");
}

#[test]
fn test_operator_keyword_tokens() {
    execute_yaml_test_cases("tests/lexer/test_operator_keyword_tokens.yaml");
}

#[test]
fn test_punctuation_tokens() {
    execute_yaml_test_cases("tests/lexer/test_punctuation_tokens.yaml");
}

#[test]
fn test_type_keyword_tokens() {
    execute_yaml_test_cases("tests/lexer/test_type_keyword_tokens.yaml");
}
