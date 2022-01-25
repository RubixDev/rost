#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    LParen,   // '('
    RParen,   // ')'
    Plus,     // '+'
    Minus,    // '-'
    Multiply, // '*'
    Divide,   // '/'
    Modulo,   // '%'

    Number,

    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        return Token {
            token_type,
            value,
        }
    }
}
