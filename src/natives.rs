use crate::expr::LiteralValue;
use std::io;

pub fn native_sin(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("sin function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().sin()),
        _ => LiteralValue::StringValue("sin function requires a numeric argument".to_string()),
    }
}
pub fn native_asin(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("asin function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().asin()),
        _ => LiteralValue::StringValue("asin function requires a numeric argument".to_string()),
    }
}
pub fn native_cos(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("cos function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().cos()),
        _ => LiteralValue::StringValue("cos function requires a numeric argument".to_string()),
    }
}
pub fn native_acos(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("acos function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().acos()),
        _ => LiteralValue::StringValue("acos function requires a numeric argument".to_string()),
    }
}
pub fn native_tan(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("tan function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().tan()),
        _ => LiteralValue::StringValue("tan function requires a numeric argument".to_string()),
    }
}
pub fn native_atan(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("atan function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().atan()),
        _ => LiteralValue::StringValue("atan function requires a numeric argument".to_string()),
    }
}
pub fn native_round(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("round function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.round()),
        _ => LiteralValue::StringValue("round function requires a numeric argument".to_string()),
    }
}
pub fn native_floor(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("floor function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.floor()),
        _ => LiteralValue::StringValue("floor function requires a numeric argument".to_string()),
    }
}
pub fn native_todgrees(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue(
            "to_degrees function takes exactly one argument".to_string(),
        );
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.to_degrees()),
        _ => {
            LiteralValue::StringValue("to_degrees function requires a numeric argument".to_string())
        }
    }
}
pub fn native_toradians(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue(
            "to_radians function takes exactly one argument".to_string(),
        );
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.to_radians()),
        _ => {
            LiteralValue::StringValue("to_radians function requires a numeric argument".to_string())
        }
    }
}
pub fn native_input(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("input function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::StringValue(n) => {
            println!("{}", n.to_string());
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            LiteralValue::StringValue(format!("{}", input))
        }
        _ => LiteralValue::StringValue("input function requires a string argument".to_string()),
    }
}
pub fn native_typeof(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("typeof function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::Number(_n) => LiteralValue::StringValue("number".to_string()),
        LiteralValue::StringValue(_n) => LiteralValue::StringValue("string".to_string()),
        LiteralValue::Nil => LiteralValue::StringValue("null".to_string()),
        LiteralValue::False => LiteralValue::StringValue("boolean".to_string()),
        LiteralValue::True => LiteralValue::StringValue("boolean".to_string()),
        LiteralValue::ArrayValue(_n) => LiteralValue::StringValue("array".to_string()),
        _ => LiteralValue::StringValue("uknown type".to_string()),
    }
}
pub fn native_len(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("len function takes exactly one argument".to_string());
    }
    match &args[0] {
        LiteralValue::StringValue(n) => LiteralValue::StringValue(n.len().to_string()),
        LiteralValue::ArrayValue(n) => LiteralValue::StringValue(n.len().to_string()),
        _ => {
            LiteralValue::StringValue("len function requires array or string argument".to_string())
        }
    }
}
pub fn native_push(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        return LiteralValue::StringValue("push function takes exactly two arguments".to_string());
    }
    match &args[0] {
        LiteralValue::ArrayValue(arr) => {
            let mut arr = arr.clone();
            arr.push(args[1].clone());
            LiteralValue::ArrayValue(arr)
        }
        _ => {
            LiteralValue::StringValue("push function requires array as first argument".to_string())
        }
    }
}
pub fn native_join(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        return LiteralValue::StringValue("join function takes exactly two arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (LiteralValue::ArrayValue(arr), LiteralValue::StringValue(join_str)) => {
            let mut strings = Vec::new();
            for val in arr.iter() {
                match val {
                    LiteralValue::Number(num) => strings.push(num.to_string()),
                    LiteralValue::StringValue(s) => strings.push(s.clone()),
                    // Add more types if needed
                    _ => return LiteralValue::StringValue("join function requires an array of strings or numbers".to_string()),
                }
            }
            let joined = strings.join(join_str);
            LiteralValue::StringValue(joined)
        },
        _ => LiteralValue::StringValue("join function requires an array as the first argument and a string as the second argument".to_string()),
    }
}
pub fn native_pop(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("pop function takes exactly one argument".to_string());
    }
    if let LiteralValue::ArrayValue(arr) = &args[0] {
        let mut arr = arr.clone();
        arr.pop();
        LiteralValue::ArrayValue(arr)
    } else {
        LiteralValue::StringValue("pop function requires an array as the argument".to_string())
    }
}

pub fn native_shift(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("shift function takes exactly one argument".to_string());
    }
    if let LiteralValue::ArrayValue(arr) = &args[0] {
        let mut arr = arr.clone();
        if arr.is_empty() {
            return LiteralValue::StringValue(
                "shift function cannot remove from an empty array".to_string(),
            );
        }
        arr.remove(0);
        LiteralValue::ArrayValue(arr)
    } else {
        LiteralValue::StringValue("shift function requires an array as the argument".to_string())
    }
}
