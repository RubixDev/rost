mod lexer;
mod tokens;
mod parser;
mod nodes;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use rustyline::{Editor, error::ReadlineError};

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.chars().all(|char| [' ', '\t'].contains(&char)) { continue; }

                let start = std::time::Instant::now();

                let mut lexer = Lexer::new(&line);
                let tokens = lexer.scan();

                let mut parser = Parser::new(tokens);
                let nodes = parser.parse();

                let interpreter = Interpreter::new(nodes);
                interpreter.run();
                println!("\x1b[90m[{:?}]\x1b[0m", start.elapsed());
            },
            Err(ReadlineError::Eof) => break,
            Err(_) => std::process::exit(1),
        }
    }
}
