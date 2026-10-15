use crate::parser::{parse_root_statement, Parser, ParserError};

fn parse_error(source: &str) -> ParserError {
    let mut parser = Parser::new(source).expect("Failed to tokenize the test source");
    match parse_root_statement(&mut parser) {
        Ok(ast) => panic!("Expected a parse error for {:?}, got: {:?}", source, ast),
        Err(error) => error,
    }
}

#[test]
fn test_missing_expression_is_an_error() {
    for source in ["x = ;", "let x: number = ;", "1 + ;", "foo(1, );", "a[];", ";;)"] {
        let error = parse_error(source);
        assert!(
            matches!(error, ParserError::ParserError { .. }),
            "Unexpected error variant for {:?}: {:?}",
            source,
            error
        );
        assert!(
            error.to_string().contains("expected an expression"),
            "Unexpected error message for {:?}: {}",
            source,
            error
        );
    }
}
