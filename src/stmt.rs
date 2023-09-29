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
    Input {
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
        name: Token,
        initializer: Expr,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
    IfStmt {
        predicate: Expr,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    },
    BenchStmt {
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    CmdFunction {
        name: Token,
        cmd: String,
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Expr>,
    },
}
