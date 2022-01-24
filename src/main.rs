mod lexer;
mod tokens;
mod parser;
mod nodes;
mod interpreter;

use std::io::{BufRead, Write};
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let line = std::io::stdin().lock().lines().next().unwrap().unwrap();

        let start = std::time::Instant::now();

        let mut lexer = Lexer::new(&line);
        let tokens = lexer.scan();
        // println!("{:?}", tokens);

        let mut parser = Parser::new(tokens);
        let nodes = parser.parse();

        let interpreter = Interpreter::new(nodes);
        interpreter.run();
        println!("\x1b[90m[{:?}]\x1b[0m", start.elapsed());
    }
}
