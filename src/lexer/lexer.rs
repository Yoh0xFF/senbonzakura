use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Lexer {
    index: Cell<u32>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            index: Cell::new(0),
        }
    }

    pub fn next_token(&self) -> String {
        self.increment();
        return String::from("Hello World");
    }

    fn increment(&self) {
        self.index.set(self.index.get() + 1);
    }

    // fn index(&self) -> u32 {
    //     return self.index.get();
    // }
}
