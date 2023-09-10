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
impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        use Stmt::*;
        match self {
            Expression { expression } => expression.to_string(),
            Print { expression } => format!("(print {})", expression.to_string()),
            Input { expression } => format!("(input {})", expression.to_string()),
            Errors { expression } => format!("(error {})", expression.to_string()),
            Import { expression } => format!("(run {})", expression.to_string()),
            Exits {} => format!("(exit)"),
            Var {
                name,
                initializer: _,
            } => format!("(let {})", name.lexeme),
            Block { statements } => format!(
                "(block {})",
                statements
                    .into_iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<String>()
            ),
            IfStmt {
                predicate: _,
                then: _,
                els: _,
            } => todo!(),
            WhileStmt {
                condition: _,
                body: _,
            } => todo!(),
            Function {
                name: _,
                params: _,
                body: _,
            } => todo!(),
            CmdFunction { name: _, cmd: _ } => todo!(),
            ReturnStmt {
                keyword: _,
                value: _,
            } => todo!(),
        }
    }
}
