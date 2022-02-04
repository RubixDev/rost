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
                self.next();
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

    fn next(&mut self) {
        self.current_char_index += 1;
        self.current_char = self.input.chars().nth(self.current_char_index);
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
        self.next();
        return Token::new(tok_type, char.to_string());
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
