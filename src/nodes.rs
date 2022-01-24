use rust_decimal::Decimal;

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
    IntDivide,
}
#[derive(Clone)]
pub enum Factor {
    Unary(TermOperator, Box<Factor>),
    Power(Box<Power>),
}
#[derive(Clone)]
pub struct Power {
    pub base: Atom,
    pub exponent: Option<Factor>,
}
#[derive(Clone)]
pub enum Atom {
    Number(Decimal),
    Expression(Expression),
}
