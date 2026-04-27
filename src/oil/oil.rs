use std::{fs, io};

use anyhow::Result;

use crate::oil::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

pub struct Oil {
    scanner: Scanner,
    parser: Parser,
    interpreter: Interpreter,
}

impl Oil {
    pub fn new() -> Oil {
        Oil {
            scanner: Scanner::new(),
            parser: Parser::new(),
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, path: &str) {
        println!("Attempting to run file at path: {}", path);
        let contents = fs::read_to_string(path).expect("Unable to read file");

        self.run(&contents);
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        let mut input = String::new();

        loop {
            input.clear();

            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("Exiting...");
                    break;
                }
                Ok(_) => {
                    self.run(&input);
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    break;
                }
            }
        }

        Ok(())
    }

    fn run(&mut self, source: &str) {
        let token_result = self.scanner.scan_tokens(source);

        if let Err(error) = token_result {
            eprintln!("Scanning error: {error}");
            return;
        }

        let tokens = token_result.unwrap();
        let statements = self.parser.parse(tokens);

        if let Err(error) = statements {
            eprintln!("Parsing error: {error}");
            return;
        }

        let statements = statements.unwrap();
        self.interpreter.interpret(&statements);
    }
}
