use bigdecimal::BigDecimal;
use crate::{nodes::{Expression, Term, Factor}, tokens::TokenType};

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
                TokenType::Plus => { base += other },
                TokenType::Minus => { base -= other },
                _ => panic!(),
            }
        }

        return base;
    }

    fn visit_term(&self, node: &Term) -> BigDecimal {
        let mut base = self.visit_factor(&node.factor);

        for (operator, factor) in &node.following {
            let other = self.visit_factor(&factor);
            match operator {
                TokenType::Multiply => { base *= other },
                TokenType::Divide => { base = base / other },
                TokenType::Modulo => { base = base % other },
                _ => panic!(),
            }
        }

        return base;
    }

    fn visit_factor(&self, node: &Factor) -> BigDecimal {
        return match node {
            Factor::Unary(operator, factor) => {
                let base = self.visit_factor(factor);
                match operator {
                    TokenType::Plus => base,
                    TokenType::Minus => -base,
                    _ => panic!(),
                }
            },
            Factor::Expression(expression) => self.visit_expression(expression),
            Factor::Number(number) => number.clone(),
        };
    }
}
