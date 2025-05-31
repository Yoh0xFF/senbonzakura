// use crate::parser::parse_root_statement;
use lexer::{Lexer, Token};
use parser::{parse_root_statement, Parser};

mod ast;
mod lexer;
mod parser;

fn main() {
    let mut parser = Parser::new(
        r#"
        class Point {
            def constructor(x: number, y: number) {
                this.x = x;
                this.y = y;
            }
        }

        let p: Point = new Point(10, 20);
        "#,
    );
    let ast = parse_root_statement(&mut parser);
    println!("Yaml:\n{}", serde_yaml::to_string(&ast).unwrap());
}
