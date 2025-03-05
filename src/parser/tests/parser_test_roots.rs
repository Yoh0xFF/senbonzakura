use std::rc::Rc;

use crate::Expression;

use super::parser_tests::execute;

#[test]
fn test_block_statement() {
    execute(
        r#"
        {
            "Hello";
            17;
        }
        "#,
        Rc::new(Expression::Program {
            body: Rc::new(vec![Rc::new(Expression::BlockStatement {
                body: Rc::new(vec![
                    Rc::new(Expression::ExpressionStatement {
                        expression: Rc::new(Expression::StringLiteral("Hello".to_string())),
                    }),
                    Rc::new(Expression::ExpressionStatement {
                        expression: Rc::new(Expression::NumericLiteral(17)),
                    }),
                ]),
            })]),
        }),
    )
}

#[test]
fn test_empty_block_statement() {
    execute(
        r#"
        {
        }
        "#,
        Rc::new(Expression::Program {
            body: Rc::new(vec![Rc::new(Expression::BlockStatement {
                body: Rc::new(vec![]),
            })]),
        }),
    )
}

#[test]
fn test_nested_block_statement() {
    execute(
        r#"
        {
            "Hello";
            {
                17;
            }
        }
        "#,
        Rc::new(Expression::Program {
            body: Rc::new(vec![Rc::new(Expression::BlockStatement {
                body: Rc::new(vec![
                    Rc::new(Expression::ExpressionStatement {
                        expression: Rc::new(Expression::StringLiteral("Hello".to_string())),
                    }),
                    Rc::new(Expression::BlockStatement {
                        body: Rc::new(vec![Rc::new(Expression::ExpressionStatement {
                            expression: Rc::new(Expression::NumericLiteral(17)),
                        })]),
                    }),
                ]),
            })]),
        }),
    )
}

#[test]
fn test_empty_statement() {
    execute(
        r#"
        {
            ;
        }
        "#,
        Rc::new(Expression::Program {
            body: Rc::new(vec![Rc::new(Expression::BlockStatement {
                body: Rc::new(vec![Rc::new(Expression::EmptyStatement)]),
            })]),
        }),
    )
}
