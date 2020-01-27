mod backend;
mod parser;
mod tokenizer;

use parser::ParseError;
use std::io;
use std::io::{BufRead, Write};

fn main() {
    loop {
        print!("pebble> ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().lock().read_line(&mut s).unwrap();
        let mut lexer = tokenizer::Tokenizer {
            input: s.chars().peekable(),
        }
        .peekable();
        let tree = parser::parse_expr(&mut lexer);
        match tree {
            Ok(x) => match backend::evaluate(&x) {
                Ok(y) => println!("= {}", y),
                Err(e) => println!("Interpreter error: {}", e),
            },
            Err(e) => println!(
                "Parser error: {}",
                match e {
                    ParseError::UnexpectedEOF => "unexpected EOF".to_string(),
                    ParseError::UnexpectedToken(t) => format!("unexpected token `{:?}`", t),
                }
            ),
        }
    }
}
