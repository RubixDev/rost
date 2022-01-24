use rust_decimal::{Decimal, MathematicalOps};
use crate::nodes::{Expression, TermOperator, Term, FactorOperator, Factor, Power, Atom};

pub struct Interpreter {
    start_node: Expression,
}

impl Interpreter {
    pub fn new(nodes: Expression) -> Self {
        return Interpreter {
            start_node: nodes,
        }
    }

    pub fn run(&self) {
        println!("{}", self.visit_expression(&self.start_node.clone()));
    }

    // --------------------------------------

    fn visit_expression(&self, node: &Expression) -> Decimal {
        let mut base = self.visit_term(&*node.term);

        for (operator, term) in &node.following {
            let other = self.visit_term(&term);
            match operator {
                TermOperator::Plus => { base += other },
                TermOperator::Minus => { base -= other },
            }
        }

        return base;
    }

    fn visit_term(&self, node: &Term) -> Decimal {
        let mut base = self.visit_factor(&node.factor);

        for (operator, factor) in &node.following {
            let other = self.visit_factor(&factor);
            match operator {
                FactorOperator::Multiply => { base *= other },
                FactorOperator::Divide => { base /= other },
                FactorOperator::Modulo => { base %= other },
                FactorOperator::IntDivide => { base = (base / other).floor() },
            }
        }

        return base;
    }

    fn visit_factor(&self, node: &Factor) -> Decimal {
        return match node {
            Factor::Unary(operator, factor) => {
                let base = self.visit_factor(factor);
                match operator {
                    TermOperator::Plus => base,
                    TermOperator::Minus => -base,
                }
            },
            Factor::Power(power) => self.visit_power(power),
        };
    }

    fn visit_power(&self, node: &Power) -> Decimal {
        let base = self.visit_atom(&node.base);
        if let Some(exponent) = &node.exponent {
            return base.powd(self.visit_factor(&exponent));
        }
        return base;
    }

    fn visit_atom(&self, node: &Atom) -> Decimal {
        return match node {
            Atom::Number(value) => *value,
            Atom::Expression(expression) => self.visit_expression(expression),
        };
    }
}
