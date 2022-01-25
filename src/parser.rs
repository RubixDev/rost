use rust_decimal::Decimal;
use std::slice::Iter;
use crate::{tokens::{Token, TokenType}, nodes::{Expression, TermOperator, Term, FactorOperator, Factor, Power, Atom}, error::{Result, ErrorKind}};

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current_token: Token,
}

impl <'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        return Parser {
            tokens: tokens.iter(),
            current_token: Token::new(TokenType::EOF, String::from("EOF")),
        }
    }

    pub fn parse(&mut self) -> Result<Expression> {
        self.advance();
        let expression = self.expression()?;
        if self.current_token.token_type != TokenType::EOF {
            error!(ErrorKind::SyntaxError, "Expected end of file, got `{}`", self.current_token.value);
        }
        return Ok(expression);
    }

    fn advance(&mut self) {
        self.current_token = self.tokens
            .next()
            .unwrap_or(&Token::new(TokenType::EOF, String::from("EOF")))
            .clone();
    }

    // ------------------------------

    fn expression(&mut self) -> Result<Expression> {
        let term = self.term()?;

        let mut following = vec![];
        loop {
            let operator = match self.current_token.token_type {
                TokenType::Plus => TermOperator::Plus,
                TokenType::Minus => TermOperator::Minus,
                _ => break,
            };
            self.advance();
            following.push((operator, self.term()?));
        }

        return Ok(Expression { term: Box::new(term), following });
    }

    fn term(&mut self) -> Result<Term> {
        let factor = self.factor()?;

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
            following.push((operator, self.factor()?));
        }

        return Ok(Term { factor, following });
    }

    fn factor(&mut self) -> Result<Factor> {
        let operator = match self.current_token.token_type {
            TokenType::Plus => TermOperator::Plus,
            TokenType::Minus => TermOperator::Minus,
            _ => { return Ok(Factor::Power(Box::new(self.power()?))); },
        };
        self.advance();
        return Ok(Factor::Unary(operator, Box::new(self.factor()?)));
    }

    fn power(&mut self) -> Result<Power> {
        let base = self.atom()?;

        let mut exponent = None;
        if self.current_token.token_type == TokenType::Power {
            self.advance();
            exponent = Some(self.factor()?);
        }

        return Ok(Power { base, exponent });
    }

    fn atom(&mut self) -> Result<Atom> {
        if self.current_token.token_type == TokenType::LParen {
            self.advance();
            let expression = self.expression()?;
            if self.current_token.token_type != TokenType::RParen {
                error!(ErrorKind::SyntaxError, "Expected `)`, got `{}`", self.current_token.value);
            }
            self.advance();
            return Ok(Atom::Expression(expression));
        }

        let num = match self.current_token.value.parse::<Decimal>() {
            Ok(num) => num,
            Err(_) => error!(ErrorKind::SyntaxError, "Expected number, got `{}`", self.current_token.value),
        };
        self.advance();
        return Ok(Atom::Number(num));
    }
}
