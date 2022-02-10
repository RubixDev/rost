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
        // Eine Zeile als Eingabe lesen
        let line = std::io::stdin().lock().lines().next().unwrap().unwrap();
        // Ignorieren, wenn nur Leerzeichen
        if line.chars().all(|char| [' ', '\t', '\r'].contains(&char)) { continue; }

        let mut lexer = Lexer::new(line);
        let tokens = lexer.scan();

        let mut parser = Parser::new(tokens);
        let nodes = parser.parse();

        Interpreter::run(nodes);
    }
}
