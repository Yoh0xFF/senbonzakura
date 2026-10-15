use super::internal_util::execute_tokens_test_cases;

#[test]
fn test_lexer_parse_comments() {
    execute_tokens_test_cases("tests/lexer/test_comment_tokens.tokens");
}

#[test]
fn test_keyword_tokens() {
    execute_tokens_test_cases("tests/lexer/test_keyword_tokens.tokens");
}

#[test]
fn test_literal_tokens() {
    execute_tokens_test_cases("tests/lexer/test_literal_tokens.tokens");
}

#[test]
fn test_mixed_tokens() {
    execute_tokens_test_cases("tests/lexer/test_mixed_tokens.tokens");
}

#[test]
fn test_operator_keyword_tokens() {
    execute_tokens_test_cases("tests/lexer/test_operator_keyword_tokens.tokens");
}

#[test]
fn test_punctuation_tokens() {
    execute_tokens_test_cases("tests/lexer/test_punctuation_tokens.tokens");
}

#[test]
fn test_type_keyword_tokens() {
    execute_tokens_test_cases("tests/lexer/test_type_keyword_tokens.tokens");
}
