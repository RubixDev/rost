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
        print!("> ");
        std::io::stdout().flush().unwrap();
        let line = std::io::stdin().lock().lines().next().unwrap().unwrap();
        if line.chars().all(|char| [' ', '\t'].contains(&char)) { continue; }

        let start = std::time::Instant::now();

        let mut lexer = Lexer::new(line);
        let tokens = lexer.scan();

        let mut parser = Parser::new(tokens);
        let nodes = parser.parse();

        let interpreter = Interpreter::new(nodes);
        interpreter.run();
        println!("[{:?}]", start.elapsed());
    }
}
