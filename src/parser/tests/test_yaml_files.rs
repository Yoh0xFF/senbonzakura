use super::internal_util::execute_yaml_test_cases;

#[test]
fn test_expression_parse_assignment() {
    execute_yaml_test_cases("tests/parser/test_expression_parse_assignment.yaml");
}

#[test]
fn test_expression_parse_binary() {
    execute_yaml_test_cases("tests/parser/test_expression_parse_binary.yaml");
}

#[test]
fn test_expression_parse_literals() {
    execute_yaml_test_cases("tests/parser/test_expression_parse_literals.yaml");
}

#[test]
fn test_statement_parse_block() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_block.yaml");
}

#[test]
fn test_statement_parse_class_declaration() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_class_declaration.yaml");
}

#[test]
fn test_statement_parse_conditional_if() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_conditional_if.yaml");
}

#[test]
fn test_statement_parse_function_declaration() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_function_declaration.yaml");
}

#[test]
fn test_statement_parse_loop_do_while() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_loop_do_while.yaml");
}

#[test]
fn test_statement_parse_loop_for() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_loop_for.yaml");
}

#[test]
fn test_statement_parse_loop_while() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_loop_while.yaml");
}

#[test]
fn test_statement_parse_variable_declaration() {
    execute_yaml_test_cases("tests/parser/test_statement_parse_variable_declaration.yaml");
}
