use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::scanner;
use crate::scanner::{Token, TokenType};
use std::cell::RefCell;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone)]
pub enum CallableImpl {
    JekoFunction(JekoFunctionImpl),
    NativeFunction(NativeFunctionImpl),
}
use CallableImpl::*;

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
    JekoClass {
        name: String,
        methods: HashMap<String, JekoFunctionImpl>,
        superclass: Option<Box<LiteralValue>>,
    },
    JekoInstance {
        class: Box<LiteralValue>,
        fields: Rc<RefCell<Vec<(String, LiteralValue)>>>,
    },
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

macro_rules! class_name {
    ($class:expr) => {{
        if let LiteralValue::JekoClass {
            name,
            methods: _,
            superclass: _,
        } = &**$class
        {
            name
        } else {
            panic!("Unreachable")
        }
    }};
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
            LiteralValue::JekoClass {
                name,
                methods: _,
                superclass: _,
            } => format!("Class '{name}'"),
            LiteralValue::JekoInstance { class, fields: _ } => {
                format!("Instance of '{}'", class_name!(class))
            }
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
            LiteralValue::JekoClass {
                name: _,
                methods: _,
                superclass: _,
            } => "Class",
            LiteralValue::JekoInstance { class, fields: _ } => &class_name!(class),
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
            JekoClass { .. } => panic!("Cannot use class as a falsy value"),
            _ => panic!("Not valid as a boolean value"),
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
            JekoClass { .. } => panic!("Cannot use class as a truthy value"),
            _ => panic!("Not valid as a boolean value"),
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
    This {
        id: usize,
        keyword: Token,
    },
    Super {
        id: usize,
        keyword: Token,
        method: Token,
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
            Expr::This { id, keyword: _ } => *id,
            Expr::Super {
                id,
                keyword: _,
                method: _,
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
    #[allow(dead_code)]
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
            Expr::This { id: _, keyword: _ } => format!("(this)"),
            Expr::Super {
                id: _,
                keyword: _,
                method,
            } => format!("(super {})", method.lexeme),
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
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
                    Err(format!("Variable {} has not been declared", name.lexeme))
                }
            }
            Expr::Variable { id: _, name } => match environment.get(&name.lexeme, self.get_id()) {
                Some(value) => Ok(value.clone()),
                None => Err(format!(
                    "Variable '{}' has not been declared at distance {:?}",
                    name.lexeme,
                    environment.get_distance(self.get_id())
                )),
            },
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                let callable: LiteralValue = (*callee).evaluate(environment.clone())?;
                let callable_clone = callable.clone();
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
                    JekoClass {
                        name: _,
                        methods,
                        superclass: _,
                    } => {
                        let instance = JekoInstance {
                            class: Box::new(callable_clone.clone()),
                            fields: Rc::new(RefCell::new(vec![])),
                        };
                        if let Some(init_method) = methods.get("init") {
                            if init_method.arity != arguments.len() {
                                return Err(
                                    "Invalid number of arguments in constructor".to_string()
                                );
                            }
                            let mut init_method = init_method.clone();
                            init_method.parent_env = init_method.parent_env.enclose();
                            init_method
                                .parent_env
                                .define("this".to_string(), instance.clone());

                            if let Err(msg) = run_jeko_function(init_method, arguments, environment)
                            {
                                return Err(msg);
                            }
                        }

                        Ok(instance)
                    }
                    other => Err(format!("{} is not callable", other.to_type())),
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
                ttype => Err(format!("Invalid token in logical expression: {}", ttype)),
            },
            Expr::Get {
                id: _,
                object,
                name,
            } => {
                let obj_value = object.evaluate(environment.clone())?;
                if let JekoInstance { class, fields } = obj_value.clone() {
                    for (field_name, value) in (*fields.borrow()).iter() {
                        if field_name == &name.lexeme {
                            return Ok(value.clone());
                        }
                    }
                    if let JekoClass {
                        name: _,
                        methods: _,
                        superclass: _,
                    } = class.as_ref()
                    {
                        if let Some(method) = find_method(&name.lexeme, *class.clone()) {
                            let mut callable_impl = method.clone();
                            let new_env = callable_impl.parent_env.enclose();
                            new_env.define("this".to_string(), obj_value.clone());
                            callable_impl.parent_env = new_env;
                            return Ok(Callable(JekoFunction(callable_impl)));
                        }
                    } else {
                        panic!("The class field on an instance was not a JekoClass");
                    }
                    Err(format!("No field named {} on this instance", name.lexeme))
                } else {
                    Err(format!(
                        "Cannot access property on type {}",
                        obj_value.to_type()
                    ))
                }
            }
            Expr::Set {
                id: _,
                object,
                name,
                value,
            } => {
                let obj_value = object.evaluate(environment.clone())?;
                if let JekoInstance { class: _, fields } = obj_value {
                    let value = value.evaluate(environment.clone())?;

                    let mut idx = 0;
                    let mut found = false;
                    for i in 0..(*fields.borrow()).len() {
                        let field_name = &(*fields.borrow())[i].0;
                        if field_name == &name.lexeme {
                            idx = i;
                            found = true;
                            break;
                        }
                    }

                    if found {
                        (*fields.borrow_mut())[idx].1 = value.clone();
                    } else {
                        (*fields.borrow_mut()).push((name.lexeme.clone(), value));
                    }

                    Ok(Nil)
                } else {
                    Err(format!(
                        "Cannot set property on type {}",
                        obj_value.to_type()
                    ))
                }
            }
            Expr::This { id: _, keyword: _ } => {
                let this = environment
                    .get("this", self.get_id())
                    .expect("Couldn't lookup 'this'");
                Ok(this)
            }
            Expr::Super {
                id: _,
                keyword: _,
                method,
            } => {
                let superclass = environment.get("super", self.get_id()).expect(&format!(
                    "Couldn't lookup 'super':\n---------------\n{}---------------\n",
                    environment.dump(0)
                ));

                let instance = environment.get_this_instance(self.get_id()).unwrap();

                if let JekoClass {
                    name: _,
                    methods,
                    superclass: _,
                } = superclass.clone()
                {
                    if let Some(method_value) = methods.get(&method.lexeme) {
                        let mut method = method_value.clone();
                        method.parent_env = method.parent_env.enclose();
                        method
                            .parent_env
                            .define("this".to_string(), instance.clone());
                        Ok(Callable(JekoFunction(method)))
                    } else {
                        Err(format!(
                            "No method named {} on superclass {}",
                            method.lexeme,
                            superclass.to_type()
                        ))
                    }
                } else {
                    panic!("The superclass field on an instance was not a JekoClass");
                }
            }
            Expr::Grouping { id: _, expression } => expression.evaluate(environment),
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let right = right.evaluate(environment)?;

                match (&right, operator.token_type) {
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (_, TokenType::Minus) => {
                        Err(format!("Minus not implemented for {}", right.to_type()))
                    }
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, ttype) => Err(format!("{} is not a valid unary operator", ttype)),
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
                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
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

                    (StringValue(_), op, Number(_)) => {
                        Err(format!("{} is not defined for string and number", op))
                    }
                    (Number(_), op, StringValue(_)) => {
                        Err(format!("{} is not defined for string and number", op))
                    }

                    (StringValue(s1), TokenType::Plus, StringValue(s2)) => {
                        Ok(StringValue(format!("{}{}", s1, s2)))
                    }

                    (x, TokenType::BangEqual, y) => Ok(LiteralValue::from_bool(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(LiteralValue::from_bool(x == y)),
                    (StringValue(s1), TokenType::Greater, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1 > s2))
                    }
                    (StringValue(s1), TokenType::GreaterEqual, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1 >= s2))
                    }
                    (StringValue(s1), TokenType::Less, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1 < s2))
                    }
                    (StringValue(s1), TokenType::LessEqual, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1 <= s2))
                    }
                    (x, ttype, y) => Err(format!(
                        "{} is not implemented for operands {:?} and {:?}",
                        ttype, x, y
                    )),
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

pub fn run_jeko_function(
    jekofun: JekoFunctionImpl,
    arguments: &Vec<Expr>,
    eval_env: Environment,
) -> Result<LiteralValue, String> {
    if arguments.len() != jekofun.arity {
        return Err(format!(
            "Callable {} expected {} arguments but got {}",
            jekofun.name,
            jekofun.arity,
            arguments.len()
        ));
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
            return Err(e);
        }
        if let Some(value) = int.specials.get("return") {
            return Ok(value.clone());
        }
    }

    Ok(LiteralValue::Nil)
}

pub fn find_method(name: &str, class: LiteralValue) -> Option<JekoFunctionImpl> {
    if let JekoClass {
        name: _,
        methods,
        superclass,
    } = class
    {
        if let Some(fun) = methods.get(name) {
            return Some(fun.clone());
        }
        if let Some(superclass) = superclass {
            return find_method(name, *superclass.clone());
        }
        None
    } else {
        panic!("Cannot find method on non-class");
    }
}
