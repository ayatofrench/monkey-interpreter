use std::io::{self, Write};

use crate::{lexer::Lexer, token::TokenType};

const PROMPT: &str = ">>";

pub fn start() {
    let mut input = String::new();

    loop {
        io::stdout().write(b">>").unwrap();
        // print!("{} ", PROMPT);

        io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(&input);
        let mut tok = lexer.next_token();

        while tok.token_type != TokenType::EOF {
            println!("{:?}", tok);

            tok = lexer.next_token();
        }
    }
}
