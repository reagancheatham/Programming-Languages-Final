use anyhow::Result;
use std::{env, process::exit};

use crate::oil::{oil::Oil};

mod oil;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: oil [script]");
        exit(64);
    } else if args.len() == 2 {
        let path = env::current_dir()?.to_string_lossy().into_owned();

        Oil::run_file(&format!("{path}\\{}", args[1]))?;
    } else {
        Oil::run_prompt()?;
    }

    Ok(())
}
