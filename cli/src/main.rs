use std::{env, fs};

use rost::compiler::Compiler;
use rost::lexer::Lexer;
use rost::parser::Parser;
// use rustyline::{error::ReadlineError, Editor};

fn main() {
    let code = env::args().nth(1).expect("code");

    let mut lexer = Lexer::new(&code);
    let tokens = match lexer.scan() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("\x1b[31m{}\x1b[0m", e);
            return;
        }
    };

    let mut parser = Parser::new(&tokens);
    let nodes = match parser.parse() {
        Ok(nodes) => nodes,
        Err(e) => {
            eprintln!("\x1b[31m{}\x1b[0m", e);
            return;
        }
    };

    let compiler = Compiler::new(nodes);
    let bytes = compiler.compile();
    fs::write("output.wasm", &bytes).unwrap();

    // let mut rl = Editor::<()>::new();
    // loop {
    //     let line = rl.readline("> ");
    //     match line {
    //         Ok(line) => {
    //             rl.add_history_entry(line.as_str());
    //             if line.chars().all(|char| [' ', '\t'].contains(&char)) { continue; }
    //
    //             let start = std::time::Instant::now();
    //
    //             let mut lexer = Lexer::new(&line);
    //             let tokens = match lexer.scan() {
    //                 Ok(tokens) => tokens,
    //                 Err(e) => { eprintln!("\x1b[31m{}\x1b[0m", e); continue; },
    //             };
    //
    //             let mut parser = Parser::new(&tokens);
    //             let nodes = match parser.parse() {
    //                 Ok(nodes) => nodes,
    //                 Err(e) => { eprintln!("\x1b[31m{}\x1b[0m", e); continue; },
    //             };
    //
    //             let interpreter = Interpreter::new(nodes);
    //             let res = interpreter.run();
    //             println!("{}", res);
    //             println!("\x1b[90m[{:?}]\x1b[0m", start.elapsed());
    //         },
    //         Err(ReadlineError::Eof) => break,
    //         Err(_) => std::process::exit(1),
    //     }
    // }
}
