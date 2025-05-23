use super::internal_util::execute_yaml_test_cases;

#[test]
fn test_lexer_parse_comments() {
    execute_yaml_test_cases("tests/lexer/test_comment_tokens.yaml");
}

#[test]
fn test_number_tokens() {
    execute_yaml_test_cases("tests/lexer/test_number_tokens.yaml");
}

#[test]
fn test_string_tokens() {
    execute_yaml_test_cases("tests/lexer/test_string_tokens.yaml");
}
