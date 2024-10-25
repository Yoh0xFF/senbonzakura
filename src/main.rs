use crate::lexer::Lexer;

mod lexer;

fn main() {
    let mut lexer = Lexer::new();
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
}
