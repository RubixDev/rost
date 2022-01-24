use bigdecimal::BigDecimal;
use crate::nodes::{Expression, TermOperator, Term, FactorOperator, Factor};

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

    fn visit_expression(&self, node: &Expression) -> BigDecimal {
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

    fn visit_term(&self, node: &Term) -> BigDecimal {
        let mut base = self.visit_factor(&node.factor);

        for (operator, factor) in &node.following {
            let other = self.visit_factor(&factor);
            match operator {
                FactorOperator::Multiply => { base *= other },
                FactorOperator::Divide => { base = base / other },
                FactorOperator::Modulo => { base = base % other },
            }
        }

        return base;
    }

    fn visit_factor(&self, node: &Factor) -> BigDecimal {
        return match node {
            Factor::Unary(operator, factor) => {
                let base = self.visit_factor(factor);
                match operator {
                    TermOperator::Plus => base,
                    TermOperator::Minus => -base,
                }
            },
            Factor::Expression(expression) => self.visit_expression(expression),
            Factor::Number(number) => number.clone(),
        };
    }
}
