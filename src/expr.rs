use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::panic;
use crate::scanner::{self, Token, TokenType};
use rand::Rng;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::process::{self, exit};
use std::rc::Rc;
use std::collections::HashMap;

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
    pub params: Vec<(Token, Option<Token>)>,
    pub body: Vec<Box<Stmt>>,
}
#[derive(Clone)]
pub struct NativeFunctionImpl {
    pub name: String,
    pub arity: usize,
    pub fun: Rc<dyn Fn(&Vec<LiteralValue>) -> LiteralValue>,
}
#[allow(dead_code)]
#[derive(Clone)]
pub enum LiteralValue {
    Integer(i32),
    BigInteger(i128),
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
    ArrayValue(Vec<LiteralValue>),
    Callable(CallableImpl),
    ExactValue(String), 
    Object(HashMap<String, LiteralValue>),
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
            (Integer(x), Integer(y)) => x == y,
            (BigInteger(x), BigInteger(y)) => x == y,
            (Integer(x), Number(y)) => x.to_owned() as f32 == *y,
            (BigInteger(x), Number(y)) => x.to_owned() as f32 == *y,
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
        _ => {
            panic("\n Could not unwrap as f64");
            0.0
        }
    }
}
fn unwrap_as_i128(literal: Option<scanner::LiteralValue>) -> i128 {
    match literal {
        Some(scanner::LiteralValue::IntegerValue(x)) => x as i128,
        _ => {
            panic("\n Could not unwrap as i128");
            0
        }
    }
}
fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        _ => {
            panic("\n Could not unwrap as string");
            "".to_string()
        }
    }
}
impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {   
            LiteralValue::Object(fields) => {
                let fields_str = fields.iter()
                    .map(|(key, value)| format!("{}: {}", key, value.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", fields_str)
            }
            LiteralValue::ExactValue(x) => x.to_string(),
            LiteralValue::Integer(x) => x.to_string(),
            LiteralValue::BigInteger(x) => x.to_string(),
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::ArrayValue(x) => format!("\"{:?}\"", x),
            LiteralValue::StringValue(x) => format!("\"{}\"", x),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "null".to_string(),
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
            LiteralValue::Object(_) => "object",
            LiteralValue::ExactValue(x) => x,
            LiteralValue::BigInteger(_) => "big integer",
            LiteralValue::Number(_) => "number",
            LiteralValue::Integer(_) => "integer",
            LiteralValue::StringValue(_) => "string",
            LiteralValue::ArrayValue(_) => "array",
            LiteralValue::True => "bool",
            LiteralValue::False => "bool",
            LiteralValue::Nil => "null",
            LiteralValue::Callable(_) => "callable",
        }
    }
    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f64(token.literal) as f32),
            TokenType::Integer => Self::Integer(unwrap_as_i128(token.literal) as i32),
            TokenType::StringLit => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => {
                panic(&format!("\n Could not create LiteralValue from {:?}", token).to_string());
                Self::Nil
            }
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
            Object(_) => {
                False
            }
            ExactValue(s) => {
                if s.len() == 0 {
                    True
                } else {
                    False
                }
            }
            Number(x) => {
                if *x == 0.0 as f32 {
                    True
                } else {
                    False
                }
            }
            Integer(x) => {
                if *x == 0 as i32 {
                    True
                } else {
                    False
                }
            }
            BigInteger(x) => {
                if *x == 0 as i128 {
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
            Callable(_) => {
                panic("\n Cannot use Callable as a falsy value");
                LiteralValue::Nil
            }
        }
    }
    pub fn is_truthy(&self) -> LiteralValue {
        match self {
            Object(_) => {
                True
            }
            ExactValue(s) => {
                if s.len() == 0 {
                    False
                } else {
                    True
                }
            }
            Number(x) => {
                if *x == 0.0 as f32 {
                    False
                } else {
                    True
                }
            }
            Integer(x) => {
                if *x == 0 as i32 {
                    False
                } else {
                    True
                }
            }
            BigInteger(x) => {
                if *x == 0 as i128 {
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
            Callable(_) => {
                panic("\n Cannot use Callable as a truthy value");
                LiteralValue::Nil
            }
        }
    }
}
use crate::stmt::Stmt;
#[allow(dead_code)]
#[derive(Clone)]
pub enum Expr {  
     ObjectLiteral {
        id: usize,
        properties: Vec<(Token, Expr)>,
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
    Unary {
        id: usize,
        operator: Token,
        right: Box<Expr>,
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
            Expr::ObjectLiteral { id, properties: _ } => *id,
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
            Expr::Grouping { id, expression: _ } => *id,
            Expr::Literal { id, value: _ } => *id,
            Expr::Logical {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expr::Unary {
                id,
                operator: _,
                right: _,
            } => *id,
            Expr::Variable { id, name: _ } => *id,
        }
    }
}
impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::ObjectLiteral { id: _, properties } => {
                let properties_str = properties.iter()
                    .map(|(key, value)| format!("{}: {}", key.lexeme, value.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", properties_str)
            }
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
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({}({}))", operator_str, right_str)
            }
            Expr::Variable { id: _, name } => format!("(let {})", name.lexeme),
        }
    }

    pub fn evaluate(&self, environment: Environment) -> Result<LiteralValue, String> {
        match self {
            Expr::ObjectLiteral { id: _, properties } => {
                let mut fields = HashMap::new();
                for (key, value_expr) in properties {
                    let value = value_expr.evaluate(environment.clone())?;
                    fields.insert(key.lexeme.clone(), value);
                }
                Ok(LiteralValue::Object(fields))
            }
            Expr::Array { id: _, elements } => {
                if elements.len() == 2 {
                    let array = elements[0].evaluate(environment.clone())?;
                    let index = elements[1].evaluate(environment.clone())?;
                    if let LiteralValue::Integer(index_num) = index {
                        if let LiteralValue::ArrayValue(arr) = array {
                            let idx = index_num as usize;
                            return arr.get(idx).cloned().ok_or_else(|| {
                                panic("\n Array index out of bounds");
                                process::exit(1);
                            });
                        }
                    }
                    panic("\n Invalid array indexing operation");
                    process::exit(1);
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
              if name.lexeme.chars().next().unwrap().is_uppercase() {
                    panic(&format!(
                        "\n Immutable variable {} cannot be re-declared",
                        name.lexeme.to_string()
                    ));
                    exit(1);
                }
                    let new_value = (*value).evaluate(environment.clone())?;
                    let assign_success =
                        environment.assign(&name.lexeme, new_value.clone(), self.get_id());
                    let type_annotation = environment.get_type_annotation(&name.lexeme);
                    match type_annotation {
        Some(expected_type) => {
            match (expected_type.as_str(), &new_value) {
                ("number", LiteralValue::Number(_)) => {},
                ("integer", LiteralValue::Integer(_)) => {},
                ("Biginteger", LiteralValue::BigInteger(_)) => {},
                ("string", LiteralValue::StringValue(_)) => {},
                ("array", LiteralValue::ArrayValue(_)) => {},
                ("bool", LiteralValue::True) | ("bool", LiteralValue::False) => {},
                ("null", LiteralValue::Nil) => {},
                _ => panic(&format!("Type mismatch: variable '{:?}' expected type '{}', but got value of type '{}'", name.lexeme, expected_type, new_value.to_type())),
            }
        },
        None => {
            // No type annotation exists, so no type checking is necessary
        },
    }
                if assign_success {
                    Ok(new_value)
                } else {
                    panic(&format!("\n Variable {} has not been declared",name.lexeme.to_string()));
                    process::exit(1);
                }
            }
            Expr::Variable { id: _, name } => match environment.get(&name.lexeme, self.get_id()) {
                Some(value) => Ok(value.clone()),
                None => {
                    panic(&format!(
                        "Variable '{}' has not been declared at distance {:?}",
                        name.lexeme,
                        environment.get_distance(self.get_id())
                    ));
                    process::exit(1);
                }
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
                    Callable(CallableImpl::NativeFunction(nativefun)) => {
                        let mut evaluated_arguments = vec![];
                        for argument in arguments {
                            evaluated_arguments.push(argument.evaluate(environment.clone())?);
                        }
                        Ok((nativefun.fun)(&evaluated_arguments))
                    }
                    other => {
                        panic(&format!("\n {} is not callable", other.to_type()));
                        process::exit(1);
                    }
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
                ttype => {
                    panic(&format!(
                        "\n Invalid token in logical expression: {}",
                        ttype
                    ));
                    process::exit(1);
                }
            },
            Expr::Grouping { id: _, expression } => expression.evaluate(environment),
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let right = right.evaluate(environment)?;
                let mut rng = rand::thread_rng();
                match (&right, operator.token_type) {
                    // minus
                    (Number(x), TokenType::Minus) => Ok(Number(-x)), 
                    (Integer(x), TokenType::Minus) => { panic(&format!(
                            "\n Integer can't be less than 0 {}",
                            x
                        ));
                        process::exit(1)},
                        (BigInteger(x), TokenType::Minus) => { panic(&format!(
                            "\n BigInteger can't be less than 0 {}",
                            x
                        ));
                        process::exit(1)},
                    (True, TokenType::Minus) => Ok(False),
                    (False, TokenType::Minus) => Ok(True),
                     (_, TokenType::Minus) => {
                        panic(&format!("\n Minus not implemented for {}", right.to_type()));
                        process::exit(1);
                    }
                    (Number(x), TokenType::Increment) => Ok(Number(x + 1.0)),
                    (Number(x), TokenType::Decrement) => Ok(Number(x - 1.0)),
                    (Number(x), TokenType::Power2) => Ok(Number(x * x)),
                    (Number(x), TokenType::Root2) => Ok(Number(x.sqrt())),
                    (Number(x), TokenType::Random) => {
                        Ok(Number(rng.gen_range(0..*x as i32) as f32))
                    }
                    (Integer    (x), TokenType::Increment) => Ok(Integer(x + 1)),
                    (Integer(x), TokenType::Decrement) => Ok(Integer(x - 1)),
                    (Integer(x), TokenType::Power2) => Ok(Integer(x * x)),
                    (Integer(x), TokenType::Root2) => {
                        panic(&format!(
                            "\n You can't take root from Integer {}",
                            x
                        ));
                        process::exit(1);
                    },
                    (Integer(x), TokenType::Random) => {
                        Ok(Integer(rng.gen_range(0..*x)))
                    }
                   

                   (BigInteger    (x), TokenType::Increment) => Ok(BigInteger(x + 1)),
                    (BigInteger(x), TokenType::Decrement) => Ok(BigInteger(x - 1)),
                    (BigInteger(x), TokenType::Power2) => Ok(BigInteger(x * x)),
                    (BigInteger(x), TokenType::Root2) => {
                        panic(&format!(
                            "\n You can't take root from BigInteger {}",
                            x
                        ));
                        process::exit(1);
                    },
                    (BigInteger(x), TokenType::Random) => {
                        Ok(BigInteger(rng.gen_range(0..*x)))
                    }
                    (_, TokenType::Increment) => {
                        panic(&format!(
                            "\n Increment not implemented for {}",
                            right.to_type()
                        ));
                        process::exit(1);
                    }
                    (_, TokenType::Decrement) => {
                        panic(&format!(
                            "\n Decrement not implemented for {}",
                            right.to_type()
                        ));
                        process::exit(1);
                    }
                    (_, TokenType::Percent) => {
                        panic(&format!(
                            "\n Percent not implemented for {}",
                            right.to_type()
                        ));
                        process::exit(1);
                    }
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, ttype) => {
                        panic(&format!("\n {} is not a valid unary operator", ttype));
                        exit(1);
                    }
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
                match (&left, operator.token_type, &right) {
                    (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),
                    (Integer(x), TokenType::Plus, Number(y)) => Ok(Number(*x as f32 + y)),
                    (Integer(x), TokenType::Plus, Integer(y)) => Ok(Integer(x + y)),
                    (BigInteger(x), TokenType::Plus, Number(y)) => Ok(Number(*x as f32 + y)),
                    (BigInteger(x), TokenType::Plus, BigInteger(y)) => Ok(BigInteger(x - y)),
                    (StringValue(x), TokenType::Plus, Number(y)) => {
                        Ok(StringValue(format!("{}{}", x, y.to_string())))
                    }
                    (Number(x), TokenType::Plus, StringValue(y)) => {
                        Ok(StringValue(format!("{}{}", x.to_string(), y)))
                    }
                    (Integer(x), TokenType::Plus, StringValue(y)) => {
                        Ok(StringValue(format!("{}{}", x.to_string(), y)))
                    }
                    (BigInteger(x), TokenType::Plus, StringValue(y)) => {
                        Ok(StringValue(format!("{}{}", x.to_string(), y)))
                    }
                    (StringValue(x), TokenType::Plus, StringValue(y)) => {
                        Ok(StringValue(format!("{}{}", x.to_string(), y)))
                    }
                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (Integer(x), TokenType::Minus, Number(y)) => Ok(Number(*x as f32 - y)),
                    (Number(x), TokenType::Minus, Integer(y)) => Ok(Number(x - *y as f32)),
                    (Integer(x), TokenType::Minus, Integer(y)) => Ok(Integer(x - y)),
                    (Number(x), TokenType::Minus, BigInteger(y)) => Ok(Number(x - *y as f32)),
                    (BigInteger(x), TokenType::Minus, BigInteger(y)) => Ok(BigInteger(x - y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),
                    (Integer(x), TokenType::Star, Integer(y)) => Ok(Integer(x * y)),
                    (Integer(x), TokenType::Slash, Integer(y)) => Ok(Integer(x / y)),
                    (BigInteger(x), TokenType::Star, BigInteger(y)) => Ok(BigInteger(x * y)),
                    (BigInteger(x), TokenType::Slash, BigInteger(y)) => Ok(BigInteger(x / y)),
                    (Number(x), TokenType::Greater, Number(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (Integer(x), TokenType::Greater, Integer(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (BigInteger(x), TokenType::Greater, BigInteger(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (StringValue(x), TokenType::Greater, StringValue(y)) => {
                        Ok(LiteralValue::from_bool(x.len() > y.len()))
                    }
                    (Number(x), TokenType::GreaterEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                     (Integer(x), TokenType::GreaterEqual, Integer(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                     (BigInteger(x), TokenType::GreaterEqual, BigInteger(y)) => {
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
                    (Integer(x), TokenType::Less, Integer(y)) => Ok(LiteralValue::from_bool(x < y)),
                    (Integer(x), TokenType::LessEqual, Integer(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (BigInteger(x), TokenType::Less, BigInteger(y)) => Ok(LiteralValue::from_bool(x < y)),
                    (BigInteger(x), TokenType::LessEqual, BigInteger(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (StringValue(x), TokenType::LessEqual, StringValue(y)) => {
                        Ok(LiteralValue::from_bool(x.len() <= y.len()))
                    }
                    (StringValue(_), op, Number(_)) => {
                        panic(&format!("\n {} is not defined for string and number", op));
                        exit(1);
                    }
                    (Number(_), op, StringValue(_)) => {
                        panic(&format!("\n {} is not defined for string and number", op));
                        exit(1);
                    }
                    (x, TokenType::BangEqual, y) => Ok(LiteralValue::from_bool(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(LiteralValue::from_bool(x == y)),
                    (x, ttype, y) => {
                        panic(&format!(
                            "{} is not implemented for operands {:?} and {:?}",
                            ttype, x, y
                        ));
                        exit(1)
                    }
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
            "Callable {} expected {} arguments but got {}",
            tronfun.name,
            tronfun.arity,
            arguments.len()
        ));
    }
    let mut arg_vals = vec![];
    for arg in arguments {
        let val = arg.evaluate(eval_env.clone())?;
        arg_vals.push(val);
    }
    let fun_env = tronfun.parent_env.enclose();
    for (i, val) in arg_vals.iter().enumerate() {
        if i < tronfun.params.len() {
            let (param_name_token, param_type_token_option) = &tronfun.params[i];
            let param_name_lexeme = &param_name_token.lexeme;

            // Check if a type was provided for the parameter and perform type checking if necessary
            if let Some(param_type_token) = param_type_token_option {
                let param_type_lexeme = &param_type_token.lexeme;

                // Type checking logic
                match (param_type_lexeme.as_str(), val) {
                    ("number", LiteralValue::Number(_)) => {}
                    ("integer", LiteralValue::Integer(_)) => {}
                    ("bigintiger", LiteralValue::BigInteger(_)) => {}
                    ("string", LiteralValue::StringValue(_)) => {}
                    ("array", LiteralValue::ArrayValue(_)) => {}
                    ("bool", LiteralValue::True) | ("bool", LiteralValue::False) => {}
                    ("null", LiteralValue::Nil) => {}
                    _ => {
                        panic(&format!(
                        "Type mismatch: expected argument of type '{}' for parameter '{}', but got value of type '{}'",
                         val.to_type(),
                        param_name_lexeme,param_type_lexeme,
                       
                    ));
                    }
                }
            }

            fun_env.define(param_name_lexeme.clone(), val.clone());
        } else {
            panic("Function call argument count does not match parameter count.");
        }
    }
    let mut int = Interpreter::with_env(fun_env);
    for stmt in tronfun.body.iter() {
        let result = int.interpret(vec![stmt.as_ref()]);
        if let Err(e) = result {
            return Err(e.to_string());
        } else if let Some(value) = int.specials.get("return") {
            return Ok(value.clone());
        }
    }
    Ok(LiteralValue::Nil)
}
