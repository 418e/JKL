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

pub fn settings(param: &str) -> String {
    let options = vec![
        "name",
        "entry",
        "version",
        "authors",
        "license",
        "decor",
        "pointer",
        "env",
        "experimental",
        "credits",
        "warnings",
    ];
    if options.iter().any(|&i| i == param) {
        let settings = Config::builder()
            .add_source(config::File::with_name(pathf(true)))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        let setting = &settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()[&param.to_string()];
        return setting.to_string();
    } else {
        return "NotFound".to_string();
    }
}

pub fn pathf(param: bool) -> &'static str {
    let settings = Config::builder()
        .add_source(config::File::with_name("test/tron"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    let envm = &settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()["env"];
    if envm == "dev" {
        if param {
            return "test/tron.toml";
        } else {
            return "test/";
        }
    } else {
        if param {
            return "../../tron.toml";
        } else {
            return "../../";
        }
    }
}
pub fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(pathf(false).to_owned() + path + ".tron") {
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
    if settings("experimental") == "true" {
        if settings("warnings") != "false" {
            println!(
                "\n {} \n",
                "⚠️ Warning! \n ⚠️ Warning! \n ⚠️ Warning! \n Experimental Features are enabled"
                    .yellow()
                    .to_string()
            )
        }
    }
    if settings("credits") == "true" {
        if settings("warnings") != "false" {
            println!(
                "\n APP: {} \n Version: {}\n Author: {}\n License: {}",
                settings("name").yellow().to_string(),
                settings("version").yellow().to_string(),
                settings("authors").yellow().to_string(),
                settings("license").yellow().to_string()
            )
        }
    }
    if settings("decor") == "false" {
        println!("\n");
    } else if settings("decor") == "default" {
        println!("\n ╔════════════《 📄 》════════════╗ \n");
    } else {
        println!("\n ╚════════════《 {} 》════════════╝", settings("decor"));
    }
    interpreter.resolve(locals);
    interpreter.interpret(stmts.iter().collect())?;
    if settings("decor") == "false" {
        println!("\n");
    } else if settings("decor") == "default" {
        println!("\n ╚════════════《 📄 》════════════╝ \n");
    } else {
        println!("\n ╚════════════《 {} 》════════════╝", settings("decor"));
    }
    return Ok(());
}
fn main() {
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
        match run_file(&settings("entry")) {
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
