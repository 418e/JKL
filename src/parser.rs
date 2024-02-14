/*

    Tron Parser

    - Statement parsing happens here, if you are looking for expressions visit expr.rs

*/
use crate::expr::{Expr, Expr::*, LiteralValue};
use crate::panic;
use crate::scanner::{Token, TokenType, TokenType::*};
use crate::stmt::BeforeBlock;
use crate::stmt::Stmt;
use std::process::exit;
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    next_id: usize,
}
#[derive(Debug)]
enum FunctionKind {
    Function,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            next_id: 0,
        }
    }
    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    panic(&format!("Parser Error: {}", msg.to_string()));
                }
            }
        }
        Ok(stmts)
    }
    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(Var) {
            self.var_declaration()
        } else if self.match_token(Fun) {
            self.function(FunctionKind::Function)
        } else {
            self.statement()
        }
    }
    fn function(&mut self, kind: FunctionKind) -> Result<Stmt, String> {
        let name = self.consume(Identifier, &format!("expected {kind:?} name"))?;
        self.consume(LeftParen, &format!("expected '(' after {kind:?} name"))?;

        let mut parameters: Vec<(Token, Option<Token>)> = vec![];
        if !self.check(RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let location = self.peek().line_number;
                    panic(
                        &format!(
                            "Parser Error at line {location}: can't have more than 255 arguments"
                        )
                        .to_string(),
                    );
                }
                let param_name = self.consume(Identifier, "expected parameter name")?;
                let param_type = if self.match_token(Colon) {
                    Some(self.consume(Identifier, "expected type after ':'")?)
                } else {
                    None
                };
                parameters.push((param_name, param_type));
                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(RightParen, "expected ')' after parameters.")?;
        if self.match_token(Equal) {
            let body_expr = self.expression()?;
            self.consume(Semicolon, "expected ';' after function body expression.")?;
            return Ok(Stmt::Function {
                name,
                params: parameters,
                body: vec![Box::new(Stmt::ReturnStmt {
                    keyword: Token {
                        token_type: TokenType::Return,
                        lexeme: "".to_string(),
                        line_number: 0,
                        literal: None,
                    },
                    value: Some(body_expr),
                })],
            });
        }
        self.consume(Start, &format!("expected 'start' before {kind:?} body."))?;
        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => {
                panic("\n Parser Error: block statement parsed something that was not a block");
                exit(1)
            }
        };
        Ok(Stmt::Function {
            name,
            params: parameters,
            body,
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let mut names: Vec<Token> = vec![];
        let mut type_annotation: Vec<Option<Token>> = vec![];
        loop {
            let variable_name = self.consume(Identifier, "Expected variable name")?;
            names.push(variable_name);
            let variable_type = if self.match_token(Colon) {
                Some(self.consume(Identifier, "Expected type after ':'")?)
            } else {
                None
            };
            type_annotation.push(variable_type);
            if !self.match_token(Comma) {
                break;
            }
        }
        self.consume(Equal, "Expected '=' after variable name")?;
        let initializer = self.expression()?;
        self.consume(Semicolon, "Expected ';' after variable declaration")?;
        Ok(Stmt::Var {
            names,
            type_annotation,
            initializer,
        })
    }
    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(Start) {
            self.block_statement()
        } else if self.match_token(Print) {
            self.print_statement()
        } else if self.match_token(Errors) {
            self.error_statement()
        } else if self.match_token(Exits) {
            self.exits_statement()
        } else if self.match_token(Import) {
            self.import_statement()
        } else if self.match_token(If) {
            self.if_statement()
        } else if self.match_token(While) {
            self.while_statement()
        } else if self.match_token(Wait) {
            self.wait_statement()
        } else if self.match_token(Bench) {
            self.bench_statement()
        } else if self.match_token(For) {
            self.for_statement()
        } else if self.match_token(Return) {
            self.return_statement()
        } else if self.match_token(Break) {
            self.break_statement()
        } else {
            self.expression_statement()
        }
    }
    fn return_statement(&mut self) -> Result<Stmt, String> {
        let keyword = self.previous();
        let value;
        if !self.check(Semicolon) {
            value = Some(self.expression()?);
        } else {
            value = None;
        }
        self.consume(Semicolon, "Expected ';' after return value;")?;
        Ok(Stmt::ReturnStmt { keyword, value })
    }
    fn break_statement(&mut self) -> Result<Stmt, String> {
        let keyword = self.previous();
        self.consume(Semicolon, "Expected Semicolon after return value")?;
        Ok(Stmt::BreakStmt { keyword })
    }
    fn for_statement(&mut self) -> Result<Stmt, String> {
        let initializer;
        if self.match_token(Semicolon) {
            initializer = None;
        } else if self.match_token(Var) {
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
        self.consume(Semicolon, "Expected ';' after loop condition.")?;
        let increment;
        if !self.check(RightParen) {
            let expr = self.expression()?;
            increment = Some(expr);
        } else {
            increment = None;
        }
        let mut body = self.statement()?;
        if let Some(incr) = increment {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(Stmt::Expression { expression: incr }),
                ],
            };
        }
        let cond;
        match condition {
            None => {
                cond = Expr::Literal {
                    id: self.get_id(),
                    value: LiteralValue::True,
                }
            }
            Some(c) => cond = c,
        }
        body = Stmt::WhileStmt {
            conditions: vec![cond],
            body: Box::new(body),
        };
        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)],
            };
        }
        Ok(body)
    }
    fn while_statement(&mut self) -> Result<Stmt, String> {
        let mut conditions = Vec::new();
        loop {
            let condition = self.expression()?;
            conditions.push(condition);
            if !self.match_token(Comma) {
                break;
            }
        }
        let body = self.statement()?;
        Ok(Stmt::WhileStmt {
            conditions,
            body: Box::new(body),
        })
    }
    fn wait_statement(&mut self) -> Result<Stmt, String> {
        let time = self.expression()?;
        let body = self.statement()?;
        let before = if self.match_token(Before) {
            let before_time = self.expression()?;
            let before_body = self.statement()?;
            Some(BeforeBlock {
                time: before_time,
                body: Box::new(before_body),
            })
        } else {
            None
        };
        Ok(Stmt::WaitStmt {
            time,
            body: Box::new(body),
            before,
        })
    }
    fn bench_statement(&mut self) -> Result<Stmt, String> {
        let body = self.statement()?;
        Ok(Stmt::BenchStmt {
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        let mut predicates = Vec::new();
        loop {
            let predicate = self.expression()?;
            predicates.push(predicate);
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
        Ok(Stmt::IfStmt {
            predicates,
            then_branch,
            elif_branches,
            else_branch,
        })
    }
    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = vec![];
        while !self.check(End) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(Box::new(decl));
        }
        self.consume(End, "Expected 'end' after a block")?;
        Ok(Stmt::Block { statements })
    }
    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }
    fn error_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Errors { expression: value })
    }
    fn exits_statement(&mut self) -> Result<Stmt, String> {
        self.consume(Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Exits {})
    }
    fn import_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Import { expression: value })
    }
    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }
    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }
    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.pipe()?;
        if self.match_token(Equal) {
            let value = self.expression()?;
            match expr {
                Variable { id: _, name } => Ok(Assign {
                    id: self.get_id(),
                    name,
                    value: Box::from(value),
                }),
                Get {
                    id: _,
                    object,
                    name,
                } => Ok(Set {
                    id: self.get_id(),
                    object,
                    name,
                    value: Box::new(value),
                }),
                _ => {
                    panic("Invalid assignment target");
                    exit(1);
                }
            }
        } else {
            Ok(expr)
        }
    }
    fn pipe(&mut self) -> Result<Expr, String> {
        let mut expr = self.or()?;
        while self.match_token(Pipe) {
            let pipe = self.previous();
            let function = self.or()?;
            expr = Call {
                id: self.get_id(),
                callee: Box::new(function),
                paren: pipe,
                arguments: vec![expr],
            };
        }
        Ok(expr)
    }
    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.nor()?;
        while self.match_token(Or) {
            let operator = self.previous();
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
    fn nor(&mut self) -> Result<Expr, String> {
        let mut expr = self.xor()?;
        while self.match_token(Nor) {
            let operator = self.previous();
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
    fn xor(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;
        while self.match_token(Xor) {
            let operator = self.previous();
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
    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        while self.match_token(And) {
            let operator = self.previous();
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
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
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
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_tokens(&[Minus, Plus, PlusEqual, MinusEqual, Random]) {
            let op = self.previous();
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
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[Slash, Star, Power, Cube, Root, CubicRoot]) {
            let op = self.previous();
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

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[Bang, Minus, Increment, Decrement, Percent, Power2, Root2]) {
            let op = self.previous();
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
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        if self.match_token(Dot) {
            let name = self.consume(Identifier, "Expected token after dot-accessor")?;
            expr = Get {
                id: self.get_id(),
                object: Box::new(expr),
                name,
            };
        }

        loop {
            if self.match_token(LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(Dot) {
                let name = self.consume(Identifier, "Expected token after dot-accessor")?;
                expr = Get {
                    id: self.get_id(),
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = vec![];
        if !self.check(RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);
                if arguments.len() >= 255 {
                    let location = self.peek().line_number;
                    panic(
                        &format!("Line {location}: Cant have more than 255 arguments").to_string(),
                    )
                } else if !self.match_token(Comma) {
                    break;
                }
            }
        }
        let paren = self.consume(RightParen, "Expected ')' after arguments.")?;
        Ok(Call {
            id: self.get_id(),
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }
    fn parse_array(&mut self) -> Result<Expr, String> {
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
        self.consume(TokenType::RightBracket, "Expect ']' after array elements.")?;

        Ok(Expr::Array {
            id: array_id,
            elements,
        })
    }
    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();
        let result;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')' after expression")?;
                result = Expr::Grouping {
                    id: self.get_id(),
                    expression: Box::new(expr),
                };
            }
            False | True | Nil | Number | Integer | StringLit => {
                self.advance();
                result = Expr::Literal {
                    id: self.get_id(),
                    value: LiteralValue::from_token(token),
                };
            }
            TokenType::LeftBracket => {
                return self.parse_array();
            }
            Identifier => {
                self.advance();
                let mut expr = Expr::Variable {
                    id: self.get_id(),
                    name: self.previous(),
                };
                if self.match_token(LeftBracket) {
                    let index = self.expression()?;
                    self.consume(RightBracket, "Expected ']' after index")?;
                    expr = Expr::Array {
                        id: self.get_id(),
                        elements: vec![Box::new(expr), Box::new(index)],
                    };
                }
                result = expr;
            }
            _ => {
                panic(&format!("Unexpected token: {:?}", token.token_type));
                exit(1);
            }
        }
        Ok(result)
    }
    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            panic(&format!(
                "\nParser Error at line {}: {}",
                token.line_number, msg
            ));
            exit(1)
        }
    }
    fn check(&mut self, typ: TokenType) -> bool {
        self.peek().token_type == typ
    }
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
    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        for typ in typs {
            if self.match_token(*typ) {
                return true;
            }
        }
        false
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }
    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == Eof
    }
}
