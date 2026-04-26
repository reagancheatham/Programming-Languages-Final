use std::{fs, io};

use anyhow::Result;

use crate::oil::{parser::Parser, scanner::Scanner};

pub struct Oil {}

impl Oil {
    pub fn run_file(path: &str) -> Result<()> {
        println!("Attempting to run file at path: {}", path);
        let contents = fs::read_to_string(path).expect("Unable to read file");

        Oil::run(&contents)
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
                    Oil::run(&input)?;
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    break;
                }
            }
        }

        Ok(())
    }

    fn run(source: &str) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        println!("Tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        match expression {
            Some(expr) => println!("{}", expr.to_tree()),
            None => {}
        }

        Ok(())
    }
}
