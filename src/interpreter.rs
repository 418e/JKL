use crate::environment::Environment;
use crate::expr::{CallableImpl, LiteralValue, NativeFunctionImpl, TronFunctionImpl};
use crate::parser::*;
use crate::pathf;
use crate::resolver::*;
use crate::scanner::Token;
use crate::scanner::*;
use crate::settings;
use crate::stmt::Stmt;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::process::exit;
use std::process::Command;
use std::rc::Rc;
pub struct Interpreter {
    pub specials: HashMap<String, LiteralValue>,
    pub environment: Environment,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            specials: HashMap::new(),
            environment: Environment::new(HashMap::new()),
        }
    }
    pub fn resolve(&mut self, locals: HashMap<usize, usize>) {
        self.environment.resolve(locals);
    }
    pub fn with_env(env: Environment) -> Self {
        Self {
            specials: HashMap::new(),
            environment: env,
        }
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    if settings("pointer") == "default" {
                        println!(" ➤ {}", value.to_string().green().to_string());
                    } else {
                        println!(
                            " {} {}",
                            settings("pointer"),
                            value.to_string().green().to_string()
                        );
                    }
                }
                Stmt::Input { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    if settings("pointer") == "default" {
                        println!(" ➤ {}", value.to_string());
                    } else {
                        println!(" {} {}", settings("pointer"), value.to_string());
                    }
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                }
                Stmt::Errors { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    if settings("pointer") == "default" {
                        println!(" ➤ {}", value.to_string().red().to_string());
                    } else {
                        println!(
                            " {} {}",
                            settings("pointer"),
                            value.to_string().red().to_string()
                        );
                    }
                    exit(1)
                }
                Stmt::Exits {} => exit(1),
                Stmt::Import { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    fn run_file(path: &str) -> Result<(), String> {
                        match fs::read_to_string(pathf(false).to_owned() + path + ".tron") {
                            Err(msg) => Err(msg.to_string()),
                            Ok(contents) => run_string(&contents),
                        }
                    }
                    fn run_string(contents: &str) -> Result<(), String> {
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
                    let val: String = value.to_string();
                    fn rem_first_and_last(value: &str) -> &str {
                        let mut chars = value.chars();
                        chars.next();
                        chars.next_back();
                        chars.as_str()
                    }
                    match run_file(rem_first_and_last(&val)) {
                        Ok(_) => {}
                        Err(msg) => {
                            println!("Error 108:\n{}", msg);
                            exit(1);
                        }
                    }
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(self.environment.clone())?;
                    self.environment.define(name.lexeme.clone(), value);
                }
                Stmt::Block { statements } => {
                    let new_environment = self.environment.enclose();
                    let old_environment = self.environment.clone();
                    self.environment = new_environment;
                    let block_result =
                        self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
                    self.environment = old_environment;
                    block_result?;
                }
                Stmt::IfStmt {
                    predicate,
                    then,
                    els,
                } => {
                    let truth_value = predicate.evaluate(self.environment.clone())?;
                    if truth_value.is_truthy() == LiteralValue::True {
                        let statements = vec![then.as_ref()];
                        self.interpret(statements)?;
                    } else if let Some(els_stmt) = els {
                        let statements = vec![els_stmt.as_ref()];
                        self.interpret(statements)?;
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    let mut flag = condition.evaluate(self.environment.clone())?;
                    while flag.is_truthy() == LiteralValue::True {
                        let statements = vec![body.as_ref()];
                        self.interpret(statements)?;
                        flag = condition.evaluate(self.environment.clone())?;
                    }
                }
                Stmt::BenchStmt { body } => {
                    let start_time = std::time::SystemTime::now();
                    let statements = vec![body.as_ref()];
                    self.interpret(statements)?;
                    let end_time = std::time::SystemTime::now().duration_since(start_time);
                    if end_time.clone().unwrap().as_micros() < 10000 {
                        println!("{:?}µs", end_time.unwrap().as_micros());
                    } else if end_time.clone().unwrap().as_micros() > 10000 {
                        println!("{:?}ms", end_time.unwrap().as_millis());
                    } else if end_time.clone().unwrap().as_millis() > 10000 {
                        println!("{:?}s", end_time.unwrap().as_secs_f32());
                    } else {
                        println!("{:?}µs", end_time.unwrap().as_micros());
                    }
                }
                Stmt::Function {
                    name,
                    params: _,
                    body: _,
                } => {
                    let callable = self.make_function(stmt);
                    let fun = LiteralValue::Callable(CallableImpl::TronFunction(callable));
                    self.environment.define(name.lexeme.clone(), fun);
                }
                Stmt::CmdFunction { name, cmd } => {
                    let cmd = cmd.clone();
                    let local_fn = move |_args: &Vec<LiteralValue>| {
                        let cmd = cmd.clone();
                        let parts = cmd.split(" ").collect::<Vec<&str>>();
                        let mut command = Command::new(parts[0].replace("\"", ""));
                        for part in parts[1..].iter() {
                            command.arg(part.replace("\"", ""));
                        }
                        let output = command.output().expect("Failed to run command");
                        return LiteralValue::StringValue(
                            std::str::from_utf8(output.stdout.as_slice())
                                .unwrap()
                                .to_string(),
                        );
                    };
                    let fun_val =
                        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                            name: name.lexeme.clone(),
                            arity: 0,
                            fun: Rc::new(local_fn),
                        }));
                    self.environment.define(name.lexeme.clone(), fun_val);
                }
                Stmt::ReturnStmt { keyword: _, value } => {
                    let eval_val;
                    if let Some(value) = value {
                        eval_val = value.evaluate(self.environment.clone())?;
                    } else {
                        eval_val = LiteralValue::Nil;
                    }
                    self.specials.insert("return".to_string(), eval_val);
                }
            };
        }
        Ok(())
    }
    fn make_function(&self, fn_stmt: &Stmt) -> TronFunctionImpl {
        if let Stmt::Function { name, params, body } = fn_stmt {
            let arity = params.len();
            let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
            let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
            let name_clone = name.lexeme.clone();
            let parent_env = self.environment.clone();
            let callable_impl = TronFunctionImpl {
                name: name_clone,
                arity,
                parent_env,
                params,
                body,
            };
            callable_impl
        } else {
            panic!("Tried to make a function from a non-function statement");
        }
    }
}
