use crate::parser::parse_root_statement;
use ast::Statement;
use lexer::{Lexer, Token};
use parser::Parser;
use visitor_s_expression::ToSExpression;

mod ast;
mod lexer;
mod parser;
mod visitor_s_expression;

fn main() {
    let mut lexer = Lexer::new(r#"12 17 "Hello"   'world' "#);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);

    let mut parser = Parser::new(
        r#"
        class Person {
            def constructor(name: string, surname: string) {
                this.name = name;
                this.surname = surname;
            }

            def getName(): string {
                return this.name + " " + this.surname;
            }
        }

        class Student extends Person {
            def constructor(name: string, surname: string, university: string) {
                super(name, surname);
                this.university = university;
            }

            def getInfo(): string {
                return super.getName() + ", " + this.university;
            }
        }

        def add(x: number, y: number): number {
            return x + y;
        }

        add(2, 5);
        "#,
    );
    let ast = parse_root_statement(&mut parser);
    let s_expression = ast.to_pretty_s_expression().unwrap();
    println!("SExpression:\n{}", s_expression);
}
