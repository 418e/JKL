use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::scanner;
use crate::scanner::{Token, TokenType};
use colored::Colorize;
use num::integer::Roots;
use rand::Rng;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone)]
pub enum CallableImpl {
    TronFunction(TronFunctionImpl),
    NativeFunction(NativeFunctionImpl),
}
#[derive(Clone)]
pub struct TronFunctionImpl {
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
    ArrayValue(Vec<LiteralValue>),
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
                Callable(CallableImpl::TronFunction(TronFunctionImpl { name, arity, .. })),
                Callable(CallableImpl::TronFunction(TronFunctionImpl {
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
            LiteralValue::ArrayValue(x) => format!("\"{:?}\"", x),
            LiteralValue::StringValue(x) => format!("\"{}\"", x),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Callable(CallableImpl::TronFunction(TronFunctionImpl {
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
            LiteralValue::ArrayValue(_) => "Array",
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
            ArrayValue(x) => {
                if x.len() == 0 {
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
            ArrayValue(x) => {
                if x.len() == 0 {
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
    Array {
        id: usize,
        elements: Vec<Box<Expr>>,
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
            Expr::Array { id, elements: _ } => *id,
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
            Expr::Array { id: _, elements } => {
                let elements_str = elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", elements_str)
            }
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
                let callable_impl = CallableImpl::TronFunction(TronFunctionImpl {
                    name: "anon_funciton".to_string(),
                    arity,
                    parent_env: environment.clone(),
                    params: arguments,
                    body,
                });
                Ok(Callable(callable_impl))
            }
            Expr::Array { id: _, elements } => {
                if elements.len() == 2 {
                    // This is an array indexing operation
                    let array = elements[0].evaluate(environment.clone())?;
                    let index = elements[1].evaluate(environment.clone())?;
                    if let LiteralValue::Number(index_num) = index {
                        if let LiteralValue::ArrayValue(arr) = array {
                            let idx = index_num as usize;
                            return arr.get(idx).cloned().ok_or_else(|| {
                                format!("Error 108: Array index out of bounds: {}", idx)
                            });
                        }
                    }
                    Err("Error 109: Invalid array indexing operation".to_string())
                } else {
                    let mut array_elements = Vec::new();
                    for element_expr in elements.iter() {
                        let evaluated = element_expr.evaluate(environment.clone())?;
                        array_elements.push(evaluated);
                    }
                    Ok(LiteralValue::ArrayValue(array_elements))
                }
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
                    Callable(CallableImpl::TronFunction(tronfun)) => {
                        run_tron_function(tronfun, arguments, environment)
                    }
                    LiteralValue::Number(num) => {
                        if let Expr::Literal {
                            value: LiteralValue::StringValue(method_name),
                            ..
                        } = &arguments[0]
                        {
                            match method_name.as_str() {
                                "sin" => Ok(LiteralValue::Number(num.sin())),
                                // Add other internal functions here
                                _ => Err("Error: Unknown method".to_string()),
                            }
                        } else {
                            Err("Error: First argument of method call must be a method name"
                                .to_string())
                        }
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
                TokenType::Xor => {
                    let lhs_value = left.evaluate(environment.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == True {
                        Ok(False)
                    } else {
                        Ok(True)
                    }
                }
                TokenType::Nor => {
                    let lhs_value = left.evaluate(environment.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == False {
                        Ok(True)
                    } else {
                        Ok(False)
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
                let object_evaluated = object.evaluate(environment)?;
                match object_evaluated {
                    LiteralValue::ArrayValue(elements) => {
                        let index = name
                            .lexeme
                            .parse::<usize>()
                            .map_err(|_| format!("Invalid array index: {}", name.lexeme))?;
                        elements
                            .get(index)
                            .cloned()
                            .ok_or_else(|| format!("Index out of bounds: {}", index))
                    }
                    _ => Err(format!("Trying to index a non-array value")),
                }
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
                        "Error 107: Decrement not implemented for {}",
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
                    (_, TokenType::Percent) => Err(format!(
                        "Error 107: Percent not implemented for {}",
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
                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),
                    (Number(x), TokenType::Greater, Number(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (StringValue(x), TokenType::Greater, StringValue(y)) => {
                        Ok(LiteralValue::from_bool(x.len() > y.len()))
                    }
                    (Number(x), TokenType::GreaterEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                    (StringValue(x), TokenType::GreaterEqual, StringValue(y)) => {
                        Ok(LiteralValue::from_bool(x.len() >= y.len()))
                    }
                    (Number(x), TokenType::Less, Number(y)) => Ok(LiteralValue::from_bool(x < y)),
                    (StringValue(x), TokenType::Less, StringValue(y)) => {
                        Ok(LiteralValue::from_bool(x.len() < y.len()))
                    }
                    (Number(x), TokenType::LessEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (StringValue(x), TokenType::LessEqual, StringValue(y)) => {
                        Ok(LiteralValue::from_bool(x.len() <= y.len()))
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
                    (x, TokenType::BangEqual, y) => Ok(LiteralValue::from_bool(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(LiteralValue::from_bool(x == y)),
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
pub fn run_tron_function(
    tronfun: TronFunctionImpl,
    arguments: &Vec<Expr>,
    eval_env: Environment,
) -> Result<LiteralValue, String> {
    if arguments.len() != tronfun.arity {
        return Err(format!(
            "Error 108: Callable {} expected {} arguments but got {}",
            tronfun.name,
            tronfun.arity,
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
    let fun_env = tronfun.parent_env.enclose();
    for (i, val) in arg_vals.iter().enumerate() {
        fun_env.define(tronfun.params[i].lexeme.clone(), (*val).clone());
    }
    let mut int = Interpreter::with_env(fun_env);
    for i in 0..(tronfun.body.len()) {
        let result = int.interpret(vec![&tronfun.body[i]]);
        if let Err(e) = result {
            return Err(e.red().to_string());
        } else if let Some(value) = int.specials.get("return") {
            return Ok(value.clone());
        }
    }
    Ok(LiteralValue::Nil)
}
