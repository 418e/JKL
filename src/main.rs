mod commands;
mod environment;
mod interpreter;
mod library;
mod parser;
mod resolver;
mod scanner;
mod utils;
use crate::commands::help::cli_help;
use crate::commands::run::cli_run;
use crate::commands::update::cli_update;
use crate::commands::version::cli_version;
use crate::{interpreter::*, parser::*, resolver::*, scanner::*};
use std::{env, fs, process::exit};
use utils::TronError;

pub fn panic(message: &str) {
    eprintln!("\x1B[31m{}\x1B[0m \n", message);
    exit(1);
}

pub fn run_file(path: &str) -> Result<(), String> {
    let current_dir = std::env::current_dir().unwrap();
    match fs::read_to_string(&current_dir.join(path).to_str().unwrap().to_string()) {
        Err(_msg) => {
            TronError::throw("E0001", 0, vec![]);
            Ok(())
        }
        Ok(contents) => run(&contents),
    }
}

fn run(contents: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    let scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    let resolver = Resolver::new();
    let locals = resolver.resolve(&stmts.iter().collect(), &mut interpreter.environment)?;
    interpreter.resolve(locals);
    interpreter.interpret(stmts.iter().collect())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = std::env::current_dir().unwrap();
    if args.len() == 1 {
        TronError::throw("E0002", 0, vec![]);
        exit(64);
    }
    let command = args[1].as_str();
    match command {
        "version" => cli_version(),
        "update" => cli_update(),
        "help" => cli_help(),
        "run" => cli_run(args[2].as_str(), path),
        _ => TronError::throw("E0002", 0, vec![]),
    }
}
