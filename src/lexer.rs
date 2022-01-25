use crate::tokens::{Token, TokenType};

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPACES: [char; 2] = [' ', '\t'];
const SINGLE_CHARS: [char; 7] = ['(', ')', '+', '-', '*', '/', '%'];

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
            if SPACES.contains(&current_char) {
                self.advance();
            } else if SINGLE_CHARS.contains(&current_char) {
                tokens.push(self.make_single_char(current_char));
            } else if DIGITS.contains(&current_char) {
                tokens.push(self.make_number());
            } else {
                panic!("SyntaxError: Illegal character `{}`", current_char);
            }
        }
        tokens.push(Token::new(TokenType::EOF, String::new()));

        return tokens;
    }

    fn advance(&mut self) {
        self.current_char_index += 1;
        self.current_char = self.input.chars().nth(self.current_char_index);
    }

    fn next(&self) -> Option<char> {
        return self.input.chars().nth(self.current_char_index + 1);
    }

    // ---------------------------------------------

    fn make_single_char(&mut self, char: char) -> Token {
        let tok_type = match char {
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Multiply,
            '/' => TokenType::Divide,
            '%' => TokenType::Modulo,
            _ => panic!(),
        };
        self.advance();
        return Token::new(tok_type, char.to_string());
    }

    fn make_number(&mut self) -> Token {
        let mut number = self.current_char.unwrap().to_string();
        self.advance();

        while self.current_char != None && DIGITS.contains(&self.current_char.unwrap()) {
            number.push(self.current_char.unwrap());
            self.advance();
        }

        if let Some(next_char) = self.next() {
            if self.current_char == Some('.') && DIGITS.contains(&next_char) {
                number.push('.');
                self.advance();
                number.push(next_char);
                self.advance();

                while self.current_char != None && DIGITS.contains(&self.current_char.unwrap()) {
                    number.push(self.current_char.unwrap());
                    self.advance();
                }
            }
        }

        return Token::new(TokenType::Number, number);
    }
}
