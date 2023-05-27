use anyhow::Result;
use std::io::{self, Write};

use crate::{lexer::Lexer, token::Token};

const PROMPT: &str = ">>";

pub fn start() -> Result<()> {
    let mut input = String::new();

    loop {
        io::stdout().write(b">>").unwrap();
        // print!("{} ", PROMPT);

        io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(input.clone().into());
        let mut tok = lexer.next_token()?;

        while tok != Token::EOF {
            println!("{:?}", tok);

            tok = lexer.next_token()?;
        }
    }
}
