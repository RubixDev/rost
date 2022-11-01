use crate::{
    error::{ErrorKind, Result},
    nodes::*,
    tokens::{Token, TokenType},
};
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser<'a> {
        return Parser {
            tokens: tokens.iter(),
            current_token: Token::new(TokenType::Eof, String::from("EOF")),
        };
    }

    pub fn parse(&mut self) -> Result<Program> {
        self.advance();
        let program = self.program()?;
        if self.current_token.token_type != TokenType::Eof {
            error!(
                ErrorKind::SyntaxError,
                "Expected end of file, got `{}`", self.current_token.value
            );
        }
        Ok(program)
    }

    fn advance(&mut self) {
        self.current_token = self
            .tokens
            .next()
            .unwrap_or(&Token::new(TokenType::Eof, String::from("EOF")))
            .clone();
    }

    // ------------------------------
    fn program(&mut self) -> Result<Program> {
        let mut exprs = vec![];
        if self.current_token.token_type != TokenType::Eof {
            exprs.push(self.expression()?);

            while self.current_token.token_type == TokenType::Semicolon {
                self.advance();
                exprs.push(self.expression()?);
            }
        }
        Ok(exprs)
    }

    fn expression(&mut self) -> Result<Expression> {
        match self.current_token.token_type {
            TokenType::Let => Ok(Expression::Let(self.let_expr()?)),
            _ => Ok(Expression::Add(self.add_expr()?)),
        }
    }

    fn let_expr(&mut self) -> Result<LetExpr> {
        self.advance();

        if self.current_token.token_type != TokenType::Ident {
            error!(
                ErrorKind::SyntaxError,
                "Expected identifier, got `{}`", self.current_token.value
            );
        }
        let name = self.current_token.value.clone();
        self.advance();

        if self.current_token.token_type != TokenType::Assign {
            error!(
                ErrorKind::SyntaxError,
                "Expected `=`, got `{}`", self.current_token.value
            );
        }
        self.advance();

        let expr = Box::new(self.expression()?);

        Ok(LetExpr { name, expr })
    }

    fn add_expr(&mut self) -> Result<AddExpr> {
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

        Ok(AddExpr {
            term: Box::new(term),
            following,
        })
    }

    fn term(&mut self) -> Result<Term> {
        let factor = self.factor()?;

        let mut following = vec![];
        loop {
            let operator = match self.current_token.token_type {
                TokenType::Multiply => FactorOperator::Multiply,
                TokenType::Divide => FactorOperator::Divide,
                _ => break,
            };
            self.advance();
            following.push((operator, self.factor()?));
        }

        Ok(Term { factor, following })
    }

    fn factor(&mut self) -> Result<Factor> {
        let mut ops = vec![];
        loop {
            ops.push(match self.current_token.token_type {
                TokenType::Plus => TermOperator::Plus,
                TokenType::Minus => TermOperator::Minus,
                _ => break,
            });
            self.advance();
        }

        Ok(Factor {
            ops,
            atom: self.atom()?,
        })
    }

    fn atom(&mut self) -> Result<Atom> {
        if self.current_token.token_type == TokenType::LParen {
            self.advance();
            let expression = self.add_expr()?;
            if self.current_token.token_type != TokenType::RParen {
                error!(
                    ErrorKind::SyntaxError,
                    "Expected `)`, got `{}`", self.current_token.value
                );
            }
            self.advance();
            return Ok(Atom::Expression(expression));
        }

        if self.current_token.token_type == TokenType::Ident {
            let ident = self.current_token.value.clone();
            self.advance();
            return Ok(Atom::Ident(ident));
        }

        let num = match self.current_token.value.parse::<f64>() {
            Ok(num) => num,
            Err(_) => error!(
                ErrorKind::SyntaxError,
                "Expected number, got `{}`", self.current_token.value
            ),
        };
        self.advance();
        Ok(Atom::Number(num))
    }
}
