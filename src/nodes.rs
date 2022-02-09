use bigdecimal::BigDecimal;
use crate::tokens::TokenType;

#[derive(Clone)]
pub struct Expression {
    pub term: Box<Term>,
    pub following: Vec<(TokenType, Term)>,
}
#[derive(Clone)]
pub struct Term {
    pub factor: Factor,
    pub following: Vec<(TokenType, Factor)>,
}
#[derive(Clone)]
pub enum Factor {
    Unary(TokenType, Box<Factor>),
    Expression(Expression),
    Number(BigDecimal),
}
