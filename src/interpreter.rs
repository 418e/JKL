/*

    Tron Interpreter

    - Output of every statement

*/
use crate::environment::Environment;
use crate::expr::{CallableImpl, LiteralValue, NativeFunctionImpl, TronFunctionImpl};
use crate::tstd::*;
use crate::panic;
use crate::parser::*;
use crate::resolver::*;
use crate::scanner::Token;
use crate::scanner::*;
use crate::stmt::Stmt;
use std::collections::HashMap;
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
        let interpreter = Self {
            specials: HashMap::new(),
            environment: Environment::new(HashMap::new()),
        };

        interpreter.environment.define(
            "typeof".to_string(),
            LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                name: "typeof".to_string(),
                arity: 1,
                fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
                    if args.len() != 1 {
                        panic("\n typeof() expects one argument");
                    }
                    match &args[0] {
                        LiteralValue::Number(_n) => LiteralValue::StringValue("number".to_string()),
                        LiteralValue::StringValue(_n) => {
                            LiteralValue::StringValue("string".to_string())
                        }
                        LiteralValue::Nil => LiteralValue::StringValue("null".to_string()),
                        LiteralValue::False => LiteralValue::StringValue("boolean".to_string()),
                        LiteralValue::True => LiteralValue::StringValue("boolean".to_string()),
                        LiteralValue::ArrayValue(_n) => {
                            LiteralValue::StringValue("array".to_string())
                        }
                        _ => {
                            panic("\n uknown type");
                            exit(1)
                        }
                    }
                }),
            })),
        );
        interpreter.environment.define(
            "input".to_string(),
            LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                name: "input".to_string(),
                arity: 1,
                fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
                    if args.len() != 1 {
                        panic("\n input() expects one argument");
                    }
                    match &args[0] {
                        LiteralValue::StringValue(n) => {
                            println!("{}", n.to_string());
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).unwrap();
                            LiteralValue::StringValue(format!("{}", input))
                        }
                        _ => {
                            panic("\n input() requires a numeric argument");
                            exit(1)
                        }
                    }
                }),
            })),
        );
        interpreter.environment.define(
            "len".to_string(),
            LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                name: "len".to_string(),
                arity: 1,
                fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
                    if args.len() != 1 {
                        panic("\n len() expects one argument");
                    }
                    match &args[0] {
                        LiteralValue::StringValue(n) => LiteralValue::Number(n.len() as f32),
                        LiteralValue::ArrayValue(n) => LiteralValue::Number(n.len() as f32),
                        _ => {
                            panic("\n len() requires a numeric argument");
                            exit(1)
                        }
                    }
                }),
            })),
        );
        interpreter.environment.define(
            "push".to_string(),
            LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                name: "push".to_string(),
                arity: 1,
                fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
                    if args.len() != 2 {
                        panic("\n push() expects two argumentss");
                    }
                    match &args[0] {
                        LiteralValue::ArrayValue(arr) => {
                            let mut arr = arr.clone();
                            arr.push(args[1].clone());
                            LiteralValue::ArrayValue(arr)
                        }
                        _ => {
                            panic("\n push() requires a numeric argument");
                            exit(1)
                        }
                    }
                }),
            })),
        );
        interpreter.environment.define(
            "join".to_string(),
            LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                name: "join".to_string(),
                arity: 1,
                fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
                    if args.len() != 2 {
                        panic("\n join() expects two argumentss");
                    }
                    match (&args[0], &args[1]) {
                        (LiteralValue::ArrayValue(arr), LiteralValue::StringValue(join_str)) => {
                            let mut strings = Vec::new();
                            for val in arr.iter() {
                                match val {
                                    LiteralValue::Number(num) => strings.push(num.to_string()),
                                    LiteralValue::StringValue(s) => strings.push(s.clone()),
                                    _ => panic("\n join() requires an array of strings or numbers"),
                                }
                            }
                            let joined = strings.join(join_str);
                            LiteralValue::StringValue(joined)
                        }
                        _ => {
                            panic("\n join() requires a numeric argument");
                            exit(1)
                        }
                    }
                }),
            })),
        );
        interpreter.environment.define(
            "pop".to_string(),
            LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                name: "pop".to_string(),
                arity: 1,
                fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
                    if args.len() != 1 {
                        panic("\n pop() expects one argument");
                    }
                    if let LiteralValue::ArrayValue(arr) = &args[0] {
                        let mut arr = arr.clone();
                        arr.pop();
                        LiteralValue::ArrayValue(arr)
                    } else {
                        panic("\n pop() requires a numeric argument");
                        exit(1)
                    }
                }),
            })),
        );
        interpreter.environment.define(
            "shift".to_string(),
            LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                name: "shift".to_string(),
                arity: 1,
                fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
                    if args.len() != 1 {
                        panic("\n shift() expects one argument");
                    }
                    if let LiteralValue::ArrayValue(arr) = &args[0] {
                        let mut arr = arr.clone();
                        if arr.is_empty() {
                            panic("\n shift() cannot remove from an empty array");
                            exit(1)
                        }
                        arr.remove(0);
                        LiteralValue::ArrayValue(arr)
                    } else {
                        panic("\n shift() requires a numeric argument");
                        exit(1)
                    }
                }),
            })),
        );

        interpreter
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
                    println!(" ➤ {}", value.to_string());
                }
                Stmt::Errors { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    panic!(" ➤ {}", value.to_string());
                }
                Stmt::Exits {} => exit(1),
                Stmt::Import { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    let val = value.to_string();
                    let path = std::env::current_dir().unwrap();
                    let path_buf = path.join(val.trim_matches('"').trim_start_matches('/'));

                    match val.as_str() {
                        "\"#math\"" => include_math_natives(&mut self.environment),
                        _ => {
                            if std::path::Path::new(&path_buf).exists() {
                                // Load and execute the library file
                                let lib_contents = std::fs::read_to_string(&path_buf)
                                    .map_err(|e| e.to_string())?;
                                self.execute_lib(&lib_contents)?;
                            } else {
                                panic(&format!(
                                    "Library not found: {:?}",
                                    std::path::Path::new(&path_buf)
                                ));
                                exit(1);
                            }
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
                    elif_branches,
                    els,
                } => {
                    let truth_value = predicate.evaluate(self.environment.clone())?;
                    if truth_value.is_truthy() == LiteralValue::True {
                        self.interpret(vec![then.as_ref()])?;
                    } else {
                        let mut executed_branch = false;
                        for (elif_predicate, elif_stmt) in elif_branches {
                            let elif_truth_value =
                                elif_predicate.evaluate(self.environment.clone())?;
                            if elif_truth_value.is_truthy() == LiteralValue::True {
                                self.interpret(vec![elif_stmt.as_ref()])?;
                                executed_branch = true;
                                break;
                            }
                        }
                        if !executed_branch {
                            if let Some(els_stmt) = els {
                                self.interpret(vec![els_stmt.as_ref()])?;
                            }
                        }
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    while condition.evaluate(self.environment.clone())?.is_truthy()
                        == LiteralValue::True
                    {
                        match self.interpret(vec![body.as_ref()]) {
                            Ok(_) => {}
                            Err(e) if e == "break" => break,
                            Err(e) => return Err(e),
                        }
                    }
                }
                Stmt::WaitStmt { time, body, before } => {
                    let time_in_ms = match time.evaluate(self.environment.clone())? {
                        LiteralValue::Number(i) => i,
                        _ => return Err("Expected a number for time".to_string()),
                    };

                    let mut before_time_in_ms = 0;

                    if let Some(before_block) = before {
                        before_time_in_ms =
                            match before_block.time.evaluate(self.environment.clone())? {
                                LiteralValue::Number(i) => i.round() as i32,
                                _ => return Err("Expected a number for before time".to_string()),
                            };

                        // Calculate the number of times to execute the before block
                        let num_executions =
                            (time_in_ms as f64 / before_time_in_ms as f64).ceil() as u64;
                        for _ in 0..num_executions {
                            self.interpret(vec![before_block.body.as_ref()])?;
                            std::thread::sleep(std::time::Duration::from_millis(
                                before_time_in_ms as u64,
                            ));
                        }
                    }

                    // Wait for the remaining time if necessary
                    let remaining_time = if before_time_in_ms > 0 {
                        time_in_ms as u64 % before_time_in_ms as u64
                    } else {
                        time_in_ms as u64
                    };
                    if remaining_time > 0 {
                        std::thread::sleep(std::time::Duration::from_millis(remaining_time));
                    }

                    self.interpret(vec![body.as_ref()])?;
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
                Stmt::BreakStmt { .. } => {
                    return Err("break".to_string());
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
            panic("\n Tried to make a function from a non-function statement");
            exit(1)
        }
    }
    fn execute_lib(&mut self, lib_contents: &str) -> Result<(), String> {
        let scanner = Scanner::new(lib_contents);
        let tokens = scanner.scan_tokens().map_err(|e| e.to_string())?;
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse().map_err(|e| e.to_string())?;
        let stmts_refs: Vec<&Stmt> = stmts.iter().collect();
        let resolver = Resolver::new();
        let locals = resolver.resolve(&stmts_refs)?;
        self.resolve(locals);
        self.interpret(stmts_refs)
    }
}
