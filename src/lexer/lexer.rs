#[derive(Debug, Clone)]
pub struct Lexer {
    index: u32,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer { index: 0 }
    }

    pub fn next_token(&mut self) -> String {
        self.index += 1;
        return String::from("Hello World");
    }
}
