/*

    Tron Statements

    - This file contains statement enums

*/
use crate::expr::Expr;
use crate::scanner::Token;
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Errors {
        expression: Expr,
    },
    Exits {},
    Import {
        expression: Expr,
    },
    Var {
        names: Vec<Token>,
        type_annotation: Vec<Option<Token>>,
        initializer: Expr,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
    WhileStmt {
        conditions: Vec<Expr>,
        body: Box<Stmt>,
    },
    IfStmt {
        predicates: Vec<Expr>,
        then_branch: Box<Stmt>,
        elif_branches: Vec<(Vec<Expr>, Box<Stmt>)>,
        else_branch: Option<Box<Stmt>>,
    },
    WaitStmt {
        time: Expr,
        body: Box<Stmt>,
        before: Option<BeforeBlock>,
    },
    BenchStmt {
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<(Token, Option<Token>)>,
        body: Vec<Box<Stmt>>,
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Expr>,
    },
    BreakStmt {
        keyword: Token,
    },
}
#[derive(Debug, Clone)]
pub struct BeforeBlock {
    pub time: Expr,
    pub body: Box<Stmt>,
}
