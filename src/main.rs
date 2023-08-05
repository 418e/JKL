mod environment;
mod expr;
mod interpreter;
mod parser;
mod resolver;
mod scanner;
mod stmt;
use crate::interpreter::*;
use crate::parser::*;
use crate::resolver::*;
use crate::scanner::*;
use std::env;
use std::fs;
use std::process::exit;
use colored::Colorize;

pub fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string("tron/".to_owned() + path + ".tron") {
        Err(msg) => return Err(msg.to_string().yellow().to_string()),
        Ok(contents) => return run_string(&contents),
    }
}
pub fn run_string(contents: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    run(&mut interpreter, contents)
}
fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    let resolver = Resolver::new();
    let locals = resolver.resolve(&stmts.iter().collect())?;
    interpreter.resolve(locals);
    interpreter.interpret(stmts.iter().collect())?;
    return Ok(());
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        println!("Argument isn't specified.");
        exit(64);
    }
}