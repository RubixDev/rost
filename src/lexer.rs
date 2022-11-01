use crate::{
    error::{ErrorKind, Result},
    tokens::{Token, TokenType},
};
use std::str::Chars;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPACES: [char; 2] = [' ', '\t'];
const SINGLE_CHARS: [char; 8] = ['(', ')', '+', '-', '*', '/', '=', ';'];

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        return Lexer {
            input: input.chars(),
            current_char: None,
        };
    }

    pub fn scan(&mut self) -> Result<Vec<Token>> {
        self.advance();
        let mut tokens = vec![];

        while let Some(current_char) = self.current_char {
            if SPACES.contains(&current_char) {
                self.advance();
            } else if SINGLE_CHARS.contains(&current_char) {
                tokens.push(self.make_single_char(current_char));
            } else if DIGITS.contains(&current_char) {
                tokens.push(self.make_number());
            } else if current_char.is_ascii_alphabetic() || current_char == '_' {
                tokens.push(self.make_name());
            } else {
                error!(
                    ErrorKind::SyntaxError,
                    "Illegal character `{}`", current_char
                );
            }
        }
        tokens.push(Token::new(TokenType::Eof, String::from("EOF")));

        Ok(tokens)
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    fn next(&self) -> Option<char> {
        self.input.clone().next()
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
            '=' => TokenType::Assign,
            ';' => TokenType::Semicolon,
            _ => panic!(), // will never happen
        };
        self.advance();
        Token::new(tok_type, char.to_string())
    }

    fn make_number(&mut self) -> Token {
        let mut number = self.current_char.unwrap().to_string();
        self.advance();

        while self.current_char != None && DIGITS.contains(&self.current_char.unwrap())
            || self.current_char == Some('_')
        {
            if self.current_char != Some('_') {
                number.push(self.current_char.unwrap());
            }
            self.advance();
        }

        if let Some(next_char) = self.next() {
            if self.current_char == Some('.') && DIGITS.contains(&next_char) {
                number.push('.');
                self.advance();
                number.push(next_char);
                self.advance();

                while self.current_char != None && DIGITS.contains(&self.current_char.unwrap())
                    || self.current_char == Some('_')
                {
                    if self.current_char != Some('_') {
                        number.push(self.current_char.unwrap());
                    }
                    self.advance();
                }
            }
        }

        Token::new(TokenType::Number, number)
    }

    fn make_name(&mut self) -> Token {
        let mut name = self.current_char.unwrap().to_string();
        self.advance();

        while let Some(char) = self.current_char {
            if !char.is_ascii_alphabetic() && !char.is_ascii_digit() && char != '_' {
                break;
            }
            name.push(char);
            self.advance();
        }

        Token::new(
            match name.as_str() {
                "let" => TokenType::Let,
                _ => TokenType::Ident,
            },
            name,
        )
    }
}
