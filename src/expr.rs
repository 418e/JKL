use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::scanner;
use crate::scanner::{Token, TokenType};
use colored::Colorize;
use num::integer::Roots;
use rand::Rng;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::io;
use std::rc::Rc;
#[derive(Clone)]
pub enum CallableImpl {
    JekoFunction(JekoFunctionImpl),
    NativeFunction(NativeFunctionImpl),
}
#[derive(Clone)]
pub struct JekoFunctionImpl {
    pub name: String,
    pub arity: usize,
    pub parent_env: Environment,
    pub params: Vec<Token>,
    pub body: Vec<Box<Stmt>>,
}
#[derive(Clone)]
pub struct NativeFunctionImpl {
    pub name: String,
    pub arity: usize,
    pub fun: Rc<dyn Fn(&Vec<LiteralValue>) -> LiteralValue>,
}
#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    StringValue(String),
    True,
    False,
    Nil,
    Callable(CallableImpl),
}
use LiteralValue::*;
impl std::fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number(x), Number(y)) => x == y,
            (
                Callable(CallableImpl::JekoFunction(JekoFunctionImpl { name, arity, .. })),
                Callable(CallableImpl::JekoFunction(JekoFunctionImpl {
                    name: name2,
                    arity: arity2,
                    ..
                })),
            ) => name == name2 && arity == arity2,
            (
                Callable(CallableImpl::NativeFunction(NativeFunctionImpl { name, arity, .. })),
                Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                    name: name2,
                    arity: arity2,
                    ..
                })),
            ) => name == name2 && arity == arity2,
            (StringValue(x), StringValue(y)) => x == y,
            (True, True) => true,
            (False, False) => true,
            (Nil, Nil) => true,
            _ => false,
        }
    }
}
fn unwrap_as_f64(literal: Option<scanner::LiteralValue>) -> f64 {
    match literal {
        Some(scanner::LiteralValue::FValue(x)) => x as f64,
        _ => panic!("Could not unwrap as f64"),
    }
}
fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as string"),
    }
}
impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => format!("\"{}\"", x),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Callable(CallableImpl::JekoFunction(JekoFunctionImpl {
                name,
                arity,
                ..
            })) => format!("{name}/{arity}"),
            LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
                name,
                arity,
                ..
            })) => format!("{name}/{arity}"),
        }
    }
    pub fn to_type(&self) -> &str {
        match self {
            LiteralValue::Number(_) => "Number",
            LiteralValue::StringValue(_) => "String",
            LiteralValue::True => "Boolean",
            LiteralValue::False => "Boolean",
            LiteralValue::Nil => "nil",
            LiteralValue::Callable(_) => "Callable",
        }
    }
    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f64(token.literal)),
            TokenType::StringLit => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token),
        }
    }
    pub fn from_bool(b: bool) -> Self {
        if b {
            True
        } else {
            False
        }
    }
    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            Number(x) => {
                if *x == 0.0 as f64 {
                    True
                } else {
                    False
                }
            }
            StringValue(s) => {
                if s.len() == 0 {
                    True
                } else {
                    False
                }
            }
            True => False,
            False => True,
            Nil => True,
            Callable(_) => panic!("Cannot use Callable as a falsy value"),
        }
    }
    pub fn is_truthy(&self) -> LiteralValue {
        match self {
            Number(x) => {
                if *x == 0.0 as f64 {
                    False
                } else {
                    True
                }
            }
            StringValue(s) => {
                if s.len() == 0 {
                    False
                } else {
                    True
                }
            }
            True => True,
            False => False,
            Nil => False,
            Callable(_) => panic!("Cannot use Callable as a truthy value"),
        }
    }
}
use crate::stmt::Stmt;
#[derive(Clone)]
pub enum Expr {
    AnonFunction {
        id: usize,
        paren: Token,
        arguments: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    Assign {
        id: usize,
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        id: usize,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        id: usize,
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        id: usize,
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        id: usize,
        expression: Box<Expr>,
    },
    Literal {
        id: usize,
        value: LiteralValue,
    },
    Logical {
        id: usize,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        id: usize,
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Unary {
        id: usize,
        operator: Token,
        right: Box<Expr>,
    },
    SUnary {
        id: usize,
        operator: Token,
        left: Box<Expr>,
    },
    Variable {
        id: usize,
        name: Token,
    },
}
impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.get_id(), self.to_string())
    }
}
impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state)
    }
}
impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        let ptr = std::ptr::addr_of!(self);
        let ptr2 = std::ptr::addr_of!(other);
        ptr == ptr2
    }
}
impl Eq for Expr {}
impl Expr {
    pub fn get_id(&self) -> usize {
        match self {
            Expr::AnonFunction {
                id,
                paren: _,
                arguments: _,
                body: _,
            } => *id,
            Expr::Assign {
                id,
                name: _,
                value: _,
            } => *id,
            Expr::Binary {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expr::Call {
                id,
                callee: _,
                paren: _,
                arguments: _,
            } => *id,
            Expr::Get {
                id,
                object: _,
                name: _,
            } => *id,
            Expr::Grouping { id, expression: _ } => *id,
            Expr::Literal { id, value: _ } => *id,
            Expr::Logical {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expr::Set {
                id,
                object: _,
                name: _,
                value: _,
            } => *id,
            Expr::Unary {
                id,
                operator: _,
                right: _,
            } => *id,
            Expr::SUnary {
                id,
                operator: _,
                left: _,
            } => *id,
            Expr::Variable { id, name: _ } => *id,
        }
    }
}
impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::AnonFunction {
                id: _,
                paren: _,
                arguments,
                body: _,
            } => format!("anon/{}", arguments.len()),
            Expr::Assign { id: _, name, value } => format!("({name:?} = {}", value.to_string()),
            Expr::Binary {
                id: _,
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => format!("({} {:?})", (*callee).to_string(), arguments),
            Expr::Get {
                id: _,
                object,
                name,
            } => format!("(get {} {})", object.to_string(), name.lexeme),
            Expr::Grouping { id: _, expression } => {
                format!("(group {})", (*expression).to_string())
            }
            Expr::Literal { id: _, value } => format!("{}", value.to_string()),
            Expr::Logical {
                id: _,
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.to_string(),
                left.to_string(),
                right.to_string()
            ),
            Expr::Set {
                id: _,
                object,
                name,
                value,
            } => format!(
                "(set {} {} {})",
                object.to_string(),
                name.to_string(),
                value.to_string()
            ),
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({}({}))", operator_str, right_str)
            }
            Expr::SUnary {
                id: _,
                operator,
                left,
            } => {
                let operator_str = operator.lexeme.clone();
                let left_str = (*left).to_string();
                format!("({}({}))", operator_str, left_str)
            }
            Expr::Variable { id: _, name } => format!("(let {})", name.lexeme),
        }
    }
    pub fn evaluate(&self, environment: Environment) -> Result<LiteralValue, String> {
        match self {
            Expr::AnonFunction {
                id: _,
                paren: _,
                arguments,
                body,
            } => {
                let arity = arguments.len();
                let arguments: Vec<Token> = arguments.iter().map(|t| (*t).clone()).collect();
                let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                let callable_impl = CallableImpl::JekoFunction(JekoFunctionImpl {
                    name: "anon_funciton".to_string(),
                    arity,
                    parent_env: environment.clone(),
                    params: arguments,
                    body,
                });
                Ok(Callable(callable_impl))
            }
            Expr::Assign { id: _, name, value } => {
                let new_value = (*value).evaluate(environment.clone())?;
                let assign_success =
                    environment.assign(&name.lexeme, new_value.clone(), self.get_id());
                if assign_success {
                    Ok(new_value)
                } else {
                    Err(
                        format!("Error 100: Variable {} has not been declared", name.lexeme)
                            .red()
                            .to_string(),
                    )
                }
            }
            Expr::Variable { id: _, name } => match environment.get(&name.lexeme, self.get_id()) {
                Some(value) => Ok(value.clone()),
                None => Err(format!(
                    "Error 100: Variable '{}' has not been declared at distance {:?}",
                    name.lexeme,
                    environment.get_distance(self.get_id())
                )
                .red()
                .to_string()),
            },
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                let callable: LiteralValue = (*callee).evaluate(environment.clone())?;
                match callable {
                    Callable(CallableImpl::JekoFunction(jekofun)) => {
                        run_jeko_function(jekofun, arguments, environment)
                    }
                    Callable(CallableImpl::NativeFunction(nativefun)) => {
                        let mut evaluated_arguments = vec![];
                        for argument in arguments {
                            evaluated_arguments.push(argument.evaluate(environment.clone())?);
                        }
                        Ok((nativefun.fun)(&evaluated_arguments))
                    }
                    other => Err(format!("Error 102: {} is not callable", other.to_type())
                        .red()
                        .to_string()),
                }
            }
            Expr::Literal { id: _, value } => Ok((*value).clone()),
            Expr::Logical {
                id: _,
                left,
                operator,
                right,
            } => match operator.token_type {
                TokenType::Or => {
                    let lhs_value = left.evaluate(environment.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == True {
                        Ok(lhs_value)
                    } else {
                        right.evaluate(environment.clone())
                    }
                }
                TokenType::And => {
                    let lhs_value = left.evaluate(environment.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == False {
                        Ok(lhs_true)
                    } else {
                        right.evaluate(environment.clone())
                    }
                }
                ttype => Err(
                    format!("Error 103: Invalid token in logical expression: {}", ttype)
                        .red()
                        .to_string(),
                ),
            },
            Expr::Get {
                id: _,
                object,
                name,
            } => {
                let obj_value = object.evaluate(environment.clone())?;
                Err(format!(
                    "Error 105: Cannot access property on type {} / {:?}",
                    obj_value.to_type(),
                    name
                )
                .red()
                .to_string())
            }
            Expr::Set {
                id: _,
                object,
                name,
                value,
            } => {
                let obj_value = object.evaluate(environment.clone())?;
                Err(format!(
                    "Error 105: Cannot set property on type {} /  {:?} / {:?}",
                    obj_value.to_type(),
                    name,
                    value
                )
                .red()
                .to_string())
            }
            Expr::Grouping { id: _, expression } => expression.evaluate(environment),
            Expr::SUnary {
                id: _,
                operator,
                left,
            } => {
                let left = left.evaluate(environment)?;
                match (operator.token_type, &left) {
                    (TokenType::Sin, Number(x)) => Ok(Number(x.sin())),
                    (TokenType::Sin, _) => Err(format!(
                        "Error 107: Sin not implemented for {}",
                        left.to_type()
                    )
                    .red()
                    .to_string()),
                    (ttype, _) => Err(format!(
                        "Error 107: {} is not a valid unary operator",
                        ttype
                    )
                    .red()
                    .to_string()),
                }
            }
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let right = right.evaluate(environment)?;
                let mut rng = rand::thread_rng();
                match (&right, operator.token_type) {
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (Number(x), TokenType::Increment) => Ok(Number(x + 1.0)),
                    (Number(x), TokenType::Decrement) => Ok(Number(x - 1.0)),
                    (Number(x), TokenType::Power) => Ok(Number(x * x)),
                    (Number(x), TokenType::Cube) => Ok(Number(x * x * x)),
                    (Number(x), TokenType::Root) => Ok(Number(Roots::sqrt(&(*x as i64)) as f64)),
                    (Number(x), TokenType::CubicRoot) => {
                        Ok(Number(Roots::cbrt(&(*x as i64)) as f64))
                    }
                    (Number(x), TokenType::Random) => {
                        Ok(Number(rng.gen_range(0..*x as i64) as f64))
                    }
                    (StringValue(x), TokenType::In) => {
                        println!("{}", x.to_string());
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        Ok(StringValue(format!("{}", input)))
                    }
                    (StringValue(x), TokenType::Num) => {
                        println!("{}", x.to_string());
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        let inputs: f64 = input.trim().parse().expect("Expected a number");
                        Ok(LiteralValue::Number(inputs))
                    }
                    (StringValue(x), TokenType::Parse) => Ok(LiteralValue::Number(
                        x.trim().parse().expect("Expected a number"),
                    )),
                    (Number(x), TokenType::Sin) => Ok(Number(x.sin())),
                    (Number(x), TokenType::ASin) => Ok(Number(x.asin())),
                    (Number(x), TokenType::Cos) => Ok(Number(x.cos())),
                    (Number(x), TokenType::ACos) => Ok(Number(x.acos())),
                    (Number(x), TokenType::Tan) => Ok(Number(x.tan())),
                    (Number(x), TokenType::ATan) => Ok(Number(x.atan())),
                    (Number(x), TokenType::Round) => Ok(Number(x.round())),
                    (Number(x), TokenType::Floor) => Ok(Number(x.floor())),
                    (Number(x), TokenType::Percent) => Ok(Number(*x / 100 as f64)),
                    (Number(x), TokenType::ToDeg) => Ok(Number(x.to_degrees())),
                    (Number(x), TokenType::ToRad) => Ok(Number(x.to_radians())),
                    (_, TokenType::Minus) => Err(format!(
                        "Error 107: Minus not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Increment) => Err(format!(
                        "Error 107: Increment not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Decrement) => Err(format!(
                        "Error 107: Minus not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Power) => Err(format!(
                        "Error 107: Power not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Cube) => Err(format!(
                        "Error 107: Cube not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Root) => Err(format!(
                        "Error 107: Root not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::CubicRoot) => Err(format!(
                        "Error 107: CubicRoot not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::In) => Err(format!(
                        "Error 107: In not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Num) => Err(format!(
                        "Error 107: Num not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Parse) => Err(format!(
                        "Error 107: Parse not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Sin) => Err(format!(
                        "Error 107: Sin not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Cos) => Err(format!(
                        "Error 107: Cos not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Tan) => Err(format!(
                        "Error 107: Tan not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Percent) => Err(format!(
                        "Error 107: Percent not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Round) => Err(format!(
                        "Error 107: Round not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (_, TokenType::Floor) => Err(format!(
                        "Error 107: Floor not implemented for {}",
                        right.to_type()
                    )
                    .red()
                    .to_string()),
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, ttype) => Err(format!(
                        "Error 107: {} is not a valid unary operator",
                        ttype
                    )
                    .red()
                    .to_string()),
                }
            }
            Expr::Binary {
                id: _,
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment.clone())?;
                let right = right.evaluate(environment.clone())?;
                let mut rng = rand::thread_rng();
                match (&left, operator.token_type, &right) {
                    (Number(x), TokenType::Random, Number(y)) => Ok(Number(rng.gen_range(*x..*y))),
                    (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),
                    (StringValue(x), TokenType::Plus, Number(y)) => {
                        Ok(StringValue(format!("{}{}", x, y.to_string())))
                    }
                    (Number(x), TokenType::Plus, StringValue(y)) => {
                        Ok(StringValue(format!("{}{}", x.to_string(), y)))
                    }
                    (Number(x), TokenType::PlusEqual, Number(y)) => Ok(Number(x + y)),
                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (Number(x), TokenType::MinusEqual, Number(y)) => Ok(Number(x - y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),
                    (Number(x), TokenType::Greater, Number(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (Number(x), TokenType::GreaterEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                    (Number(x), TokenType::Less, Number(y)) => Ok(LiteralValue::from_bool(x < y)),
                    (Number(x), TokenType::LessEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (StringValue(_), op, Number(_)) => Err(format!(
                        "Error 107: {} is not defined for string and number",
                        op
                    )
                    .red()
                    .to_string()),
                    (Number(_), op, StringValue(_)) => Err(format!(
                        "Error 107: {} is not defined for string and number",
                        op
                    )
                    .red()
                    .to_string()),
                    (StringValue(s1), TokenType::Plus, StringValue(s2)) => {
                        Ok(StringValue(format!("{}{}", s1, s2)))
                    }
                    (x, TokenType::BangEqual, y) => Ok(LiteralValue::from_bool(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(LiteralValue::from_bool(x == y)),
                    (StringValue(s1), TokenType::Greater, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1.len() > s2.len()))
                    }
                    (StringValue(s1), TokenType::GreaterEqual, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1.len() >= s2.len()))
                    }
                    (StringValue(s1), TokenType::Less, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1.len() < s2.len()))
                    }
                    (StringValue(s1), TokenType::LessEqual, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1.len() <= s2.len()))
                    }
                    (x, ttype, y) => Err(format!(
                        "Error 107: {} is not implemented for operands {:?} and {:?}",
                        ttype, x, y
                    )
                    .red()
                    .to_string()),
                }
            }
        }
    }
}
pub fn run_jeko_function(
    jekofun: JekoFunctionImpl,
    arguments: &Vec<Expr>,
    eval_env: Environment,
) -> Result<LiteralValue, String> {
    if arguments.len() != jekofun.arity {
        return Err(format!(
            "Error 108: Callable {} expected {} arguments but got {}",
            jekofun.name,
            jekofun.arity,
            arguments.len()
        )
        .red()
        .to_string());
    }
    let mut arg_vals = vec![];
    for arg in arguments {
        let val = arg.evaluate(eval_env.clone())?;
        arg_vals.push(val);
    }
    let fun_env = jekofun.parent_env.enclose();
    for (i, val) in arg_vals.iter().enumerate() {
        fun_env.define(jekofun.params[i].lexeme.clone(), (*val).clone());
    }
    let mut int = Interpreter::with_env(fun_env);
    for i in 0..(jekofun.body.len()) {
        let result = int.interpret(vec![&jekofun.body[i]]);
        if let Err(e) = result {
            return Err(e.red().to_string());
        } else if let Some(value) = int.specials.get("return") {
            return Ok(value.clone());
        }
    }
    Ok(LiteralValue::Nil)
}
