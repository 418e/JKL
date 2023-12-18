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

fn pathf(param: bool) -> &'static str {
    let settings = Config::builder()
        .add_source(config::File::with_name("tron"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    let envm = &settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()["env"];
    if envm == "dev" {
        if param {
            return "./tron.toml";
        } else {
            return "test/";
        }
    } else if param {
        return "./tron.toml";
    } else {
        return "src/";
    }
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
    if settings("experimental") == "true" && settings("warnings") != "false" {
        println!(
            "\n {} \n",
            "\n ⚠️ Warning! \n ⚠️ Warning! \n ⚠️ Warning! \n Experimental Features are enabled \n \n"
        )
    }
    if settings("credits") == "true" && settings("warnings") != "false" {
        println!(
            "\n APP: {} \n Version: {}\n Author: {}\n License: {}",
            settings("name"),
            settings("version"),
            settings("authors"),
            settings("license")
        )
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
    let path = std::env::current_dir().unwrap();
    if args.len() == 2 {
        let command = &args[1];
        if command == "config" {
            let config = r#"
                name = "TronProject"
                entry = "main"
                version = "0.0.1"
                authors = "YOU"
                license = "MIT"
                decor = "default"
                pointer = "default"
                env = "prod"
                experimental = "false"
                credits = "false"
                warnings = "true"
            "#;
            fs::write(path.join("tron.toml"), config).expect("Unable to write file");
        } else {
            let filename = command;
            let path_buf = path.join(filename);
            let input = path_buf.to_str();
            match input {
                Some(input) => {
                    println!("{:?}", input);
                    match run_file(input) {
                        Ok(_) => exit(0),
                        Err(msg) => {
                            println!("Error 108:\n{}", msg);
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
        println!("Usage: tron <filename> or tron config");
        exit(64);
    }
}
