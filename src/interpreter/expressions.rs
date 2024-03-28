#[derive(Clone)]
pub enum CallableImpl {
    Function(FunctionImpl),
    StdFunction(StdFunctionImpl),
}
#[derive(Clone, Debug)]
pub struct FunctionImpl {
    pub name: String,
    pub arity: usize,
    pub parent_env: Environment,
    pub params: Vec<(Token, Token)>,
    pub body: Vec<Box<Statement>>,
    pub output_type: Token,
}
#[derive(Clone)]
pub struct StdFunctionImpl {
    pub name: String,
    pub arity: usize,
    pub function: Rc<dyn Fn(&Vec<TronType>) -> TronType>,
}
#[derive(Clone)]

pub struct ObjectImpl {
    pub name: String,
    pub params: Vec<(Token, Expression)>,
}

#[derive(Clone)]
pub struct CallbackImpl {
    pub id: usize,
    pub args: Vec<(Token, Option<Token>)>,
    pub body: Vec<Box<Statement>>,
}

#[derive(Clone)]

pub enum TronType {
    Number(f32),
    StringValue(String),
    True,
    False,
    Null,
    ArrayValue(Vec<TronType>),
    Callable(CallableImpl),
    Object(HashMap<String, TronType>),
}

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    rc::Rc,
};

