mod environment;
mod expr;
mod interpreter;
mod libs;
mod natives;
mod parser;
mod resolver;
mod scanner;
mod stmt;
use crate::interpreter::*;
use crate::parser::*;
use crate::resolver::*;
use crate::scanner::*;
use colored::Colorize;
use std::env;
use std::fs;
use std::process::exit;

pub fn panic(message: &str) {
    eprintln!("{} \n", message.red());
    exit(1);
}

pub fn run_file(path: &str) -> Result<(), String> {
    let absolute_path = if path.starts_with("/") {
        path.to_string()
    } else {
        let current_dir = std::env::current_dir().unwrap();
        current_dir.join(path).to_str().unwrap().to_string()
    };
    match fs::read_to_string(&absolute_path) {
        Err(msg) => Err(msg.to_string()),
        Ok(contents) => run_string(&contents),
    }
}
pub fn run_string(contents: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    run(&mut interpreter, contents)
}
fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    let resolver = Resolver::new();
    let locals = resolver.resolve(&stmts.iter().collect())?;
    println!("\n");
    interpreter.resolve(locals);
    interpreter.interpret(stmts.iter().collect())?;
    println!("\n");
    return Ok(());
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = std::env::current_dir().unwrap();
    if args.len() == 2 {
        let command = &args[1];
        if command == "version" {
            println!("v2.0.0")
        } else {
            let filename = command;
            let path_buf = path.join(filename);
            let input = path_buf.to_str();
            match input {
                Some(input) => {
                    match run_file(input) {
                        Ok(_) => exit(0),
                        Err(msg) => {
                            println!("\nError:\n{}", msg);
                            exit(1);
                        }
                    }
                }
                None => {
                    println!("Error: Non-Unicode file path");
                    exit(1);
                }
            }
        }
    } else {
        println!("Usage: tron <filename> or tron version");
        exit(64);
    }
}
