use crate::token::{self, Token, TokenType};

// might want to change this to options but for now I think this is fine..
#[derive(Default)]
struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.to_string(),
            ..Default::default()
        };

        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = &[self.ch];
        let utf8_ch = std::str::from_utf8(ch).unwrap();

        let tok: Token;
        tok = match utf8_ch {
            "=" => Token::new(TokenType::ASSIGN, String::from("=")),
            ";" => Token::new(TokenType::SEMICOLON, String::from(";")),
            "(" => Token::new(TokenType::LPAREN, String::from("(")),
            ")" => Token::new(TokenType::RPAREN, String::from(")")),
            "," => Token::new(TokenType::COMMA, String::from(",")),
            "+" => Token::new(TokenType::PLUS, String::from("+")),
            "{" => Token::new(TokenType::LBRACE, String::from("{")),
            "}" => Token::new(TokenType::RBRACE, String::from("}")),
            "\0" => Token::new(TokenType::EOF, String::from("")),
            _ => {
                if self.is_letter(utf8_ch) {
                    let literal = self.read_identifier();
                    return Token::new(token::lookup_ident(literal), literal.to_string());
                } else if self.is_digit(utf8_ch) {
                    return Token::new(TokenType::INT, self.read_number());
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch.to_string())
                }
            }
        };

        self.read_char();

        return tok;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.is_digit(std::str::from_utf8(&[self.ch]).unwrap()) {
            self.read_char();
        }

        let input_slice = &self.input.as_bytes()[position..self.position];

        return String::from_utf8(input_slice.to_vec()).unwrap();
    }

    fn is_digit(&self, ch: &str) -> bool {
        return "0" <= ch && ch <= "9";
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;

        while self.is_letter(std::str::from_utf8(&[self.ch]).unwrap()) {
            self.read_char();
        }

        let input_slice = &self.input.as_bytes()[position..self.position];

        return std::str::from_utf8(input_slice).unwrap();
    }

    fn is_letter(&self, ch: &str) -> bool {
        return ("a" <= ch) && (ch <= "z") || ("A" <= ch) && (ch <= "Z") || ch == "_";
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn test_next_token() {
        // const INPUT: &str = "=+(){},;";
        const INPUT: &str = r#"let five = 5;
            let ten = 10;
            
            let add = fn(x, y) {
                x + y;
            };
        "#;

        struct TestToken {
            expected_token: TokenType,
            expected_literal: String,
        }

        let tests: [TestToken; 27] = [
            TestToken {
                expected_token: TokenType::LET,
                expected_literal: String::from("let"),
            },
            TestToken {
                expected_token: TokenType::IDENT,
                expected_literal: String::from("five"),
            },
            TestToken {
                expected_token: TokenType::ASSIGN,
                expected_literal: String::from("="),
            },
            TestToken {
                expected_token: TokenType::INT,
                expected_literal: String::from("5"),
            },
            TestToken {
                expected_token: TokenType::SEMICOLON,
                expected_literal: String::from(";"),
            },
            TestToken {
                expected_token: TokenType::LET,
                expected_literal: String::from("let"),
            },
            TestToken {
                expected_token: TokenType::IDENT,
                expected_literal: String::from("ten"),
            },
            TestToken {
                expected_token: TokenType::ASSIGN,
                expected_literal: String::from("="),
            },
            TestToken {
                expected_token: TokenType::INT,
                expected_literal: String::from("10"),
            },
            TestToken {
                expected_token: TokenType::SEMICOLON,
                expected_literal: String::from(";"),
            },
            TestToken {
                expected_token: TokenType::LET,
                expected_literal: String::from("let"),
            },
            TestToken {
                expected_token: TokenType::IDENT,
                expected_literal: String::from("add"),
            },
            TestToken {
                expected_token: TokenType::ASSIGN,
                expected_literal: String::from("="),
            },
            TestToken {
                expected_token: TokenType::FUNCTION,
                expected_literal: String::from("fn"),
            },
            TestToken {
                expected_token: TokenType::LPAREN,
                expected_literal: String::from("("),
            },
            TestToken {
                expected_token: TokenType::IDENT,
                expected_literal: String::from("x"),
            },
            TestToken {
                expected_token: TokenType::COMMA,
                expected_literal: String::from(","),
            },
            TestToken {
                expected_token: TokenType::IDENT,
                expected_literal: String::from("y"),
            },
            TestToken {
                expected_token: TokenType::RPAREN,
                expected_literal: String::from(")"),
            },
            TestToken {
                expected_token: TokenType::LBRACE,
                expected_literal: String::from("{"),
            },
            TestToken {
                expected_token: TokenType::IDENT,
                expected_literal: String::from("x"),
            },
            TestToken {
                expected_token: TokenType::PLUS,
                expected_literal: String::from("+"),
            },
            TestToken {
                expected_token: TokenType::IDENT,
                expected_literal: String::from("y"),
            },
            TestToken {
                expected_token: TokenType::SEMICOLON,
                expected_literal: String::from(";"),
            },
            TestToken {
                expected_token: TokenType::RBRACE,
                expected_literal: String::from("}"),
            },
            TestToken {
                expected_token: TokenType::SEMICOLON,
                expected_literal: String::from(";"),
            },
            TestToken {
                expected_token: TokenType::EOF,
                expected_literal: String::from(""),
            },
        ];

        // Add Lexer init here
        let mut lexer = Lexer::new(INPUT);

        for token in tests {
            let tok = lexer.next_token();

            assert_eq!(token.expected_token, tok.token_type);
            assert_eq!(token.expected_literal, tok.literal);
        }
    }
}
