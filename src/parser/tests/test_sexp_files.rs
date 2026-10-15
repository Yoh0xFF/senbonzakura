use super::internal_util::execute_sexp_test_cases;

#[test]
fn test_expression_parse_assignment() {
    execute_sexp_test_cases("tests/parser/test_expression_parse_assignment.sexp");
}

#[test]
fn test_expression_parse_binary() {
    execute_sexp_test_cases("tests/parser/test_expression_parse_binary.sexp");
}

#[test]
fn test_expression_parse_left_hand_side() {
    execute_sexp_test_cases("tests/parser/test_expression_parse_left_hand_side.sexp");
}

#[test]
fn test_expression_parse_literals() {
    execute_sexp_test_cases("tests/parser/test_expression_parse_literals.sexp");
}

#[test]
fn test_expression_parse_primary() {
    execute_sexp_test_cases("tests/parser/test_expression_parse_primary.sexp");
}

#[test]
fn test_expression_parse_relational_and_logical() {
    execute_sexp_test_cases("tests/parser/test_expression_parse_relational_and_logical.sexp");
}

#[test]
fn test_expression_parse_unary() {
    execute_sexp_test_cases("tests/parser/test_expression_parse_unary.sexp");
}

#[test]
fn test_statement_parse_block() {
    execute_sexp_test_cases("tests/parser/test_statement_parse_block.sexp");
}

#[test]
fn test_statement_parse_class_declaration() {
    execute_sexp_test_cases("tests/parser/test_statement_parse_class_declaration.sexp");
}

#[test]
fn test_statement_parse_conditional_if() {
    execute_sexp_test_cases("tests/parser/test_statement_parse_conditional_if.sexp");
}

#[test]
fn test_statement_parse_function_declaration() {
    execute_sexp_test_cases("tests/parser/test_statement_parse_function_declaration.sexp");
}

#[test]
fn test_statement_parse_loop_do_while() {
    execute_sexp_test_cases("tests/parser/test_statement_parse_loop_do_while.sexp");
}

#[test]
fn test_statement_parse_loop_for() {
    execute_sexp_test_cases("tests/parser/test_statement_parse_loop_for.sexp");
}

#[test]
fn test_statement_parse_loop_while() {
    execute_sexp_test_cases("tests/parser/test_statement_parse_loop_while.sexp");
}

#[test]
fn test_statement_parse_variable_declaration() {
    execute_sexp_test_cases("tests/parser/test_statement_parse_variable_declaration.sexp");
}
