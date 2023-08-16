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
use colored::Colorize;
use config::Config;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
pub fn run_file(path: &str) -> Result<(), String> {
    /*development */
    match fs::read_to_string("test/".to_owned() + path + ".tron")
    /*production */
    // match fs::read_to_string("../../".to_owned() + path + ".tron") 
    {
        Err(msg) => return Err(msg.to_string().yellow().to_string()),
        Ok(contents) => return run_string(&contents),
    }
}
pub fn run_string(contents: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    run(&mut interpreter, contents)
}
fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let settings = Config::builder()
        /*production */
        // .add_source(config::File::with_name("../../tron"))
        /*development */
        .add_source(config::File::with_name("test/tron"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    let decor = &settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()["decor"];

    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    let resolver = Resolver::new();
    let locals = resolver.resolve(&stmts.iter().collect())?;
    if decor == "false" {
        println!("\n");
    } else if decor == "default" {
        println!("\n ╔════════════《 📄 》════════════╗ \n");
    }else {
        println!("\n ╚════════════《 {} 》════════════╝", decor);
    }

    interpreter.resolve(locals);
    interpreter.interpret(stmts.iter().collect())?;
    if decor == "false" {
        println!("\n");
    }  else if decor == "default" {
        println!("\n ╚════════════《 📄 》════════════╝ \n");
    }else {
        println!("\n ╚════════════《 {} 》════════════╝", decor);
    }
    return Ok(());
}
fn main() {
    let settings = Config::builder()
        /*production */
        // .add_source(config::File::with_name("../../tron"))
        /*development */
        .add_source(config::File::with_name("test/tron"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    let entry = &settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()["entry"];
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error 108:\n{}", msg);
                exit(1);
            }
        }
    } else if args.len() == 1 {
        match run_file(&entry) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error 108:\n{}", msg);
                exit(1);
            }
        }
    } else {
        println!("Error 108: Argument isn't specified.");
        exit(64);
    }
}
