use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    LParen,    // '('
    RParen,    // ')'
    Plus,      // '+'
    Minus,     // '-'
    Multiply,  // '*'
    Divide,    // '/'
    Modulo,    // '%'
    IntDivide, // '\'
    Power,     // '^'

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
        return Token { token_type, value }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {:?} | {:?} )", self.token_type, self.value)
    }
}
