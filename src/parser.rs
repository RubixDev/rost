use bigdecimal::BigDecimal;
use crate::{tokens::{Token, TokenType}, nodes::{Expression, Term, Factor}};

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    current_token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let first_token = tokens[0].clone();
        return Parser {
            tokens,
            current_token: first_token,
            current_token_index: 0,
        }
    }

    pub fn parse(&mut self) -> Expression {
        let expression = self.expression();
        if self.current_token.token_type != TokenType::EOF
            { panic!("SyntaxError: Expected end of file"); }
        return expression;
    }

    fn next(&mut self) {
        self.current_token_index += 1;
        self.current_token = self.tokens
            .get(self.current_token_index)
            .unwrap_or(&Token::new(TokenType::EOF, String::new()))
            .clone();
    }

    fn expression(&mut self) -> Expression {
        let term = self.term();

        let mut following = vec![];
        while [
            TokenType::Plus,
            TokenType::Minus,
        ].contains(&self.current_token.token_type) {
            let operator = self.current_token.token_type.clone();
            self.next();
            following.push((operator, self.term()));
        }

        return Expression { term: Box::new(term), following };
    }

    fn term(&mut self) -> Term {
        let factor = self.factor();

        let mut following = vec![];
        while [
            TokenType::Multiply,
            TokenType::Divide,
            TokenType::Modulo,
        ].contains(&self.current_token.token_type) {
            let operator = self.current_token.token_type.clone();
            self.next();
            following.push((operator, self.factor()));
        }

        return Term { factor, following };
    }

    fn factor(&mut self) -> Factor {
        if self.current_token.token_type == TokenType::LParen {
            self.next();
            let expression = self.expression();
            if self.current_token.token_type != TokenType::RParen
                { panic!("SyntaxError: Expected `)`, got `{}`", self.current_token.value); }
            self.next();
            return Factor::Expression(expression);
        }

        if self.current_token.token_type == TokenType::Number {
            let num = Factor::Number(self.current_token.value.parse::<BigDecimal>().unwrap());
            self.next();
            return num;
        }

        if [
            TokenType::Plus,
            TokenType::Minus,
        ].contains(&self.current_token.token_type) {
            let operator = self.current_token.token_type.clone();
            self.next();
            return Factor::Unary(operator, Box::new(self.factor()));
        }

        panic!("SyntaxError: Expected expression");
    }
}
