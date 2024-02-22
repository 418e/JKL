/*

    Tron Programming Language

    - Welcome to Tron's source code


    latest version: 2.8.0 (Feb 22, 2024)

*/
mod environment;
mod expr;
mod interpreter;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod tstd;
use crate::interpreter::*;
use crate::parser::*;
use crate::resolver::*;
use crate::scanner::*;
use std::{env, fs, process::{exit, Command}};

pub fn panic(message: &str) {
    eprintln!("\x1B[31m{}\x1B[0m \n", message);
    exit(1);
}

pub fn run_file(path: &str) -> Result<(), String> {
    let current_dir = std::env::current_dir().unwrap();
    match fs::read_to_string(&current_dir.join(path).to_str().unwrap().to_string()) {
        Err(msg) => Err(msg.to_string()),
        Ok(contents) => {
            let mut interpreter = Interpreter::new();
            run(&mut interpreter, &contents)
        }
    }
}

fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    let resolver = Resolver::new();
    let locals = resolver.resolve(&stmts.iter().collect())?;
    interpreter.resolve(locals);
    interpreter.interpret(stmts.iter().collect())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = std::env::current_dir().unwrap();
    if args.len() == 1 {
        exit(64);
    }
    let command = args[1].as_str();
    match command {
        "version" => {
            println!("v2.8.0");
        }
        "update" => {
            println!("Updating....");
            let _output = Command::new("bash")
                .arg("-c")
                .arg("curl -sSL https://tronlang.org/install.sh | bash")
                .output()
                .expect("Failed to execute command");
            println!("Update completed");
        }
        "help" => {
            println!(
                "
        \x1B[32mtron\x1B[0m \x1B[33m<filename>\x1B[0m - interpret tron files
        \x1B[32mtron\x1B[0m \x1B[34mversion\x1B[0m - current version of Tron
        \x1B[32mtron\x1B[0m \x1B[34mupdate\x1B[0m - install latest version of Tron

             \x1B[38;5;208mTron Programming Language (2.8.0)\x1B[0m
        "
            );
        }
        _ => {
            let path_buf = path.join(command);
            let input = path_buf.to_str();
            match input {
                Some(input) => match run_file(input) {
                    Ok(_) => exit(0),
                    Err(_msg) => {
                        exit(1);
                    }
                },
                None => {
                    println!("Error: Non-Unicode file path");
                    exit(1);
                }
            }
        }
    }
}
