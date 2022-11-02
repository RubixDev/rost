mod instructions;
mod types;

use std::collections::HashMap;

use crate::nodes::*;

fn as_uleb128(num: u64) -> Vec<u8> {
    let mut buf = vec![];
    leb128::write::unsigned(&mut buf, num).expect("Should write number to vec");
    buf
}

pub struct Compiler {
    tree: Option<Program>,
    symbols: HashMap<String, Vec<u8>>,

    type_section: Vec<u8>,
    function_section: Vec<u8>,
    export_section: Vec<u8>,
    code_section: Vec<u8>,
}

impl Compiler {
    pub fn new(nodes: Program) -> Self {
        Self {
            tree: Some(nodes),
            symbols: HashMap::new(),
            type_section: vec![],
            function_section: vec![],
            export_section: vec![],
            code_section: vec![],
        }
    }

    pub fn compile(mut self) -> Vec<u8> {
        self.type_section.extend_from_slice(&[
            1, // count 1
            types::FUNC,
            0,          // num of params
            1,          // num of return vals
            types::F64, // type of return val (f64)
        ]);
        self.function_section.extend_from_slice(&[
            1, // count 1
            0, // index of type in type_section
        ]);
        self.export_section.extend_from_slice(
            &[
                &[
                    1, // count 1
                    4, // string len
                ][..],
                b"main", // name of export
                &[
                    0, // export kind (0 = func)
                    0, // index of func in function_section
                ],
            ]
            .concat(),
        );
        self.code_section.push(1); // count 1

        let tree = self.tree.take().unwrap();
        let mut buf = vec![1]; // 1 = number of different local types
                               // number of f64 locals
        buf.append(&mut as_uleb128(
            tree.iter()
                .filter(|expr| matches!(expr, Expression::Let(_)))
                .count() as u64,
        ));
        buf.push(types::F64); // f64 locals type
        self.program(&mut buf, tree);
        buf.push(instructions::END); // end of function body
        self.code_section.append(&mut as_uleb128(buf.len() as u64)); // len of function body
        self.code_section.append(&mut buf); // body content

        [
            b"\0asm",             // magic
            &1_i32.to_le_bytes(), // version (1)
            // Type section
            &[1][..],                                    // section ID
            &as_uleb128(self.type_section.len() as u64), // len
            &self.type_section,                          // contents
            // Function section
            &[3],                                            // section ID
            &as_uleb128(self.function_section.len() as u64), // len
            &self.function_section,                          // contents
            // Export section
            &[7],                                          // section ID
            &as_uleb128(self.export_section.len() as u64), // len
            &self.export_section,                          // contents
            // Code section
            &[10],                                       // section ID
            &as_uleb128(self.code_section.len() as u64), // len
            &self.code_section,                          // contents
        ]
        .concat()
    }

    fn program(&mut self, buf: &mut Vec<u8>, node: Program) {
        for expr in node {
            self.expression(buf, expr);
        }
    }

    fn expression(&mut self, buf: &mut Vec<u8>, node: Expression) {
        match node {
            Expression::Let(expr) => self.let_expr(buf, expr),
            Expression::Add(expr) => self.add_expr(buf, expr),
        }
    }

    fn let_expr(&mut self, buf: &mut Vec<u8>, node: LetExpr) {
        self.expression(buf, *node.expr);

        let local_idx = as_uleb128(self.symbols.len() as u64);
        buf.push(instructions::LOCAL_SET);
        buf.append(&mut local_idx.clone()); // index in locals
        self.symbols.insert(node.name, local_idx);
    }

    fn add_expr(&mut self, buf: &mut Vec<u8>, node: AddExpr) {
        self.term(buf, *node.term);
        for (op, term) in node.following {
            self.term(buf, term);
            let instruction = match op {
                TermOperator::Plus => instructions::F64_ADD,
                TermOperator::Minus => instructions::F64_SUB,
            };
            buf.push(instruction);
        }
    }

    fn term(&mut self, buf: &mut Vec<u8>, node: Term) {
        self.factor(buf, node.factor);
        for (op, factor) in node.following {
            self.factor(buf, factor);
            let instruction = match op {
                FactorOperator::Multiply => instructions::F64_MUL,
                FactorOperator::Divide => instructions::F64_DIV,
            };
            buf.push(instruction);
        }
    }

    fn factor(&mut self, buf: &mut Vec<u8>, node: Factor) {
        self.atom(buf, node.atom);

        for op in node.ops {
            match op {
                TermOperator::Plus => {}
                TermOperator::Minus => buf.push(instructions::F64_NEG),
            }
        }
    }

    fn atom(&mut self, buf: &mut Vec<u8>, node: Atom) {
        match node {
            Atom::Number(num) => {
                buf.push(instructions::F64_CONST);
                buf.extend_from_slice(&num.to_le_bytes());
            }
            Atom::Ident(name) => {
                let mut local_idx = self
                    .symbols
                    .get(&name)
                    .unwrap_or_else(|| panic!("unresolved variable `{name}`"))
                    .clone();
                buf.push(instructions::LOCAL_GET);
                buf.append(&mut local_idx); // index in locals
            }
            Atom::Expression(node) => self.add_expr(buf, node),
        }
    }
}
