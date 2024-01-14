/*

    Tron Programming Language

    - Welcome to Tron's source code


    latest version: 2.4.1
    latest release: Jan 14

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
use std::env;
use std::fs;
use std::process::exit;
use std::process::Command;

pub fn panic(message: &str) {
    eprintln!("\x1B[31m{}\x1B[0m \n", message);
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
    interpreter.resolve(locals);
    interpreter.interpret(stmts.iter().collect())?;
    return Ok(());
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = std::env::current_dir().unwrap();
    if args.len() == 2 {
        let command = &args[1];
        if command == "version" {
            println!("v2.4.1")
        } else if command == "update" {
            println!("Updating Tron....");
            let _output = Command::new("bash")
                .arg("-c")
                .arg("curl -sSL https://tronlang.org/install.sh | bash")
                .output()
                .expect("Failed to execute command");
            println!("Update completed");
        } else {
            let filename = command;
            let path_buf = path.join(filename);
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
    } else {
        println!(
            "
\x1B[38;5;208m
            ████████╗██████╗  ██████╗ ███╗   ██╗
            ╚══██╔══╝██╔══██╗██╔═══██╗████╗  ██║
               ██║   ██████╔╝██║   ██║██╔██╗ ██║
               ██║   ██╔══██╗██║   ██║██║╚██╗██║
               ██║   ██║  ██║╚██████╔╝██║ ╚████║
               ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝\x1B[0m
                                    
        Usage: 

        \x1B[32mtron\x1B[0m \x1B[33m<filename>\x1B[0m - interpret tron files
        \x1B[32mtron\x1B[0m \x1B[34mversion\x1B[0m - current version of Tron
        \x1B[32mtron\x1B[0m \x1B[34mupdate\x1B[0m - install latest version of Tron

             \x1B[38;5;208mTron Programming Language (2.4.1)\x1B[0m
        "
        );
        exit(64);
    }
}