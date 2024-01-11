use rand::Rng;

use crate::expr::LiteralValue;
use crate::panic;
use std::io;
use std::process::exit;

pub fn native_sin(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n sin() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().sin()),
        _ => {
            panic("\n sin() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_asin(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n asin() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().asin()),
        _ => {
            panic("\n asin() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_cos(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n cos() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().cos()),
        _ => {
            panic("\n cos() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_acos(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n acos() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().acos()),
        _ => {
            panic("\n acos() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_tan(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n tan() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().tan()),
        _ => {
            panic("\n tan() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_atan(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n atan() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().atan()),
        _ => {
            panic("\n atan() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_round(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n round() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.round()),
        _ => {
            panic("\n round() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_floor(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n floor() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.floor()),
        _ => {
            panic("\n floor() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_todgrees(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("to_degrees() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.to_degrees()),
        _ => {
            panic("\n to_degrees() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_toradians(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("to_radians() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.to_radians()),
        _ => {
            panic("\n to_radians() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_pow(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        panic("pow() takes 2 arguments");
    }
    match (&args[0], &args[1]) {
        (LiteralValue::Number(base), LiteralValue::Number(exp)) => {
            LiteralValue::Number(base.powf(*exp))
        }
        _ => {
            panic("\n pow() requires a numeric argument");
            exit(1)
        }
    }
}

pub fn native_root(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        panic("root() expects two argumentss");
    }
    match (&args[0], &args[1]) {
        (LiteralValue::Number(number), LiteralValue::Number(n)) => {
            if *n == 0.0 {
                panic("root()'s second argument (root) must not be zero");
            }
            LiteralValue::Number(number.powf(1.0 / n))
        }
        _ => {
            panic("\n root() requires a numeric argument");
            exit(1)
        }
    }
}

pub fn native_random(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        panic("random() takes 2 arguments");
    }
    match (&args[0], &args[1]) {
        (LiteralValue::Number(min), LiteralValue::Number(max)) => {
            if min > max {
                panic("random() requires the first argument to be less than or equal to the second argument");
            }
            let mut rng = rand::thread_rng();
            LiteralValue::Number(rng.gen_range(*min..*max).round())
        }
        _ => {
            panic("\n sin() requires a numeric argument");
            exit(1)
        }
    }
}

pub fn native_min(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.is_empty() {
        panic("min() requires at least one argument");
    }
    let mut min_value = match &args[0] {
        LiteralValue::Number(num) => *num,
        _ => {
            panic("\n min() requires a numeric argument");
            exit(1)
        }
    };
    for arg in args.iter().skip(1) {
        match arg {
            LiteralValue::Number(num) => {
                if num < &min_value {
                    min_value = *num;
                }
            }
            _ => {
                panic("\n min() requires a numeric argument");
                exit(1)
            }
        }
    }
    LiteralValue::Number(min_value)
}

pub fn native_max(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.is_empty() {
        panic("max() requires at least one argument");
    }
    let mut max_value = match &args[0] {
        LiteralValue::Number(num) => *num,
        _ => {
            panic("\n max() requires a numeric argument");
            exit(1)
        }
    };
    for arg in args.iter().skip(1) {
        match arg {
            LiteralValue::Number(num) => {
                if num > &max_value {
                    max_value = *num;
                }
            }
            _ => {
                panic("\n max() requires a numeric argument");
                exit(1)
            }
        }
    }
    LiteralValue::Number(max_value)
}

pub fn native_log(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("log() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(num) => {
            if *num <= 0.0 {
                panic("log() argument must be greater than 0");
            }
            LiteralValue::Number(num.ln())
        }
        _ => {
            panic("\n log() requires a numeric argument");
            exit(1)
        }
    }
}

pub fn native_log2(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("log2() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(num) => {
            if *num <= 0.0 {
                panic("log2() argument must be greater than 0");
            }
            LiteralValue::Number(num.log2())
        }
        _ => {
            panic("\n log2() requires a numeric argument");
            exit(1)
        }
    }
}

pub fn native_log10(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("log10() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(num) => {
            if *num <= 0.0 {
                panic("log10() argument must be greater than 0");
            }
            LiteralValue::Number(num.log10())
        }
        _ => {
            panic("\n log10() requires a numeric argument");
            exit(1)
        }
    }
}

pub fn native_ceil(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n ceil() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.ceil()),
        _ => {
            panic("\n ceil() requires a numeric argument");
            exit(1)
        }
    }
}

pub fn native_input(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n input() expects one argument");
    }
    match &args[0] {
        LiteralValue::StringValue(n) => {
            println!("{}", n.to_string());
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            LiteralValue::StringValue(format!("{}", input))
        }
        _ => {
            panic("\n input() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_typeof(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n typeof() expects one argument");
    }
    match &args[0] {
        LiteralValue::Number(_n) => LiteralValue::StringValue("number".to_string()),
        LiteralValue::StringValue(_n) => LiteralValue::StringValue("string".to_string()),
        LiteralValue::Nil => LiteralValue::StringValue("null".to_string()),
        LiteralValue::False => LiteralValue::StringValue("boolean".to_string()),
        LiteralValue::True => LiteralValue::StringValue("boolean".to_string()),
        LiteralValue::ArrayValue(_n) => LiteralValue::StringValue("array".to_string()),
        _ => {
            panic("\n uknown type");
            exit(1)
        }
    }
}
pub fn native_len(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n len() expects one argument");
    }
    match &args[0] {
        LiteralValue::StringValue(n) => LiteralValue::Number(n.len() as f64),
        LiteralValue::ArrayValue(n) => LiteralValue::Number(n.len() as f64),
        _ => {
            panic("\n len() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_push(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        panic("\n push() expects two argumentss");
    }
    match &args[0] {
        LiteralValue::ArrayValue(arr) => {
            let mut arr = arr.clone();
            arr.push(args[1].clone());
            LiteralValue::ArrayValue(arr)
        }
        _ => {
            panic("\n push() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_join(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        panic("\n join() expects two argumentss");
    }
    match (&args[0], &args[1]) {
        (LiteralValue::ArrayValue(arr), LiteralValue::StringValue(join_str)) => {
            let mut strings = Vec::new();
            for val in arr.iter() {
                match val {
                    LiteralValue::Number(num) => strings.push(num.to_string()),
                    LiteralValue::StringValue(s) => strings.push(s.clone()),
                    _ => panic("\n join() requires an array of strings or numbers"),
                }
            }
            let joined = strings.join(join_str);
            LiteralValue::StringValue(joined)
        }
        _ => {
            panic("\n join() requires a numeric argument");
            exit(1)
        }
    }
}
pub fn native_pop(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n pop() expects one argument");
    }
    if let LiteralValue::ArrayValue(arr) = &args[0] {
        let mut arr = arr.clone();
        arr.pop();
        LiteralValue::ArrayValue(arr)
    } else {
        panic("\n pop() requires a numeric argument");
        exit(1)
    }
}

pub fn native_shift(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic("\n shift() expects one argument");
    }
    if let LiteralValue::ArrayValue(arr) = &args[0] {
        let mut arr = arr.clone();
        if arr.is_empty() {
            panic("\n shift() cannot remove from an empty array");
            exit(1)
        }
        arr.remove(0);
        LiteralValue::ArrayValue(arr)
    } else {
        panic("\n shift() requires a numeric argument");
        exit(1)
    }
}