use TronType::*;
impl std::fmt::Debug for TronType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl PartialEq for TronType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number(x), Number(y)) => x == y,
            (
                Callable(CallableImpl::Function(FunctionImpl { name, arity, .. })),
                Callable(CallableImpl::Function(FunctionImpl {
                    name: name2,
                    arity: arity2,
                    ..
                })),
            ) => name == name2 && arity == arity2,
            (
                Callable(CallableImpl::StdFunction(StdFunctionImpl { name, arity, .. })),
                Callable(CallableImpl::StdFunction(StdFunctionImpl {
                    name: name2,
                    arity: arity2,
                    ..
                })),
            ) => name == name2 && arity == arity2,
            (StringValue(x), StringValue(y)) => x == y,
            (True, True) => true,
            (False, False) => true,
            (Null, Null) => true,
            _ => false,
        }
    }
}
fn unwrap_as_f64(literal: Option<LiteralValue>) -> f64 {
    match literal {
        Some(LiteralValue::NumericValue(x)) => x as f64,
        _ => {
            TronError::throw("E4007", 0, vec!["uknown".to_string(), "f64".to_string()]);
            0.0
        }
    }
}
fn unwrap_as_string(literal: Option<LiteralValue>) -> String {
    match literal {
        Some(LiteralValue::StringValue(s)) => s.clone(),
        _ => {
            TronError::throw("E4007", 0, vec!["uknown".to_string(), "string".to_string()]);
            "".to_string()
        }
    }
}
impl TronType {
    pub fn to_string(&self) -> String {
        match self {
            TronType::Object(fields) => {
                let fields_str = fields
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", fields_str)
            }
            TronType::Number(x) => x.to_string(),
            TronType::ArrayValue(x) => format!("\"{:?}\"", x),
            TronType::StringValue(x) => format!("\"{}\"", x),
            TronType::True => "true".to_string(),
            TronType::False => "false".to_string(),
            TronType::Null => "null".to_string(),
            TronType::Callable(CallableImpl::Function(FunctionImpl { name, arity, .. })) => {
                format!("{name}/{arity}")
            }
            TronType::Callable(CallableImpl::StdFunction(StdFunctionImpl {
                name, arity, ..
            })) => format!("{name}/{arity}"),
        }
    }
    pub fn to_type(&self) -> &str {
        match self {
            TronType::Object(_) => "object",
            TronType::Number(_) => "number",
            TronType::StringValue(_) => "string",
            TronType::ArrayValue(_) => "array",
            TronType::True => "boolean",
            TronType::False => "boolean",
            TronType::Null => "null",
            TronType::Callable(_) => "function",
        }
    }
    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f64(token.literal) as f32),
            TokenType::StringLit => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Null => Self::Null,
            _ => {
                TronError::throw(
                    "E001",
                    0,
                    vec![token.token_type.to_string(), "uknown".to_string()],
                );
                Self::Null
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
    pub fn is_falsy(&self) -> TronType {
        match self {
            Object(_) => False,
            Number(x) => {
                if *x == 0.0 as f32 {
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
            Null => True,
            Callable(_) => True,
        }
    }
    pub fn is_truthy(&self) -> TronType {
        match self {
            Object(_) => True,
            Number(x) => {
                if *x == 0.0 as f32 {
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
            Null => False,
            Callable(_) => False,
        }
    }
}
use crate::{scanner::Statement, utils::TronError, Interpreter, LiteralValue, Token, TokenType};

use super::Environment;
#[derive(Clone)]
pub enum Expression {
    Object {
        id: usize,
        properties: Vec<(Token, Expression)>,
    },
    ObjectCall {
        id: usize,
        key: Token,
        name: Token,
    },
    Array {
        id: usize,
        elements: Vec<Box<Expression>>,
    },
    Assign {
        id: usize,
        name: Token,
        value: Box<Expression>,
    },
    Binary {
        id: usize,
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Call {
        id: usize,
        callee: Box<Expression>,
        paren: Token,
        arguments: Vec<Expression>,
    },
    Grouping {
        id: usize,
        expression: Box<Expression>,
    },
    Literal {
        id: usize,
        value: TronType,
    },
    Logical {
        id: usize,
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        id: usize,
        operator: Token,
        right: Box<Expression>,
    },
    Variable {
        id: usize,
        name: Token,
    },
    Function {
        id: usize,
        name: Token,
        params: Vec<(Token, Token)>,
        body: Vec<Box<Statement>>,
        output_type: Token,
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.get_id(), self.to_string())
    }
}
impl Hash for Expression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state)
    }
}
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        let ptr = std::ptr::addr_of!(self);
        let ptr2 = std::ptr::addr_of!(other);
        ptr == ptr2
    }
}
impl Eq for Expression {}
impl Expression {
    pub fn get_id(&self) -> usize {
        match self {
            Expression::Function {
                id,
                name: _,
                params: _,
                body: _,
                output_type: _,
            } => *id,
            Expression::ObjectCall {
                id,
                key: _,
                name: _,
            } => *id,
            Expression::Object { id, properties: _ } => *id,
            Expression::Array { id, elements: _ } => *id,
            Expression::Assign {
                id,
                name: _,
                value: _,
            } => *id,
            Expression::Binary {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expression::Call {
                id,
                callee: _,
                paren: _,
                arguments: _,
            } => *id,
            Expression::Grouping { id, expression: _ } => *id,
            Expression::Literal { id, value: _ } => *id,
            Expression::Logical {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expression::Unary {
                id,
                operator: _,
                right: _,
            } => *id,
            Expression::Variable { id, name: _ } => *id,
        }
    }
}
impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Function {
                id: _,
                name,
                params,
                body: _,
                output_type,
            } => format!(
                "{}({}): {}",
                name.lexeme,
                params
                    .iter()
                    .map(|(name, typ)| format!("{}: {}", name.lexeme, typ.lexeme).to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                output_type.lexeme
            ),
            Expression::Object { id: _, properties } => {
                let properties_str = properties
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key.lexeme, value.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", properties_str)
            }
            Expression::ObjectCall {
                id: _,
                key,
                name: _,
            } => format!("{}", key.lexeme), //
            Expression::Array { id: _, elements } => {
                let elements_str = elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", elements_str)
            }
            Expression::Assign { id: _, name, value } => {
                format!("({name:?} = {}", value.to_string())
            }
            Expression::Binary {
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
            Expression::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => format!("({} {:?})", (*callee).to_string(), arguments),
            Expression::Grouping { id: _, expression } => {
                format!("(group {})", (*expression).to_string())
            }
            Expression::Literal { id: _, value } => format!("{}", value.to_string()),
            Expression::Logical {
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
            Expression::Unary {
                id: _,
                operator,
                right,
            } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({}({}))", operator_str, right_str)
            }
            Expression::Variable { id: _, name } => format!("(let {})", name.lexeme),
        }
    }

    pub fn evaluate(&self, environment: Environment) -> Result<TronType, String> {
        match self {
            Expression::Function {
                id: _,
                name,
                params,
                body,
                output_type,
            } => {
                let function_impl = FunctionImpl {
                    name: name.clone().lexeme,
                    arity: params.len(),
                    parent_env: environment.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    output_type: output_type.clone(),
                };
                let callback_env = environment.enclose();

                let int = Interpreter::with_env(callback_env.clone());
                let callable = int.make_function(&Statement::FunctionStatement {
                    name: name.clone(),
                    params: function_impl.clone().params,
                    body: function_impl.clone().body,
                    output_type: function_impl.clone().output_type,
                    line: name.line_number,
                });
                let fun = TronType::Callable(CallableImpl::Function(callable));
                environment.define(function_impl.clone().name.clone(), fun);
                Ok(TronType::Callable(CallableImpl::Function(function_impl)))
            }
            Expression::Object { id: _, properties } => {
                let mut fields = HashMap::new();
                for (key, value_expr) in properties {
                    let value = value_expr.evaluate(environment.clone())?;
                    fields.insert(key.lexeme.clone(), value);
                }
                Ok(TronType::Object(fields))
            }
            Expression::ObjectCall { id, key, name } => {
                let object = environment.get(&name.lexeme, *id);
                match object {
                    Some(TronType::Object(fields)) => match fields.get(&key.lexeme) {
                        Some(value) => Ok(value.clone()),
                        None => Err(format!("Key '{}' not found in object", key.lexeme)),
                    },
                    _ => Err(format!("'{}' is not an object", key.lexeme)),
                }
            }
            Expression::Array { id: _, elements } => {
                if elements.len() == 2 {
                    let array = elements[0].evaluate(environment.clone())?;
                    let index = elements[1].evaluate(environment.clone())?;
                    if let TronType::Number(index_num) = index {
                        if let TronType::ArrayValue(arr) = array {
                            let idx = index_num as usize;
                            return arr.get(idx).cloned().ok_or_else(|| {
                                TronError::throw("E4009", 0, vec![]);
                                "".to_string()
                            });
                        }
                    }

                    TronError::throw("E4010", 0, vec![]);
                    Ok(TronType::Null)
                } else {
                    let mut array_elements = Vec::new();
                    for element_expr in elements.iter() {
                        let evaluated = element_expr.evaluate(environment.clone())?;
                        array_elements.push(evaluated);
                    }
                    Ok(TronType::ArrayValue(array_elements))
                }
            }
            Expression::Assign { id: _, name, value } => {
                if name.lexeme.chars().next().unwrap().is_uppercase() {
                    TronError::throw("E4012", name.line_number, vec![]);
                }
                let new_value = (*value).evaluate(environment.clone())?;
                let assign_success =
                    environment.assign(&name.lexeme, new_value.clone(), self.get_id());
                let type_annotation = environment.get_value_type(&name.lexeme);
                match type_annotation {
                    Some(expected_type) => match (expected_type.as_str(), &new_value) {
                        ("number", TronType::Number(_)) => {}
                        ("string", TronType::StringValue(_)) => {}
                        ("array", TronType::ArrayValue(_)) => {}
                        ("object", TronType::Object(_)) => {}
                        ("bool", TronType::True) | ("bool", TronType::False) => {}
                        ("null", TronType::Null) => {}
                        _ => TronError::throw(
                            "E4003",
                            0,
                            vec![
                                "variable".to_string(),
                                name.lexeme.to_string(),
                                expected_type,
                                new_value.to_type().to_string(),
                            ],
                        ),
                    },
                    None => {}
                }
                if assign_success {
                    Ok(new_value)
                } else {
                    TronError::throw("E4011", name.line_number, vec![name.lexeme.to_string()]);
                    Ok(TronType::Null)
                }
            }
            Expression::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                let callable: TronType = (*callee).evaluate(environment.clone())?;
                match callable {
                    Callable(CallableImpl::Function(tronfun)) => {
                        run_tron_function(tronfun, arguments, environment)
                    }
                    Callable(CallableImpl::StdFunction(nativefun)) => {
                        let mut evaluated_arguments = vec![];
                        for argument in arguments {
                            evaluated_arguments.push(argument.evaluate(environment.clone())?);
                        }
                        Ok((nativefun.function)(&evaluated_arguments))
                    }
                    _ => {
                        TronError::throw("E4013", 0, vec![]);
                        Ok(TronType::Null)
                    }
                }
            }
            Expression::Variable { id: _, name } => {
                let parts: Vec<&str> = name.lexeme.split('.').collect();
                if parts.len() == 2 {
                    let object_name = parts[0];
                    let key = parts[1];
                    match environment.get(object_name, self.get_id()) {
                        Some(TronType::Object(fields)) => match fields.get(key) {
                            Some(value) => Ok(value.clone()),
                            None => Err(format!(
                                "Key '{}' not found in object '{}'",
                                key, object_name
                            )),
                        },
                        _ => Err(format!("'{}' is not an object", object_name)),
                    }
                } else {
                    match environment.get(&name.lexeme, self.get_id()) {
                        Some(value) => Ok(value.clone()),
                        None => {
                            TronError::throw("E4011", name.line_number, vec![name.clone().lexeme]);
                            Ok(TronType::Null)
                        }
                    }
                }
            }
            Expression::Literal { id: _, value } => Ok((*value).clone()),
            Expression::Logical {
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
                operator => {
                    TronError::throw("E4016", 0, vec![operator.to_string()]);
                    Ok(TronType::Null)
                }
            },
            Expression::Grouping { id: _, expression } => expression.evaluate(environment),
            Expression::Unary {
                id: _,
                operator,
                right,
            } => {
                let right = right.evaluate(environment)?;
                match (&right, operator.token_type) {
                    // minus
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (True, TokenType::Minus) => Ok(False),
                    (False, TokenType::Minus) => Ok(True),
                    (e, TokenType::Minus) => {
                        TronError::throw("E4015", 0, vec!["minus".to_string(), e.to_string()]);
                        Ok(TronType::Null)
                    }
                    (Number(x), TokenType::Increment) => Ok(Number(x + 1.0)),
                    (Number(x), TokenType::Decrement) => Ok(Number(x - 1.0)),
                    (e, TokenType::Increment) => {
                        TronError::throw("E4015", 0, vec!["increment".to_string(), e.to_string()]);

                        Ok(TronType::Null)
                    }
                    (e, TokenType::Decrement) => {
                        TronError::throw("E4015", 0, vec!["decrement".to_string(), e.to_string()]);
                        Ok(TronType::Null)
                    }
                    (e, TokenType::Percent) => {
                        TronError::throw("E4015", 0, vec!["percent".to_string(), e.to_string()]);
                        Ok(TronType::Null)
                    }
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (e, f) => {
                        TronError::throw("E4015", 0, vec![f.to_string(), e.to_string()]);
                        Ok(TronType::Null)
                    }
                }
            }
            Expression::Binary {
                id: _,
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment.clone())?;
                let right = right.evaluate(environment.clone())?;
                match (&left, operator.token_type, &right) {
                    (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),
                    (StringValue(x), TokenType::Plus, Number(y)) => {
                        Ok(StringValue(format!("{}{}", x, y.to_string())))
                    }
                    (Number(x), TokenType::Plus, StringValue(y)) => {
                        Ok(StringValue(format!("{}{}", x.to_string(), y)))
                    }
                    (StringValue(x), TokenType::Plus, StringValue(y)) => {
                        Ok(StringValue(format!("{}{}", x.to_string(), y)))
                    }
                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),
                    (Number(x), TokenType::Greater, Number(y)) => Ok(TronType::from_bool(x > y)),
                    (StringValue(x), TokenType::Greater, StringValue(y)) => {
                        Ok(TronType::from_bool(x.len() > y.len()))
                    }
                    (Number(x), TokenType::GreaterEqual, Number(y)) => {
                        Ok(TronType::from_bool(x >= y))
                    }
                    (StringValue(x), TokenType::GreaterEqual, StringValue(y)) => {
                        Ok(TronType::from_bool(x.len() >= y.len()))
                    }
                    (Number(x), TokenType::Less, Number(y)) => Ok(TronType::from_bool(x < y)),
                    (StringValue(x), TokenType::Less, StringValue(y)) => {
                        Ok(TronType::from_bool(x.len() < y.len()))
                    }
                    (Number(x), TokenType::LessEqual, Number(y)) => Ok(TronType::from_bool(x <= y)),
                    (StringValue(x), TokenType::LessEqual, StringValue(y)) => {
                        Ok(TronType::from_bool(x.len() <= y.len()))
                    }
                    (StringValue(_), e, Number(_)) => {
                        TronError::throw(
                            "E4015",
                            0,
                            vec!["string and number".to_string(), e.to_string()],
                        );
                        Ok(TronType::Null)
                    }
                    (Number(_), e, StringValue(_)) => {
                        TronError::throw(
                            "E4015",
                            0,
                            vec!["number and string".to_string(), e.to_string()],
                        );
                        Ok(TronType::Null)
                    }
                    (x, TokenType::BangEqual, y) => Ok(TronType::from_bool(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(TronType::from_bool(x == y)),
                    (f, e, c) => {
                        TronError::throw(
                            "E4015",
                            0,
                            vec![
                                format!("{} and {}", f.to_string(), c.to_string()).to_string(),
                                e.to_string(),
                            ],
                        );
                        Ok(TronType::Null)
                    }
                }
            }
        }
    }
}
pub fn run_tron_function(
    tronfun: FunctionImpl,
    arguments: &Vec<Expression>,
    eval_env: Environment,
) -> Result<TronType, String> {
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
            let (param_name_token, param_type_token) = &tronfun.params[i];
            let param_name_lexeme = &param_name_token.lexeme;

            let param_type_lexeme = &param_type_token.lexeme;

            match (param_type_lexeme.as_str(), val) {
                ("number", TronType::Number(_)) => {}
                ("string", TronType::StringValue(_)) => {}
                ("array", TronType::ArrayValue(_)) => {}
                ("object", TronType::Object(_)) => {}
                ("bool", TronType::True) | ("bool", TronType::False) => {}
                ("null", TronType::Null) => {}
                _ => {
                    TronError::throw(
                        "E4002",
                        0,
                        vec![
                            tronfun.name.to_string(),
                            param_name_lexeme.to_string(),
                            val.to_type().to_string(),
                            param_type_lexeme.to_string(),
                        ],
                    );
                }
            }

            fun_env.define(param_name_lexeme.clone(), val.clone());
        } else {
            TronError::throw("E4014", 0, vec![]);
        }
    }
    let mut int = Interpreter::with_env(fun_env);
    for stmt in tronfun.body.iter() {
        let result = int.interpret(vec![stmt.as_ref()]);
        if let Err(_e) = result {
            TronError::throw("E4006", 0, vec![]);
        } else if let Some(value) = int.specials.get("return") {
            let output_type_lexeme = &tronfun.output_type.lexeme;
            let value_clone = value.clone();
            let value_clone_type = value_clone.to_type();
            let value_clone_string = value_clone.to_string();
            if !(output_type_lexeme == value_clone_type
                || output_type_lexeme.clone() == value_clone_string)
            {
                TronError::throw("E4017", 0, vec![]);
            }
            return Ok(value.clone());
        }
    }

    let output_type_lexeme = &tronfun.output_type.lexeme;
    if output_type_lexeme != "null" {
        TronError::throw("E4017", 0, vec![]);
    }

    Ok(TronType::Null)
}
