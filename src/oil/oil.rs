use std::{fs, io};

use anyhow::Result;

use crate::oil::{interpreter::{self, Interpreter}, parser::Parser, scanner::Scanner};

pub struct Oil {}

impl Oil {
    pub fn run_file(path: &str) {
        println!("Attempting to run file at path: {}", path);
        let contents = fs::read_to_string(path).expect("Unable to read file");

        Oil::run(&contents);
    }

    pub fn run_prompt() -> Result<()> {
        let mut input = String::new();

        loop {
            input.clear();

            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("Exiting...");
                    break;
                }
                Ok(_) => {
                    Oil::run(&input);
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    break;
                }
            }
        }

        Ok(())
    }

    fn run(source: &str) {
        let mut scanner = Scanner::new(source);
        let token_result = scanner.scan_tokens();

        if let Err(error) = token_result {
            eprintln!("Scanning error: {error}");
            return;
        }

        let tokens = token_result.unwrap();
        println!("TOKENS: {:?}", tokens);
        
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        if let None = expression {
            eprintln!("Failed to resolve expression.");
            return;
        }

        let expression = expression.unwrap();
        Interpreter::interpret(&expression);
    }
}
