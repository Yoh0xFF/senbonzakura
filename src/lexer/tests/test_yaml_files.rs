use super::internal_util::execute_yaml_test_cases;

#[test]
fn test_lexer_parse_comments() {
    execute_yaml_test_cases("tests/lexer/test_comment_tokens.yaml");
}
