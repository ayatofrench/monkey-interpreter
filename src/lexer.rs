use crate::token::Token;
use anyhow::Result;

#[derive(Default)]
pub struct Lexer {
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };

        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let tok: Token;
        tok = match self.ch {
            b'=' => match self.peek_char() {
                b'=' => {
                    self.read_char();

                    Token::EQ
                }
                _ => Token::ASSIGN,
            },
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'!' => match self.peek_char() {
                b'=' => {
                    self.read_char();

                    Token::NotEq
                }
                _ => Token::BANG,
            },
            b'*' => Token::ASTERISK,
            b'/' => Token::SLASH,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0 => Token::EOF,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();

                return Ok(match ident.as_str() {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    "true" => Token::TRUE,
                    "false" => Token::FALSE,
                    "if" => Token::IF,
                    "else" => Token::ELSE,
                    "return" => Token::RETURN,
                    _ => Token::IDENT(ident),
                });
            }
            b'0'..=b'9' => return Ok(Token::INT(self.read_number())),
            _ => Token::ILLEGAL,
        };

        self.read_char();
        return Ok(tok);
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let input_slice = &self.input[position..self.position];

        return String::from_utf8_lossy(input_slice).to_string();
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};
    use anyhow::Result;

    #[test]
    fn test_next_token() -> Result<()> {
        const INPUT: &str = "=+(){},;";
        let mut lexer = Lexer::new(INPUT.into());

        let tokens = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;

            assert_eq!(token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn test_next_complete() -> Result<()> {
        const INPUT: &str = r#"let five = 5;
            let ten = 10;
            
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        "#;

        let tests = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::INT(String::from("5")),
            Token::LT,
            Token::INT(String::from("10")),
            Token::GT,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(String::from("5")),
            Token::LT,
            Token::INT(String::from("10")),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(String::from("10")),
            Token::EQ,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::INT(String::from("10")),
            Token::NotEq,
            Token::INT(String::from("9")),
            Token::SEMICOLON,
            Token::EOF,
        ];

        // Add Lexer init here
        let mut lexer = Lexer::new(INPUT.into());

        for token in tests {
            let tok = lexer.next_token()?;

            assert_eq!(token, tok);
        }

        Ok(())
    }
}
