use super::utils::execute_yaml_test_file;

#[test]
fn test_variable_declaration_with_assignment() {
    execute_yaml_test_file("tests/parser/variable_declaration_assignment.yaml");
}
