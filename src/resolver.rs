use crate::environment::Environment;
use crate::expressions::Expression;
use crate::scanner::{Statement, Token};
use crate::utils::TronError;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug)]
enum FunctionType {
    None,
    Function,
}
#[derive(Copy, Clone, PartialEq, Debug)]
enum LoopType {
    None,
}
/// The `Resolver` struct in Rust is responsible for resolving symbols.
/// It maintains a stack of scopes, tracks the current function and loop context, and manages local variables.
///
/// # Fields
///
/// - `scopes`: A stack of scopes, where each scope is a `HashMap` mapping variable names to a boolean indicating if the variable is initialized.
/// - `current_function`: The type of the current function being resolved.
/// - `current_loop`: The type of the current loop being resolved.
/// - `locals`: A map of local variable IDs to their scope depth.
///
/// # Usage
///
/// The `Resolver` struct is used to resolve symbols in the Tron language. It is typically instantiated with the `new` method and then used to resolve statements and expressions within a given environment.
///
/// # Example
///
/// ```
/// let mut Resolver::new();
/// let mut environment = Environment::new();
/// let statements = vec![/* ... */];
/// let result = resolver.resolve(&statements, &mut environment);
/// ```
///
/// ### Last Updated: (v3.0.0)
#[derive(Debug, Clone)]
pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_loop: LoopType,
    locals: HashMap<usize, usize>,
}
impl Resolver {
    /// The `new()` function is a constructor for the `Resolver` struct.
    ///
    /// It initializes the `Resolver` with an empty stack of scopes, a `None` current function, a `None` current loop, and an empty map of locals.
    ///
    /// # Return Value
    ///
    /// A new instance of the `Resolver` struct.
    ///
    /// # Usage
    ///
    /// The `new()` function is typically called when you want to create a new `Resolver` to resolve symbols in the Tron language.
    ///
    /// # Example
    ///
    /// ```
    /// let resolver = Resolver::new();
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    pub fn new() -> Self {
        Self {
            scopes: vec![],
            current_function: FunctionType::None,
            current_loop: LoopType::None,
            locals: HashMap::new(),
        }
    }
    /// Resolves a single statement within the given environment.
    ///
    /// This method is responsible for handling various types of statements, including blocks, variable declarations, function declarations, expressions, and control flow statements like if, while, and switch. It recursively resolves nested statements and expressions, ensuring that all symbols are correctly resolved within the current scope.
    ///
    /// # Parameters
    ///
    /// - `stmt`: A reference to the statement to be resolved.
    /// - `environment`: A mutable reference to the environment in which the statement is being resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the statement is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let mut environment = Environment::new();
    /// let statement = Statement::VariableStatement
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if a return statement is encountered outside of a function context.
    /// - Panics if a break statement is encountered outside of a loop context.
    /// - Panics if a variable is declared with a mismatched type.
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_internal(
        &mut self,
        stmt: &Statement,
        environment: &mut Environment,
    ) -> Result<(), String> {
        match stmt {
            Statement::BlockStatement {
                statements: _,
                line,
            } => self.resolve_block(stmt, environment, *line)?,
            Statement::VariableStatement {
                name: _,
                value_type: _,
                value: _,
                line,
            } => self.resolve_var(stmt, environment, *line)?,
            Statement::FunctionStatement {
                name: _,
                params: _,
                body: _,
                output_type: _,
                line: _,
            } => self.resolve_function(stmt, FunctionType::Function, environment)?,
            Statement::ExpressionStatement { expression, line } => {
                self.resolve_expr(expression, *line, environment)?
            }
            Statement::IfStatement {
                conditions: _,
                then_branch: _,
                elif_branches: _,
                else_branch: _,
                line,
            } => self.resolve_if_stmt(stmt, environment, *line)?,
            Statement::UseStatement { expression, line } => {
                self.resolve_expr(expression, *line, environment)?
            }
            Statement::ReturnStatement {
                keyword: _,
                value,
                line,
            } => {
                if self.current_function == FunctionType::None {
                    TronError::throw("E3006", *line, vec![]);
                } else if let Some(value) = value {
                    self.resolve_expr(value, *line, environment)?;
                }
            }
            Statement::WhileStatement {
                conditions,
                body,
                line,
            } => {
                for condition in conditions {
                    self.resolve_expr(condition, *line, environment)?;
                }
                self.resolve_internal(body.as_ref(), environment)?;
            }
            Statement::BreakStatement { keyword: _, line } => {
                if self.current_loop == LoopType::None {
                    TronError::throw("E3007", *line, vec![]);
                }
            }
            Statement::SwitchStatement {
                condition,
                case_branches,
                default_branch,
                line,
            } => {
                self.resolve_expr(condition, *line, environment)?;
                for case_branch in case_branches {
                    for branch in case_branch.1.clone() {
                        self.resolve_internal(&branch, environment)?;
                    }
                }
                if let Some(default_branch) = default_branch {
                    for branch in default_branch {
                        self.resolve_internal(&branch, environment)?;
                    }
                }
            }
        }
        Ok(())
    }
    /// Resolves a collection of statements within the given environment.
    ///
    /// This method iterates over a collection of statements and resolves each one using the `resolve_internal` method. It's designed to handle multiple statements in sequence, ensuring that all symbols within the statements are correctly resolved within the current scope.
    ///
    /// # Parameters
    ///
    /// - `stmts`: A reference to a vector of statements to be resolved.
    /// - `environment`: A mutable reference to the environment in which the statements are being resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if all statements are successfully resolved, or an error message if resolution fails for any statement.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let mut environment = Evnrionment::new();
    /// let statements = vec![/* ... */];
    /// resolver.resolve_many(&statements, &mut environment)?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_many(
        &mut self,
        stmts: &Vec<&Statement>,
        environment: &mut Environment,
    ) -> Result<(), String> {
        for stmt in stmts {
            self.resolve_internal(stmt, environment)?;
        }
        Ok(())
    }
    /// Resolves a collection of statements within the given environment and returns a map of local variable IDs to their scope depth.
    ///
    /// This method is a wrapper around `resolve_many` that also returns the `locals` map of the `Resolver` instance. It's designed to resolve multiple statements in sequence and then provide information about the local variables that were resolved.
    ///
    /// # Parameters
    ///
    /// - `stmts`: A reference to a vector of statements to be resolved.
    /// - `environment`: A mutable reference to the environment in which the statements are being resolved.
    ///
    /// # Returns
    ///
    /// - `Result<HashMap<usize, usize>, String>`: Returns a `HashMap` mapping local variable IDs to their scope depth if all statements are successfully resolved, or an error message if resolution fails for any statement.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolve::new();
    /// let mut environment = Environment::new();
    /// let statements = vec![/* ... */];
    /// let result = resolver.resolve(&statements, &mut environment)?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    pub fn resolve(
        mut self,
        stmts: &Vec<&Statement>,
        environment: &mut Environment,
    ) -> Result<HashMap<usize, usize>, String> {
        self.resolve_many(stmts, environment)?;
        Ok(self.locals)
    }
    /// Resolves a block statement within the given environment.
    ///
    /// This method is responsible for handling block statements, which are essentially a collection of statements enclosed within curly braces `{}`. It begins a new scope for the block, resolves all statements within the block, and then ends the scope. This ensures that any variables declared within the block are properly scoped and do not leak into the surrounding environment.
    ///
    /// # Parameters
    ///
    /// - `stmt`: A reference to the block statement to be resolved.
    /// - `environment`: A mutable reference to the environment in which the block statement is being resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the block statement is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let mut environment = Environment::new();
    /// let block_statement = Statement::BlockStatement {
    ///  statements: vec![/* ... */]
    /// };
    /// resolver.resolve_block(&block_statement, &mut environment)?;
    /// ```
    ///
    ///
    /// # Panics
    ///
    /// - Panics if the provided statement is not a block statement.
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_block(
        &mut self,
        stmt: &Statement,
        environment: &mut Environment,
        line: usize,
    ) -> Result<(), String> {
        match stmt {
            Statement::BlockStatement {
                statements,
                line: _,
            } => {
                self.begin_scope();
                self.resolve_many(
                    &statements.iter().map(|b| b.as_ref()).collect(),
                    environment,
                )?;
                self.end_scope();
            }
            _ => TronError::throw("E3001", line, vec!["block".to_string()]),
        }
        Ok(())
    }
    /// Resolves a variable declaration statement within the given environment.
    ///
    /// This method is responsible for handling variable declarations. It declares the variable in the current scope, checks for type mismatches if a type is specified, and then defines the variable. This ensures that variables are correctly declared and initialized within the current scope.
    ///
    /// # Parameters
    ///
    /// - `stmt`: A reference to the variable declaration statement to be resolved.
    /// - `environment`: A mutable reference to the environment in which the variable declaration is being resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the variable declaration is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let mut environment = Environment::new();
    /// let variable_statement = Statement::VariableStatement {/* ... */};
    /// resolver.resolve_var(&variable_statement, &mut environment)?;
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the provided statement is not a variable declaration statement.
    /// - Panics if a variable is declared with a mismatched type.
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_var(
        &mut self,
        stmt: &Statement,
        environment: &mut Environment,
        line: usize,
    ) -> Result<(), String> {
        if let Statement::VariableStatement {
            name,
            value_type,
            value,
            line,
        } = stmt
        {
            self.declare(name)?;
            let new_value = (*value).evaluate(environment.clone())?;
            let value_clone = new_value.clone();

            if value_type.lexeme == value_clone.to_type()
                || value_type.lexeme == value_clone.to_string()
            {
                environment.set_value_type(name.lexeme.clone(), value_type.lexeme.clone());
            } else {
                TronError::throw(
                    "E4003",
                    *line,
                    vec![
                        "variable".to_string(),
                        name.lexeme.to_string(),
                        value_type.lexeme.to_string(),
                        value_clone.to_type().to_string(),
                    ],
                );
            }
            self.resolve_expr(value, *line, environment)?;
            self.define(name);
        } else {
            TronError::throw("E3001", line, vec!["variable".to_string()]);
        }
        Ok(())
    }
    /// Resolves a function declaration statement within the given environment.
    ///
    /// This method is responsible for handling function declarations. It declares the function in the current scope, checks for type mismatches if a type is specified, and then defines the function. This ensures that functions are correctly declared and initialized within the current scope.
    ///
    /// # Parameters
    ///
    /// - `stmt`: A reference to the function declaration statement to be resolved.
    /// - `resolving_function`: The type of the function being resolved.
    /// - `environment`: A mutable reference to the environment in which the function declaration is being resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the function declaration is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let mut environment = Environment::new();
    /// let function_statement = Statement::FunctionStatement {/* ... */};
    /// resolver.resolve_function(&function_statement, FunctionType::Function, &mut environment)?;
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the provided statement is not a function declaration statement.
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_function(
        &mut self,
        stmt: &Statement,
        resolving_function: FunctionType,
        environment: &mut Environment,
    ) -> Result<(), String> {
        if let Statement::FunctionStatement {
            name: _,
            params,
            body,
            output_type: _,
            line: _,
        } = stmt
        {
            let enclosing_function = self.current_function;
            self.current_function = resolving_function;
            self.begin_scope();
            for (param_name, _param_type) in params {
                self.declare(param_name)?;
                self.define(param_name);
            }
            self.resolve_many(&body.iter().map(|b| b.as_ref()).collect(), environment)?;
            self.end_scope();
            self.current_function = enclosing_function;
            Ok(())
        } else {
            panic!("resolve_function called with non-function statement");
        }
    }
    /// Resolves an if statement within the given environment.
    ///
    /// This method is responsible for handling if statements, which are conditional branches in the code. It resolves the conditions and the branches of the if statement, ensuring that all symbols within the branches are correctly resolved within the current scope.
    ///
    /// # Parameters
    ///
    /// - `stmt`: A reference to the if statement to be resolved.
    /// - `environment`: A mutable reference to the environment in which the if statement is being resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the if statement is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolve::new();
    /// let mut environment = Statement:IfStatement {/* ... */};
    /// resolver.resolve_if_stmt(&if_statement, &mut environment)?;
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the provided statement is not an if statement.
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_if_stmt(
        &mut self,
        stmt: &Statement,
        environment: &mut Environment,
        line: usize,
    ) -> Result<(), String> {
        if let Statement::IfStatement {
            conditions,
            then_branch: then,
            elif_branches,
            else_branch: els,
            line,
        } = stmt
        {
            for condition in conditions {
                self.resolve_expr(condition, *line, environment)?;
            }
            self.resolve_internal(then.as_ref(), environment)?;
            for (elif_predicates, elif_stmt) in elif_branches {
                for elif_predicate in elif_predicates {
                    self.resolve_expr(elif_predicate, *line, environment)?;
                }
                self.resolve_internal(elif_stmt.as_ref(), environment)?;
            }
            if let Some(els) = els {
                self.resolve_internal(els.as_ref(), environment)?;
            }
            Ok(())
        } else {
            TronError::throw("E3001", line, vec!["if".to_string()]);
            Ok(())
        }
    }
    /// Begins a new scope for variable resolution.
    ///
    /// This method is called to start a new scope, which is typically used when entering a block of code. It pushes a new `HashMap` onto the `scopes` stack to track variables declared within this scope.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// resolver.begin_scope();
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    /// Ends the current scope for variable resolution.
    ///
    /// This method is called to end the current scope, which is typically used when exiting a block of code. It pops the last `HashMap` from the `scopes` stack, effectively closing the scope and making any variables declared within it inaccessible.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// resolver.begin_scope();
    /// // ... resolve statements within the scope ...
    /// resolver.end_scope();
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn end_scope(&mut self) {
        self.scopes.pop().expect("Stack underflow");
    }
    /// Declares a variable in the current scope.
    ///
    /// This method is used to declare a variable in the current scope. It checks if the variable is already declared in the current scope and panics if it is. Otherwise, it adds the variable to the current scope's `HashMap`.
    ///
    /// # Parameters
    ///
    /// - `name`: The token representing the variable name to be declared.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the variable is successfully declared, or an error message if the variable is already declared in the current scope.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let variable_name = Token::new(TokenType::identifier, "testVar", None, 1);
    /// resolver.declare(&variable_name)?;
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if a variable with the same name is already declared in the current scope.
    ///
    /// ### Last Updated: (v3.0.0)
    fn declare(&mut self, name: &Token) -> Result<(), String> {
        let size = self.scopes.len();
        if self.scopes.is_empty() {
            return Ok(());
        } else if self.scopes[size - 1].contains_key(&name.lexeme.clone()) {
            TronError::throw("E3004", name.line_number, vec![]);
        }
        self.scopes[size - 1].insert(name.lexeme.clone(), false);
        Ok(())
    }
    /// Defines a variable in the current scope.
    ///
    /// This method is used to define a variable in the current scope after it has been declared. It marks the variable as initialized in the current scope's `HashMap`.
    ///
    /// # Parameters
    ///
    /// - `name`: The token representing the variable name to be defined.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let variable_name = TokenType::Identifier, "testVar", None, 1);
    /// resolver.declare(&variable_name)?;
    /// resolver.define(&variable_name);
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let size = self.scopes.len();
        self.scopes[size - 1].insert(name.lexeme.clone(), true);
    }
    /// Resolves an expression within the given environment.
    ///
    /// This method is responsible for handling various types of expressions, including literals, variables, binary operations, unary operations, and function calls. It recursively resolves nested expressions, ensuring that all symbols are correctly resolved within the current scope.
    ///
    /// # Parameters
    ///
    /// - `expr`: A reference to the expression to be resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the expression is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let mut environment = Environment::new();
    /// let expression = Expressiion::Variable {id : 0, name: Token::new(TokenTYpe::Identifier, "testVar", None, 1)};
    /// resolver.resolve_expr(&expression)?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_expr(
        &mut self,
        expr: &Expression,
        line: usize,
        environment: &mut Environment,
    ) -> Result<(), String> {
        match expr {
            Expression::Function {
                id: _,
                name: _,
                params,
                body,
                output_type: _,
            } => {
                let enclosing_function = self.current_function;
                self.current_function = FunctionType::Function;
                self.begin_scope();
                for (param_name, _param_type) in params {
                    self.declare(param_name)?;
                    self.define(param_name);
                }
                self.resolve_many(&body.iter().map(|b| b.as_ref()).collect(), environment)?;
                self.end_scope();
                self.current_function = enclosing_function;
                Ok(())
            }
            Expression::Object { id: _, properties } => {
                for (_, value_expr) in properties {
                    self.resolve_expr(value_expr, line, environment)?;
                }
                Ok(())
            }
            Expression::ObjectCall {
                id: _,
                key: _,
                name: _,
            } => Ok(()), //
            Expression::Variable { id: _, name: _ } => {
                self.resolve_expr_var(expr, expr.get_id(), line)
            }
            Expression::Assign {
                id: _,
                name: _,
                value: _,
            } => self.resolve_expr_assign(expr, expr.get_id(), line, environment),
            Expression::Array { id: _, elements } => {
                for element in elements {
                    self.resolve_expr(element, line, environment)?;
                }
                Ok(())
            }
            Expression::Binary {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left, line, environment)?;
                self.resolve_expr(right, line, environment)
            }
            Expression::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(callee.as_ref(), line, environment)?;
                for arg in arguments {
                    self.resolve_expr(arg, line, environment)?;
                }
                Ok(())
            }
            Expression::Grouping { id: _, expression } => {
                self.resolve_expr(expression, line, environment)
            }
            Expression::Literal { id: _, value: _ } => Ok(()),
            Expression::Logical {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left, line, environment)?;
                self.resolve_expr(right, line, environment)
            }
            Expression::Unary {
                id: _,
                operator: _,
                right,
            } => self.resolve_expr(right, line, environment),
        }
    }
    /// Resolves a variable expression within the given environment.
    ///
    /// This method is used to resolve a variable expression, ensuring that the variable is declared and initialized in the current scope. It also handles the resolution of function calls where the callee is a variable.
    ///
    /// # Parameters
    ///
    /// - `expr`: A reference to the expression to be resolved.
    /// - `resolve_id`: The ID of the expression to be resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the variable expression is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let mut environment = Environment::new();
    /// let variable_expression = Expressiion::Variable {id : 0, name: Token::new(TokenTYpe::Identifier, "testVar", None, 1)};
    /// resolver.resolve_expr_var(&variable_expression, variable_expression.get_id())?;
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the provided expression is not a variable expression or a function call with a variable callee.
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_expr_var(
        &mut self,
        expr: &Expression,
        resolve_id: usize,
        line: usize,
    ) -> Result<(), String> {
        match expr {
            Expression::Variable { id: _, name } => {
                if !self.scopes.is_empty() {
                    if let Some(false) = self.scopes[self.scopes.len() - 1].get(&name.lexeme) {
                        TronError::throw("E3003", name.line_number, vec![]);
                    }
                }
                self.resolve_local(name, resolve_id)
            }
            Expression::Call {
                id: _,
                callee,
                paren: _,
                arguments: _,
            } => match callee.as_ref() {
                Expression::Variable { id: _, name } => self.resolve_local(&name, resolve_id),
                _ => {
                    TronError::throw("E3001", line, vec!["variable".to_string()]);
                    Ok(())
                }
            },
            _ => {
                TronError::throw("E3001", line, vec!["variable".to_string()]);
                Ok(())
            }
        }
    }
    /// Resolves a local variable within the given environment.
    ///
    /// This method is used to resolve a local variable, ensuring that the variable is declared and initialized in the current scope. It also handles the resolution of function calls where the callee is a variable.
    ///
    /// # Parameters
    ///
    /// - `name`: The token representing the variable name to be resolved.
    /// - `resolve_id`: The ID of the expression to be resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the local variable is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut resolver = Resolver::new();
    /// let variable_name = Token::new(TokenType::Identifier, "testVar", None, 1);
    /// resolver.resolve_local(&variable_name, variable_name.get_id())?
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_local(&mut self, name: &Token, resolve_id: usize) -> Result<(), String> {
        let size = self.scopes.len();
        if size == 0 {
            return Ok(());
        }
        for i in (0..=(size - 1)).rev() {
            let scope = &self.scopes[i];
            if scope.contains_key(&name.lexeme) {
                self.locals.insert(resolve_id, size - 1 - i);
                return Ok(());
            }
        }
        Ok(())
    }
    /// Resolves an assignment expression within the given environment.
    ///
    /// This method is used to resolve an assignment expression, ensuring that the variable being assigned to is declared and initialized in the current scope. It also handles the resolution of function calls where the callee is a variable.
    ///
    /// # Parameters
    ///
    /// - `expr`: A reference to the expression to be resolved.
    /// - `resolve_id`: The ID of the expression to be resolved.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the assignment expression is successfully resolved, or an error message if resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let mut resolver = Resolver::new();
    /// let mut environment = Environment::new();
    /// let assignment_expression = Expression::Assign { /* ... */ };
    /// resolver.resolve_expr_assign(&assignment_expression, assignment_expression.get_id())?;
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the provided expression is not an assignment expression.
    ///
    /// ### Last Updated: (v3.0.0)
    fn resolve_expr_assign(
        &mut self,
        expr: &Expression,
        resolve_id: usize,
        line: usize,
        environment: &mut Environment,
    ) -> Result<(), String> {
        if let Expression::Assign { id: _, name, value } = expr {
            self.resolve_expr(value.as_ref(), line, environment)?;
            self.resolve_local(name, resolve_id)?;
        } else {
            TronError::throw("E3001", line, vec!["assign".to_string()]);
        }
        Ok(())
    }
}
