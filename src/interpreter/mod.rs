use crate::environment::*;
use crate::expressions::*;
use crate::library::standard_library;
use crate::parser::*;
use crate::resolver::*;
use crate::scanner::*;
use crate::utils::TronError;
use std::collections::HashMap;
pub mod expressions;

#[derive(Debug)]
pub struct Interpreter {
    pub specials: HashMap<String, TronType>,
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            specials: HashMap::new(),
            environment: Environment::new(HashMap::new()),
        };
        standard_library(&mut interpreter.environment);

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
    pub fn interpret(&mut self, stmts: Vec<&Statement>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Statement::ExpressionStatement {
                    expression,
                    line: _,
                } => {
                    expression.evaluate(self.environment.clone())?;
                }
                Statement::UseStatement { expression, line } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    let path = std::env::current_dir().unwrap();
                    let path_buf =
                        path.join(value.to_string().trim_matches('"').trim_start_matches('/'));
                    match value.to_string().as_str() {
                        // "\"#math\"" => include_math_natives(&mut self.environment),
                        _ => {
                            if std::path::Path::new(&path_buf).exists() {
                                let lib_contents = std::fs::read_to_string(&path_buf)
                                    .map_err(|e| e.to_string())?;
                                self.execute_lib(&lib_contents)?;
                            } else {
                                TronError::throw("E4005", *line, vec![value.to_string()]);
                            }
                        }
                    }
                }
                Statement::VariableStatement {
                    name,
                    value_type: _,
                    value,
                    line: _,
                } => {
                    let value = value.evaluate(self.environment.clone())?;
                    let value_clone = value.clone();
                    self.environment.define(name.lexeme.clone(), value_clone);
                }
                Statement::BlockStatement {
                    statements,
                    line: _,
                } => {
                    let new_environment = self.environment.enclose();
                    let old_environment = self.environment.clone();
                    self.environment = new_environment;
                    let block_result =
                        self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
                    self.environment = old_environment;
                    block_result?;
                }
                Statement::IfStatement {
                    conditions,
                    then_branch,
                    elif_branches,
                    else_branch,
                    line: _,
                } => {
                    let mut all_true = true;
                    for condition in conditions {
                        let truth_value = condition.evaluate(self.environment.clone())?;
                        if truth_value.is_truthy() != TronType::True {
                            all_true = false;
                            break;
                        }
                    }
                    if all_true {
                        self.interpret(vec![then_branch.as_ref()])?;
                    } else {
                        let mut executed_branch = false;
                        for (elif_predicates, elif_stmt) in elif_branches {
                            let mut all_true = true;
                            for elif_predicate in elif_predicates {
                                let elif_truth_value =
                                    elif_predicate.evaluate(self.environment.clone())?;
                                if elif_truth_value.is_truthy() != TronType::True {
                                    all_true = false;
                                    break;
                                }
                            }
                            if all_true {
                                self.interpret(vec![elif_stmt.as_ref()])?;
                                executed_branch = true;
                                break;
                            }
                        }
                        if !executed_branch {
                            if let Some(els_stmt) = else_branch {
                                self.interpret(vec![els_stmt.as_ref()])?;
                            }
                        }
                    }
                }
                Statement::WhileStatement {
                    conditions,
                    body,
                    line: _,
                } => {
                    let mut all_true = true;
                    for condition in conditions {
                        let truth_value = condition.evaluate(self.environment.clone())?;
                        if truth_value.is_truthy() != TronType::True {
                            all_true = false;
                            break;
                        }
                    }
                    while all_true {
                        self.interpret(vec![body.as_ref()])?;
                        all_true = true;
                        for condition in conditions {
                            let truth_value = condition.evaluate(self.environment.clone())?;
                            if truth_value.is_truthy() != TronType::True {
                                all_true = false;
                                break;
                            }
                        }
                    }
                }
                Statement::FunctionStatement {
                    name,
                    params: _,
                    body: _,
                    output_type: _,
                    line: _,
                } => {
                    let callable = self.make_function(stmt);
                    let fun = TronType::Callable(CallableImpl::Function(callable));
                    self.environment.define(name.lexeme.clone(), fun);
                }
                Statement::ReturnStatement {
                    keyword: _,
                    value,
                    line: _,
                } => {
                    let eval_val;
                    if let Some(value) = value {
                        eval_val = value.evaluate(self.environment.clone())?;
                    } else {
                        eval_val = TronType::Null;
                    }
                    self.specials.insert("return".to_string(), eval_val);
                }
                Statement::BreakStatement { .. } => {
                    return Err("break".to_string());
                }
                Statement::SwitchStatement {
                    condition,
                    case_branches,
                    default_branch,
                    line: _,
                } => {
                    let condition_value = condition.evaluate(self.environment.clone())?;
                    let mut executed = false;
                    for (case_value, case_body) in case_branches {
                        let case_value = case_value.evaluate(self.environment.clone())?;
                        if condition_value == case_value {
                            self.interpret(case_body.iter().collect())?;
                            executed = true;
                            break;
                        }
                    }
                    if !executed && default_branch.is_some() {
                        self.interpret(default_branch.as_ref().unwrap().iter().collect())?;
                    }
                }
            };
        }
        Ok(())
    }
    fn make_function(&self, fn_stmt: &Statement) -> FunctionImpl {
        if let Statement::FunctionStatement {
            name,
            params,
            body,
            output_type,
            line: _,
        } = fn_stmt
        {
            let arity = params.len();
            let params: Vec<(Token, Token)> = params
                .iter()
                .map(|(name, type_token)| (name.clone(), type_token.clone()))
                .collect();
            let body: Vec<Box<Statement>> = body.iter().map(|b| (*b).clone()).collect();
            let name_clone = name.lexeme.clone();
            let output_type_clone = output_type.clone();
            let parent_env = self.environment.clone();
            let callable_impl = FunctionImpl {
                name: name_clone,
                arity,
                parent_env,
                params,
                body,
                output_type: output_type_clone,
            };
            callable_impl
        } else {
            TronError::throw("E4006", 0, vec![]);
            FunctionImpl {
                name: "err".to_string(),
                arity: 0,
                parent_env: self.environment.clone(),
                params: vec![],
                body: vec![],
                output_type: Token {
                    token_type: TokenType::Null,
                    lexeme: "".to_string(),
                    literal: None,
                    line_number: 0,
                },
            }
        }
    }
    fn execute_lib(&mut self, lib_contents: &str) -> Result<(), String> {
        let scanner = Scanner::new(lib_contents);
        let tokens = scanner.scan_tokens().map_err(|e| e.to_string())?;
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse().map_err(|e| e.to_string())?;
        let stmts_refs: Vec<&Statement> = stmts.iter().collect();
        let resolver = Resolver::new();
        let locals = resolver.resolve(&stmts.iter().collect(), &mut self.environment)?;
        self.resolve(locals);
        self.interpret(stmts_refs)
    }
}
