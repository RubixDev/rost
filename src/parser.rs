use rust_decimal::Decimal;
use crate::{tokens::{Token, TokenType}, nodes::{Expression, TermOperator, Term, FactorOperator, Factor, Power, Atom}};

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    current_token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let first_token = tokens[0].clone();
        return Parser {
            tokens: tokens,
            current_token: first_token,
            current_token_index: 0,
        }
    }

    pub fn parse(&mut self) -> Expression {
        let expression = self.expression();
        if self.current_token.token_type != TokenType::EOF {
            panic!("SyntaxError: Expected end of file");
        }
        return expression;
    }

    fn advance(&mut self) {
        self.current_token_index += 1;
        self.current_token = self.tokens
            .get(self.current_token_index)
            .unwrap_or(&Token::new(TokenType::EOF, String::new()))
            .clone();
    }

    // ------------------------------

    fn expression(&mut self) -> Expression {
        let term = self.term();

        let mut following = vec![];
        loop {
            let operator = match self.current_token.token_type {
                TokenType::Plus => TermOperator::Plus,
                TokenType::Minus => TermOperator::Minus,
                _ => break,
            };
            self.advance();
            following.push((operator, self.term()));
        }

        return Expression { term: Box::new(term), following };
    }

    fn term(&mut self) -> Term {
        let factor = self.factor();

        let mut following = vec![];
        loop {
            let operator = match self.current_token.token_type {
                TokenType::Multiply => FactorOperator::Multiply,
                TokenType::Divide => FactorOperator::Divide,
                TokenType::Modulo => FactorOperator::Modulo,
                TokenType::IntDivide => FactorOperator::IntDivide,
                _ => break,
            };
            self.advance();
            following.push((operator, self.factor()));
        }

        return Term { factor, following };
    }

    fn factor(&mut self) -> Factor {
        let operator = match self.current_token.token_type {
            TokenType::Plus => TermOperator::Plus,
            TokenType::Minus => TermOperator::Minus,
            _ => { return Factor::Power(Box::new(self.power())); },
        };
        self.advance();
        return Factor::Unary(operator, Box::new(self.factor()));
    }

    fn power(&mut self) -> Power {
        let base = self.atom();

        let mut exponent = None;
        if self.current_token.token_type == TokenType::Power {
            self.advance();
            exponent = Some(self.factor());
        }

        return Power { base, exponent };
    }

    fn atom(&mut self) -> Atom {
        if self.current_token.token_type == TokenType::LParen {
            self.advance();
            let expression = self.expression();
            if self.current_token.token_type != TokenType::RParen {
                panic!("SyntaxError: Expected `)`, got `{}`", self.current_token.value);
            }
            self.advance();
            return Atom::Expression(expression);
        }

        let num = self.current_token.value.parse::<Decimal>().unwrap();
        self.advance();
        return Atom::Number(num);
    }
}
