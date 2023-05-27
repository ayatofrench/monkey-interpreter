use phf::phf_map;

// pub type TokenType = String;
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT(String),
    INT(String),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NotEq,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "fn" => Token::FUNCTION,
    "let" => Token::LET,
    "true" => Token::TRUE,
    "false" => Token::FALSE,
    "if" => Token::IF,
    "else" => Token::ELSE,
    "return" => Token::RETURN,
};

// pub fn lookup_ident(ident: &str) -> Token {
//     KEYWORDS.get(ident).cloned().unwrap_or(Token::IDENT)
// }

// impl Token {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Token::ILLEGAL => "ILLEGAL",
//             Token::EOF => "EOF",
//             Token::IDENT => "IDENT",
//             Token::INT => "INT",
//             Token::ASSIGN => "=",
//             Token::PLUS => "+",
//             Token::MINUS => "-",
//             Token::BANG => "!",
//             Token::ASTERISK => "*",
//             Token::SLASH => "/",
//             Token::LT => "<",
//             Token::GT => ">",
//             Token::EQ => "==",
//             Token::NotEq => "!=",
//             Token::COMMA => ",",
//             Token::SEMICOLON => ";",
//             Token::LPAREN => "(",
//             Token::RPAREN => ")",
//             Token::LBRACE => "{",
//             Token::RBRACE => "}",
//             Token::FUNCTION => "FUNCTION",
//             Token::LET => "LET",
//             Token::TRUE => "TRUE",
//             Token::FALSE => "FALSE",
//             Token::IF => "IF",
//             Token::ELSE => "ELSE",
//             Token::RETURN => "RETURN",
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Token {
//     pub token_type: Token,
//     pub literal: String,
// }
//
// impl Token {
//     pub fn new(token_type: Token, literal: String) -> Self {
//         return Self {
//             token_type,
//             literal,
//         };
//     }
// }
