use std::rc::Rc;

use crate::{ast::Expression, Statement};

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
        Rc::new(Statement::Program {
            body: Rc::new(vec![Rc::new(Statement::Block {
                body: Rc::new(vec![
                    Rc::new(Statement::Expression {
                        expression: Rc::new(Expression::StringLiteral(
                            "Hello".to_string(),
                        )),
                    }),
                    Rc::new(Statement::Expression {
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
        Rc::new(Statement::Program {
            body: Rc::new(vec![Rc::new(Statement::Block {
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
        Rc::new(Statement::Program {
            body: Rc::new(vec![Rc::new(Statement::Block {
                body: Rc::new(vec![
                    Rc::new(Statement::Expression {
                        expression: Rc::new(Expression::StringLiteral(
                            "Hello".to_string(),
                        )),
                    }),
                    Rc::new(Statement::Block {
                        body: Rc::new(vec![Rc::new(Statement::Expression {
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
        Rc::new(Statement::Program {
            body: Rc::new(vec![Rc::new(Statement::Block {
                body: Rc::new(vec![Rc::new(Statement::Empty)]),
            })]),
        }),
    )
}
