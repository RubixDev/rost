use bigdecimal::BigDecimal;

#[derive(Clone)]
pub struct Expression {
    pub term: Box<Term>,
    pub following: Vec<(TermOperator, Term)>,
}
#[derive(Clone)]
pub enum TermOperator {
    Plus,
    Minus,
}
#[derive(Clone)]
pub struct Term {
    pub factor: Factor,
    pub following: Vec<(FactorOperator, Factor)>,
}
#[derive(Clone)]
pub enum FactorOperator {
    Multiply,
    Divide,
    Modulo,
}
#[derive(Clone)]
pub enum Factor {
    Unary(TermOperator, Box<Factor>),
    Expression(Expression),
    Number(BigDecimal),
}
