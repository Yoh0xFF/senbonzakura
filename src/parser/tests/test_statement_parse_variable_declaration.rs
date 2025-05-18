use super::utils::execute_yaml_test_cases;

#[test]
fn test_statement_parse_variable_declaration() {
    execute_yaml_test_cases("tests/parser/variable_declaration.yaml");
}
