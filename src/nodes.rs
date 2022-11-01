pub type Program = Vec<Expression>;

#[derive(Clone)]
pub enum Expression {
    Let(LetExpr),
    Add(AddExpr),
}

#[derive(Clone)]
pub struct LetExpr {
    pub name: String,
    pub expr: Box<Expression>,
}

#[derive(Clone)]
pub struct AddExpr {
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
}
#[derive(Clone)]
pub struct Factor {
    pub ops: Vec<TermOperator>,
    pub atom: Atom,
}
#[derive(Clone)]
pub enum Atom {
    Number(f64),
    Ident(String),
    Expression(AddExpr),
}
