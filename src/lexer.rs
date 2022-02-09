use crate::tokens::{Token, TokenType};

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct Lexer {
    input: String,
    current_char: Option<char>,
    current_char_index: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let first_char = input.chars().nth(0);
        return Lexer {
            input,
            current_char: first_char,
            current_char_index: 0,
        };
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(current_char) = self.current_char {
            match current_char {
                ' ' | '\t' | '\r' => self.next(),
                '(' => tokens.push(self.make_single_char(TokenType::LParen,   "(")),
                ')' => tokens.push(self.make_single_char(TokenType::RParen,   ")")),
                '+' => tokens.push(self.make_single_char(TokenType::Plus,     "+")),
                '-' => tokens.push(self.make_single_char(TokenType::Minus,    "-")),
                '*' => tokens.push(self.make_single_char(TokenType::Multiply, "*")),
                '/' => tokens.push(self.make_single_char(TokenType::Divide,   "/")),
                '%' => tokens.push(self.make_single_char(TokenType::Modulo,   "%")),
                _ => {
                    if DIGITS.contains(&current_char) {
                        tokens.push(self.make_number());
                    } else {
                        panic!("SyntaxError: Illegal character `{}`", current_char);
                    }
                }
            }
        }
        tokens.push(Token::new(TokenType::EOF, String::new()));

        return tokens;
    }

    fn next(&mut self) {
        self.current_char_index += 1;
        self.current_char = self.input.chars().nth(self.current_char_index);
    }

    // ---------------------------------------------

    fn make_single_char(&mut self, token_type: TokenType, value: &str) -> Token {
        self.next();
        return Token::new(token_type, value.to_string());
    }

    fn make_number(&mut self) -> Token {
        let mut number = String::new();
        number.push(self.current_char.unwrap());
        self.next();

        while self.current_char != None && DIGITS.contains(&self.current_char.unwrap()) {
            number.push(self.current_char.unwrap());
            self.next();
        }

        if self.current_char == Some('.') {
            number.push('.');
            self.next();
            if self.current_char != None && DIGITS.contains(&self.current_char.unwrap()) {
                number.push(self.current_char.unwrap());
                self.next();

                while self.current_char != None && DIGITS.contains(&self.current_char.unwrap()) {
                    number.push(self.current_char.unwrap());
                    self.next();
                }
            } else {
                panic!("SyntaxError: Expected digit after decimal point");
            }
        }

        return Token::new(TokenType::Number, number);
    }
}
