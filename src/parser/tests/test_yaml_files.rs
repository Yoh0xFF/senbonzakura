use super::internal_util::execute_yaml_test_cases;

#[test]
fn test_expression_parse_assignment() {
    execute_yaml_test_cases("tests/parser/test_expression_parse_assignment.yaml");
}

#[test]
fn test_expression_parse_literals() {
    execute_yaml_test_cases("tests/parser/test_expression_parse_literals.yaml");
}

#[test]
fn test_statement_parse_variable_declaration() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_variable_declaration.yaml");
}
