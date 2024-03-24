use crate::expressions::{Expression, Expression::*, TronType};
use crate::scanner::{Statement, Token, TokenType, TokenType::*};
use crate::utils::TronError;
/// The `Parser` struct in Rust is responsible for parsing.
/// It maintains a list of tokens and provides methods to parse statements and expressions.
///
/// # Fields
///
/// - `tokens`: A vector of tokens that the parser will process.
/// - `current`: The index of the current token being parsed.
/// - `next_id`: A counter for generating unique IDs for expressions and statements.
///
/// # Usage
///
/// The `Parser` struct is used to parse Tron code into an abstract syntax tree (AST). It is typically instantiated with the `new` method and then used to parse a sequence of tokens into statements and expressions.
///
/// # Example
///
/// ```
/// let tokens = vec![/* ... */];
/// let mut parser = Parser::new(tokens);
/// let statements = parser.parse()?;
/// ```
///
/// ### Last Updated: (v3.0.0)
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    next_id: usize,
}
impl Parser {
    /// The `new()` function is a constructor for the `Parser` struct.
    ///
    /// It initializes the `Parser` with a given vector of tokens, sets the current token index to 0, and initializes the `next_id` counter to 0.
    ///
    /// # Parameters
    ///
    /// - `tokens`: A vector of tokens that the parser will process.
    ///
    /// # Return Value
    ///
    /// A new instance of the `Parser` struct.
    ///
    /// # Usage
    ///
    /// The `new()` function is typically called when you want to create a new `Parser` to parse tokens into statements and expressions.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            next_id: 0,
        }
    }
    /// The `get_id()` method is used to generate a unique ID for expressions and statements.
    ///
    /// It increments the `next_id` counter and returns the current value, ensuring that each expression or statement has a unique ID.
    ///
    /// # Return Value
    ///
    /// The current value of the `next_id` counter, which is then incremented.
    ///
    /// # Usage
    ///
    /// The `get_id()` method is called internally by the `Parser` to assign unique IDs to expressions and statements during parsing.
    ///
    /// # Example
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let id = parser.get_id();
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    /// The `parse()` method is responsible for parsing a sequence of tokens into statements.
    ///
    /// It iterates over the tokens, processing them into statements and expressions, and returns a vector of statements.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a vector of `Statement` objects if the parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `parse()` method is called to convert a sequence of tokens into a list of statements that can be executed or further processed.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let statements = parser.parse()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmts = vec![];
        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    TronError::throw("E2001", self.current, vec![msg]);
                }
            }
        }
        Ok(stmts)
    }
    /// The `declaration()` method is responsible for parsing declarations.
    ///
    /// It handles various types of declarations, including variable declarations, function declarations, and block statements.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object if the declaration is successfully parsed, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `declaration()` method is called internally by the `Parser` to process declarations within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let statement = parser.declaration()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn declaration(&mut self) -> Result<Statement, String> {
        if self.match_token(Variable) {
            self.var_declaration()
        } else if self.match_token(Function) {
            self.function()
        } else {
            self.statement()
        }
    }
    /// The `function()` method is responsible for parsing function declarations.
    ///
    /// It handles the parsing of function names, parameters, body, and output type. It ensures that the function declaration is syntactically correct and constructs a `FunctionStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the function declaration if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `function()` method is called internally by the `Parser` to process function declarations within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let function_statement = parser.function()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn function(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let name = self.consume(Identifier, &format!("expected function name"), line_number)?;
        self.consume(
            LeftParen,
            &format!("expected '(' after function name"),
            line_number,
        )?;
        let mut params: Vec<(Token, Option<Token>)> = vec![];
        if !self.check(RightParen) {
            loop {
                if params.len() >= 32 {
                    TronError::throw("E2004", self.current, vec![]);
                }
                let param_name =
                    self.consume(Identifier, "expected parameter name", line_number)?;
                let param_type = if self.match_token(Colon) {
                    Some(self.consume(Identifier, "expected type after ':'", line_number)?)
                } else {
                    None
                };
                params.push((param_name, param_type));
                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(RightParen, "expected ')' after parameters.", line_number)?;
        let output_type = if self.match_token(Colon) {
            Some(self.consume(Identifier, "expected type after ':'", line_number)?)
        } else {
            None
        };
        if self.match_token(Equal) {
            let body_expr = self.expression()?;
            self.consume(
                Semicolon,
                "expected ';' after function body expression.",
                line_number,
            )?;
            return Ok(Statement::FunctionStatement {
                name,
                params,
                body: vec![Box::new(Statement::ReturnStatement {
                    keyword: Token {
                        token_type: TokenType::Return,
                        lexeme: "".to_string(),
                        line_number: line_number,
                        literal: None,
                    },
                    value: Some(body_expr),
                    line: line_number,
                })],
                output_type,
                line: line_number,
            });
        }
        self.consume(
            LeftBrace,
            &format!("Expected 'start' before function body."),
            line_number,
        )?;
        let body = match self.block_statement()? {
            Statement::BlockStatement {
                statements,
                line: _,
            } => statements,
            _ => {
                TronError::throw("E2002", self.current, vec![]);
                vec![]
            }
        };
        Ok(Statement::FunctionStatement {
            name,
            params,
            body,
            output_type,
            line: line_number,
        })
    }
    /// The `var_declaration()` method is responsible for parsing variable declarations.
    ///
    /// It handles the parsing of variable names and their initial values. It ensures that the variable declaration is syntactically correct and constructs a `VariableStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the variable declaration if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `var_declaration()` method is called internally by the `Parser` to process variable declarations within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let variable_statement = parser.var_declaration()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn var_declaration(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let name = self.consume(Identifier, "Expected variable name", line_number)?;
        let value_type = if self.match_token(Colon) {
            if self.match_tokens(&[Identifier, StringLit, Number]) {
                Some(self.previous(1))
            } else {
                return Err(format!("Expected type after ':'"));
            }
        } else {
            None
        };
        self.consume(Equal, "Expected '=' after variable name", line_number)?;
        let value = self.expression()?;
        self.consume(
            Semicolon,
            "Expected ';' after variable declaration",
            line_number,
        )?;
        Ok(Statement::VariableStatement {
            name,
            value_type,
            value,
            line: line_number,
        })
    }
    /// The `statement()` method is responsible for parsing various types of statements.
    ///
    /// It handles different types of statements, including blocks, variable declarations, function declarations, expressions, and control flow statements like if, while, and switch. It recursively resolves nested statements and expressions, ensuring that all symbols are correctly resolved within the current scope.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object if the statement is successfully parsed, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `statement()` method is called internally by the `Parser` to process statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let statement = parser.statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_token(LeftBrace) {
            self.block_statement()
        } else if self.match_token(Use) {
            self.use_statement()
        } else if self.match_token(If) {
            self.if_statement()
        } else if self.match_token(While) {
            self.while_statement()
        } else if self.match_token(For) {
            self.for_statement()
        } else if self.match_token(Return) {
            self.return_statement()
        } else if self.match_token(Break) {
            self.break_statement()
        } else if self.match_token(Switch) {
            self.switch_statement()
        } else {
            self.expression_statement()
        }
    }
    /// The `switch_statement()` method is responsible for parsing switch statements.
    ///
    /// It handles the parsing of the switch condition, case branches, and the optional default branch. It ensures that the switch statement is syntactically correct and constructs a `SwitchStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the switch statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `switch_statement()` method is called internally by the `Parser` to process switch statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let switch_statement = parser.switch_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn switch_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let condition = self.expression()?;
        self.consume(LeftBrace, "Expected '{' after match value.", line_number)?;
        let mut case_branches: Vec<(Expression, Vec<Statement>)> = Vec::new();
        while self.match_token(Case) {
            let case_value = self.expression()?;
            self.consume(LeftBrace, "Expected Start after case value.", line_number)?;
            let mut case_body = Vec::new();
            while !self.check(RightBrace) && !self.check(Case) && !self.check(Default) {
                let stmt = self.declaration()?;
                case_body.push(stmt);
            }
            self.consume(RightBrace, "Expected End after case body.", line_number)?;
            case_branches.push((case_value, case_body));
        }
        let mut default_branch = None;
        if self.match_token(Default) {
            self.consume(
                LeftBrace,
                "Expected Start after default keyword.",
                line_number,
            )?;
            let mut default_body = Vec::new();
            while !self.check(RightBrace) {
                let stmt = self.declaration()?;
                default_body.push(stmt);
            }
            self.consume(RightBrace, "Expected End after default body.", line_number)?;
            default_branch = Some(default_body);
        }
        self.consume(
            RightBrace,
            "Expected End after switch statement.",
            line_number,
        )?;
        Ok(Statement::SwitchStatement {
            condition,
            case_branches,
            default_branch,
            line: line_number,
        })
    }
    /// The `return_statement()` method is responsible for parsing return statements.
    ///
    /// It handles the parsing of the return keyword and the optional expression that follows it. It ensures that the return statement is syntactically correct and constructs a `ReturnStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the return statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `return_statement()` method is called internally by the `Parser` to process return statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let return_statement = parser.return_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn return_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let keyword = self.previous(1);
        let value;
        if !self.check(Semicolon) {
            value = Some(self.expression()?);
        } else {
            value = None;
        }
        self.consume(Semicolon, "Expected ';' after return value;", line_number)?;
        Ok(Statement::ReturnStatement {
            keyword,
            value,
            line: line_number,
        })
    }
    /// The `break_statement()` method is responsible for parsing break statements.
    ///
    /// It handles the parsing of the break keyword and ensures that the break statement is syntactically correct and constructs a `BreakStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the break statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `break_statement()` method is called internally by the `Parser` to process break statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let break_statement = parser.break_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn break_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let keyword = self.previous(1);
        self.consume(
            Semicolon,
            "Expected Semicolon after return value",
            line_number,
        )?;
        Ok(Statement::BreakStatement {
            keyword,
            line: line_number,
        })
    }
    /// The `for_statement()` method is responsible for parsing for loop statements.
    ///
    /// It handles the parsing of the for loop initialization, condition, and increment, as well as the loop body. It ensures that the for loop statement is syntactically correct and constructs a `WhileStatement` object that represents the for loop.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the for loop statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `for_statement()` method is called internally by the `Parser` to process for loop statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let for_loop_statement = parser.for_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn for_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let initializer;
        if self.match_token(Semicolon) {
            initializer = None;
        } else if self.match_token(Variable) {
            let var_decl = self.var_declaration()?;
            initializer = Some(var_decl);
        } else {
            let expr = self.expression_statement()?;
            initializer = Some(expr);
        }
        let condition;
        if !self.check(Semicolon) {
            let expr = self.expression()?;
            condition = Some(expr);
        } else {
            condition = None;
        }
        self.consume(Semicolon, "Expected ';' after loop condition.", line_number)?;
        let increment;
        if !self.check(RightParen) {
            let expr = self.expression()?;
            increment = Some(expr);
        } else {
            increment = None;
        }
        let mut body = self.statement()?;
        if let Some(incr) = increment {
            body = Statement::BlockStatement {
                statements: vec![
                    Box::new(body),
                    Box::new(Statement::ExpressionStatement {
                        expression: incr,
                        line: line_number,
                    }),
                ],
                line: line_number,
            };
        }
        let cond;
        match condition {
            None => {
                cond = Expression::Literal {
                    id: self.get_id(),
                    value: TronType::True,
                }
            }
            Some(c) => cond = c,
        }
        body = Statement::WhileStatement {
            conditions: vec![cond],
            body: Box::new(body),
            line: line_number,
        };
        if let Some(init) = initializer {
            body = Statement::BlockStatement {
                statements: vec![Box::new(init), Box::new(body)],
                line: line_number,
            };
        }
        Ok(body)
    }
    /// The `while_statement()` method is responsible for parsing while loop statements.
    ///
    /// It handles the parsing of the while loop condition and the loop body. It ensures that the while loop statement is syntactically correct and constructs a `WhileStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the while loop statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `while_statement()` method is called internally by the `Parser` to process while loop statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let while_loop_statement = parser.while_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn while_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let mut conditions = Vec::new();
        loop {
            let condition = self.expression()?;
            conditions.push(condition);
            if !self.match_token(Comma) {
                break;
            }
        }
        let body = self.statement()?;
        Ok(Statement::WhileStatement {
            conditions,
            body: Box::new(body),
            line: line_number,
        })
    }
    /// The `if_statement()` method is responsible for parsing if statements.
    ///
    /// It handles the parsing of the if condition, the then branch, and any optional elif branches and an else branch. It ensures that the if statement is syntactically correct and constructs an `IfStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the if statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `if_statement()` method is called internally by the `Parser` to process if statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let if_statement = parser.if_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn if_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let mut conditions = Vec::new();
        loop {
            let condition = self.expression()?;
            conditions.push(condition);
            if !self.match_token(Comma) {
                break;
            }
        }
        let then_branch = Box::new(self.statement()?);
        let mut elif_branches = Vec::new();
        while self.match_token(Elif) {
            let mut elif_predicates = Vec::new();
            loop {
                let elif_predicate = self.expression()?;
                elif_predicates.push(elif_predicate);
                if !self.match_token(Comma) {
                    break;
                }
            }
            let elif_stmt = Box::new(self.statement()?);
            elif_branches.push((elif_predicates, elif_stmt));
        }
        let else_branch = if self.match_token(Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Statement::IfStatement {
            conditions,
            then_branch,
            elif_branches,
            else_branch,
            line: line_number,
        })
    }
    /// The `block_statement()` method is responsible for parsing block statements.
    ///
    /// It handles the parsing of a block of code enclosed within curly braces `{}`. It begins a new scope for the block, resolves all statements within the block, and then ends the scope. This ensures that any variables declared within the block are properly scoped and do not leak into the surrounding environment.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the block statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `block_statement()` method is called internally by the `Parser` to process block statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let block_statement = parser.block_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn block_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let mut statements = vec![];
        while !self.check(RightBrace) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(Box::new(decl));
        }
        self.consume(RightBrace, "Expected 'end' after a block", line_number)?;
        Ok(Statement::BlockStatement {
            statements,
            line: line_number,
        })
    }
    /// The `use_statement()` method is responsible for parsing use statements.
    ///
    /// It handles the parsing of the use keyword and the expression that follows it, which typically represents the module or resource to be used. It ensures that the use statement is syntactically correct and constructs an `UseStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the use statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `use_statement()` method is called internally by the `Parser` to process use statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let import_statement = parser.use_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn use_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let expression = self.expression()?;
        self.consume(Semicolon, "Expected ';' after value.", line_number)?;
        Ok(Statement::UseStatement {
            expression,
            line: line_number,
        })
    }
    /// The `expression_statement()` method is responsible for parsing expression statements.
    ///
    /// It handles the parsing of expressions that are not part of a larger statement, such as standalone expressions or assignments. It ensures that the expression statement is syntactically correct and constructs an `ExpressionStatement` object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a `Statement` object representing the expression statement if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `expression_statement()` method is called internally by the `Parser` to process expression statements within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let expression_statement = parser.expression_statement()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn expression_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line_number;
        let expression = self.expression()?;
        self.consume(Semicolon, "Expected ';' after expression.", line_number)?;
        Ok(Statement::ExpressionStatement {
            expression,
            line: line_number,
        })
    }
    /// The `expression()` method is responsible for parsing expressions.
    ///
    /// It handles the parsing of various types of expressions, including literals, variables, binary operations, unary operations, and function calls. It recursively resolves nested expressions, ensuring that all symbols are correctly resolved within the current scope.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object if the expression is successfully parsed, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `expression()` method is called internally by the `Parser` to process expressions within the code.
    ///
    /// # Example
    ///
    /// ```
    /// let tokens = vec![/* ... */];
    /// let mut parser = Parser::new(tokens);
    /// let expression = parser.expression()?;
    /// ```
    ///
    /// ### Last Updated: (v3.0.0)
    fn expression(&mut self) -> Result<Expression, String> {
        let expr = self.or()?;
        if self.match_token(Equal) {
            let value = self.expression()?;
            match expr {
                Expression::Variable { id: _, name } => Ok(Assign {
                    id: self.get_id(),
                    name,
                    value: Box::from(value),
                }),
                _ => {
                    TronError::throw("E2007", self.current, vec![]);
                    Ok(Expression::Literal {
                        id: self.get_id() * 19,
                        value: TronType::Null,
                    })
                }
            }
        } else {
            Ok(expr)
        }
    }
    /// The `or()` method is responsible for parsing logical OR expressions.
    ///
    /// It handles the parsing of expressions with the OR operator and constructs a `Logical` expression object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the logical OR expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `or()` method is called internally by the `Parser` to process logical OR expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn or(&mut self) -> Result<Expression, String> {
        let mut expr = self.nor()?;
        while self.match_token(Or) {
            let operator = self.previous(1);
            let right = self.and()?;
            expr = Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    /// The `nor()` method is responsible for parsing logical NOR expressions.
    ///
    /// It handles the parsing of expressions with the NOR operator and constructs a `Logical` expression object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the logical NOR expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `nor()` method is called internally by the `Parser` to process logical NOR expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn nor(&mut self) -> Result<Expression, String> {
        let mut expr = self.xor()?;
        while self.match_token(Nor) {
            let operator = self.previous(1);
            let right = self.and()?;
            expr = Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    /// The `xor()` method is responsible for parsing logical XOR expressions.
    ///
    /// It handles the parsing of expressions with the XOR operator and constructs a `Logical` expression object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the logical XOR expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `xor()` method is called internally by the `Parser` to process logical XOR expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn xor(&mut self) -> Result<Expression, String> {
        let mut expr = self.and()?;
        while self.match_token(Xor) {
            let operator = self.previous(1);
            let right = self.and()?;
            expr = Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    /// The `and()` method is responsible for parsing logical AND expressions.
    ///
    /// It handles the parsing of expressions with the AND operator and constructs a `Logical` expression object.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the logical AND expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `and()` method is called internally by the `Parser` to process logical AND expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn and(&mut self) -> Result<Expression, String> {
        let mut expr = self.equality()?;
        while self.match_token(And) {
            let operator = self.previous(1);
            let right = self.equality()?;
            expr = Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    /// The `equality()` method is responsible for parsing equality expressions.
    ///
    /// It handles the parsing of expressions with equality operators (e.g., `==`, `!=`). It constructs a `Binary` expression object representing the equality expression.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the equality expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `equality()` method is called internally by the `Parser` to process equality expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.comparasion()?;
        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous(1);
            let rhs = self.comparasion()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }
    /// The `comparasion()` method is responsible for parsing comparasion expressions.
    ///
    /// It handles the parsing of expressions with comparasion operators (e.g., `<`, `>`, `<=`, `>=`). It constructs a `Binary` expression object representing the comparasion expression.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the comparasion expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `comparasion()` method is called internally by the `Parser` to process comparasion expressions within the code.
    ///
    fn comparasion(&mut self) -> Result<Expression, String> {
        let mut expr = self.term()?;
        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous(1);
            let rhs = self.term()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }
    /// The `term()` method is responsible for parsing term expressions.
    ///
    /// It handles the parsing of expressions with addition and subtraction operators. It constructs a `Binary` expression object representing the term expression.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the term expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `term()` method is called internally by the `Parser` to process term expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;
        while self.match_tokens(&[Minus, Plus]) {
            let op = self.previous(1);
            let rhs = self.factor()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }
    /// The `factor()` method is responsible for parsing factor expressions.
    ///
    /// It handles the parsing of expressions with multiplication, division, and exponentiation operators. It constructs a `Binary` expression object representing the factor expression.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the factor expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `factor()` method is called internally by the `Parser` to process factor expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[Slash, Star, Power]) {
            let op = self.previous(1);
            let rhs = self.unary()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }
    /// The `unary()` method is responsible for parsing unary expressions.
    ///
    /// It handles the parsing of expressions with unary operators (e.g., `!`, `-`, `++`, `--`). It constructs a `Unary` expression object representing the unary expression.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the unary expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `unary()` method is called internally by the `Parser` to process unary expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Bang, Minus, Increment, Decrement, Percent]) {
            let op = self.previous(1);
            let rhs = self.unary()?;
            Ok(Unary {
                id: self.get_id(),
                operator: op,
                right: Box::from(rhs),
            })
        } else {
            self.call()
        }
    }
    /// The `call()` method is responsible for parsing function call expressions.
    ///
    /// It handles the parsing of expressions with function calls. It constructs a `Call` expression object representing the function call expression.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the function call expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `call()` method is called internally by the `Parser` to process function call expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn call(&mut self) -> Result<Expression, String> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }
    /// The `primary()` method is responsible for parsing primary expressions.
    ///
    /// It handles the parsing of primary expressions, which are the most basic expressions like literals, variables, and parenthesized expressions.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the primary expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `primary()` method is called internally by the `Parser` to process primary expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn primary(&mut self) -> Result<Expression, String> {
        let line_number = self.peek().line_number;
        let token = self.peek();
        let result;
        let var_name: Token = Token {
            token_type: Identifier,
            lexeme: self.previous(2).lexeme,
            line_number,
            literal: None,
        };
        match token.token_type {
            Identifier => {
                self.advance();
                let mut expr = Expression::Variable {
                    id: self.get_id(),
                    name: self.previous(1),
                };
                if self.match_token(LeftBracket) {
                    let index = self.expression()?;
                    self.consume(RightBracket, "Expected ']' after index", line_number)?;
                    expr = Expression::Array {
                        id: self.get_id(),
                        elements: vec![Box::new(expr), Box::new(index)],
                    };
                } else if self.match_token(TokenType::Line) {
                    return self.parse_callback(var_name.clone());
                } else if self.match_token(Dot) {
                    let key = self.consume(Identifier, "Expected key after '.'", line_number)?;
                    expr = Expression::ObjectCall {
                        id: self.get_id(),
                        name: token,
                        key,
                    }
                }

                result = expr;
            }
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')' after expression", line_number)?;
                result = Expression::Grouping {
                    id: self.get_id(),
                    expression: Box::new(expr),
                };
            }
            False | True | Null | Number | StringLit => {
                self.advance();
                result = Expression::Literal {
                    id: self.get_id(),
                    value: TronType::from_token(token),
                };
            }
            TokenType::LeftBracket => {
                return self.parse_array();
            }
            TokenType::LeftBrace => return self.parse_object(),
            TokenType::Line => return self.parse_callback(var_name.clone()),

            _ => {
                TronError::throw("E2003", self.current, vec![token.token_type.to_string()]);
                result = Expression::Literal {
                    id: self.get_id() * 23,
                    value: TronType::Null,
                }
            }
        }
        Ok(result)
    }
    /// The `finish_call()` method is responsible for parsing the arguments of a function call.
    ///
    /// It handles the parsing of expressions with function calls and constructs a `Call` expression object representing the function call expression.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the function call expression if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `finish_call()` method is called internally by the `Parser` to process function call expressions within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn finish_call(&mut self, callee: Expression) -> Result<Expression, String> {
        let line_number = self.peek().line_number;
        let mut arguments = vec![];
        if !self.check(RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);
                if arguments.len() >= 32 {
                    TronError::throw("E2004", line_number, vec![]);
                } else if !self.match_token(Comma) {
                    break;
                }
            }
        }
        let paren = self.consume(RightParen, "Expected ')' after arguments.", line_number)?;
        Ok(Call {
            id: self.get_id(),
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }
    /// The `parse_array()` method is responsible for parsing array literals.
    ///
    /// It handles the parsing of expressions with array literals and constructs an `Array` expression object representing the array literal.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the array literal if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `parse_array()` method is called internally by the `Parser` to process array literals within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn parse_array(&mut self) -> Result<Expression, String> {
        let line_number = self.peek().line_number;
        let mut elements = Vec::new();
        let array_id = self.get_id();
        self.advance();
        while !self.check(TokenType::RightBracket) && !self.is_at_end() {
            let element = self.expression()?;
            elements.push(Box::new(element));

            if !self.match_token(TokenType::Comma) {
                break;
            }
        }
        self.consume(
            TokenType::RightBracket,
            "Expect ']' after array elements.",
            line_number,
        )?;

        Ok(Expression::Array {
            id: array_id,
            elements,
        })
    }
    /// The `parse_object()` method is responsible for parsing object literals.
    ///
    /// It handles the parsing of expressions with object literals and constructs an `Object` expression object representing the object literal.
    ///
    /// # Return Value
    ///
    /// A `Result` containing an `Expression` object representing the object literal if parsing is successful, or an error message if parsing fails.
    ///
    /// # Usage
    ///
    /// The `parse_object()` method is called internally by the `Parser` to process object literals within the code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn parse_object(&mut self) -> Result<Expression, String> {
        let line_number = self.peek().line_number;
        let mut properties: Vec<(Token, Expression)> = Vec::new();
        let object_id = self.get_id();
        self.advance();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let key = self.consume(
                Identifier,
                "Expected identifier for object key",
                line_number,
            )?;
            self.consume(Colon, "Expected ':' after object key", line_number)?;
            let value = self.expression()?;
            properties.push((key, value));
            if !self.match_token(Comma) {
                break;
            }
        }
        self.consume(
            TokenType::RightBrace,
            "Expected '}' after object fields",
            line_number,
        )?;
        Ok(Expression::Object {
            id: object_id,
            properties,
        })
    }

    fn parse_callback(&mut self, var_name: Token) -> Result<Expression, String> {
        let line_number = self.peek().line_number;
        self.consume(
            TokenType::Line,
            "Expected '|' to start a callback function",
            line_number,
        )?;

        let mut params = Vec::new();
        while !self.check(TokenType::Line) && !self.is_at_end() {
            let arg_name = self.consume(Identifier, "Expected argument name", line_number)?;
            let arg_type = if self.match_token(Colon) {
                Some(self.consume(Identifier, "Expected type after ':'", line_number)?)
            } else {
                None
            };
            params.push((arg_name, arg_type));

            if !self.match_token(Comma) {
                break;
            }
        }

        self.consume(
            TokenType::Line,
            "Expected '|' to end the argument list",
            line_number,
        )?;

        self.consume(
            LeftBrace,
            "Expected '{' to start the callback body",
            line_number,
        )?;

        let mut body = Vec::new();
        while !self.check(RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            body.push(Box::new(stmt));
        }

        self.consume(
            RightBrace,
            "Expected '}' to end the callback body",
            line_number,
        )?;
        Ok(Expression::Callback {
            id: self.get_id(),
            params,
            body,
            var_name,
        })
    }
    /// The `consume()` method is used to consume a token of a specific type from the token stream.
    ///
    /// It checks if the next token in the stream matches the expected type and advances the token stream if it does. If the token does not match, it returns an error.
    ///
    /// # Parameters
    ///
    /// - `token_type`: The type of token expected.
    /// - `msg`: The error message to return if the token does not match the expected type.
    ///
    /// # Return Value
    ///
    /// A `Result` containing the consumed `Token` if it matches the expected type, or an error message if it does not.
    ///
    /// # Usage
    ///
    /// The `consume()` method is called internally by the `Parser` to ensure that the token stream matches the expected syntax.
    ///
    /// ### Last Updated: (v3.0.0)
    fn consume(&mut self, token_type: TokenType, msg: &str, line: usize) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous(1);
            return Ok(token);
        }
        TronError::throw("E2003", line, vec![msg.to_string()]);
        Ok(token)
    }
    /// The `check()` method is used to check if the current token is of a specific type without consuming it.
    ///
    /// # Parameters
    ///
    /// - `token_type`: The type of token to check against.
    ///
    /// # Return Value
    ///
    /// A boolean indicating whether the current token matches the expected type.
    ///
    /// # Usage
    ///
    /// The `check()` method is called internally by the `Parser` to look ahead at the next token without advancing the token stream.
    ///
    /// ### Last Updated: (v3.0.0)
    fn check(&mut self, typ: TokenType) -> bool {
        self.peek().token_type == typ
    }
    /// The `match_token()` method is used to check if the current token matches a specific type and consumes it if it does.
    ///
    /// # Parameters
    ///
    /// - `token_type`: The type of token to match against.
    ///
    /// # Return Value
    ///
    /// A boolean indicating whether the current token was matched and consumed.
    ///
    /// # Usage
    ///
    /// The `match_token()` method is called internally by the `Parser` to match and consume a token of a specific type.
    ///
    /// ### Last Updated: (v3.0.0)
    fn match_token(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek().token_type == typ {
            self.advance();
            true
        } else {
            false
        }
    }
    /// The `match_tokens()` method is used to check if the current token matches any of the specified types and consumes it if it does.
    ///
    /// # Parameters
    ///
    /// - `token_types`: A slice of token types to match against.
    ///
    /// # Return Value
    ///
    /// A boolean indicating whether the current token was matched and consumed.
    ///
    /// # Usage
    ///
    /// The `match_tokens()` method is called internally by the `Parser` to match and consume a token of any of the specified types.
    ///
    /// ### Last Updated: (v3.0.0)
    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        for typ in typs {
            if self.match_token(*typ) {
                return true;
            }
        }
        false
    }
    /// The `advance()` method is used to consume the current token and move to the next one in the token stream.
    ///
    /// # Usage
    ///
    /// The `advance()` method is called internally by the `Parser` to move forward in the token stream.
    ///
    /// ### Last Updated: (v3.0.0)
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous(1)
    }
    /// The `peek()` method is used to look at the current token without consuming it.
    ///
    /// # Return Value
    ///
    /// A reference to the current token.
    ///
    /// # Usage
    ///
    /// The `peek()` method is called internally by the `Parser` to inspect the next token without advancing the token stream.
    ///
    /// ### Last Updated: (v3.0.0)
    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }
    /// The `previous()` method is used to look at the previously consumed token.
    ///
    /// # Return Value
    ///
    /// A reference to the previously consumed token.
    ///
    /// # Usage
    ///
    /// The `previous()` method is called internally by the `Parser` to inspect the last token that was consumed.
    ///
    /// ### Last Updated: (v3.0.0)

    fn previous(&mut self, steps_back: usize) -> Token {
        if self.current < steps_back {
            Token {
                token_type: Eof,
                lexeme: String::new(),
                line_number: 0,
                literal: None,
            }
        } else {
            self.tokens[self.current - steps_back].clone()
        }
    }
    /// The `is_at_end()` method is used to check if the parser has reached the end of the token stream.
    ///
    /// # Return Value
    ///
    /// A boolean indicating whether the parser is at the end of the token stream.
    ///
    /// # Usage
    ///
    /// The `is_at_end()` method is called internally by the `Parser` to determine if there are more tokens to process.
    ///
    /// ### Last Updated: (v3.0.0)

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == Eof
    }
}
