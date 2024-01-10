// use rand::Rng;

use crate::expr::LiteralValue;
use std::io;

pub fn native_sin(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n sin function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().sin()),
        _ => panic!("\n sin function requires a numeric argument"),
    }
}
pub fn native_asin(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n asin function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().asin()),
        _ => panic!("\n asin function requires a numeric argument"),
    }
}
pub fn native_cos(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n cos function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().cos()),
        _ => panic!("\n cos function requires a numeric argument"),
    }
}
pub fn native_acos(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n acos function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().acos()),
        _ => panic!("\n acos function requires a numeric argument"),
    }
}
pub fn native_tan(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n tan function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().tan()),
        _ => panic!("\n tan function requires a numeric argument"),
    }
}
pub fn native_atan(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n atan function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(angle) => LiteralValue::Number(angle.to_radians().atan()),
        _ => panic!("\n atan function requires a numeric argument"),
    }
}
pub fn native_round(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n round function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.round()),
        _ => panic!("\n round function requires a numeric argument"),
    }
}
pub fn native_floor(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n floor function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.floor()),
        _ => panic!("\n floor function requires a numeric argument"),
    }
}
pub fn native_todgrees(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("to_degrees function takes exactly one argument",);
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.to_degrees()),
        _ => {
            panic!("\n to_degrees function requires a numeric argument")
        }
    }
}
pub fn native_toradians(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("to_radians function takes exactly one argument",);
    }
    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.to_radians()),
        _ => {
            panic!("\n to_radians function requires a numeric argument")
        }
    }
}
// pub fn native_pow(args: &Vec<LiteralValue>) -> LiteralValue {
//     if args.len() != 2 {
//         panic!("pow function takes 2 arguments");
//     }
//     match (&args[0], &args[1]) {
//         (LiteralValue::Number(base), LiteralValue::Number(exp)) => {
//             LiteralValue::Number(base.powf(*exp))
//         }
//         _ => panic!("pow function requires numeric arguments"),
//     }
// }
// pub fn native_random(args: &Vec<LiteralValue>) -> LiteralValue {
//     if args.len() != 2 {
//         panic!("random function takes 2 arguments");
//     }
//     match (&args[0], &args[1]) {
//         (LiteralValue::Number(min), LiteralValue::Number(max)) => {
//             if min > max {
//                 panic!("random function requires the first argument to be less than or equal to the second argument");
//             }
//             let mut rng = rand::thread_rng();
//             LiteralValue::Number(rng.gen_range(*min..*max))
//         }
//         _ => panic!("random function requires numeric arguments"),
//     }
// }

// pub fn native_min(args: &Vec<LiteralValue>) -> LiteralValue {
//     if args.is_empty() {
//         panic!("min function requires at least one argument");
//     }
//     let mut min_value = match &args[0] {
//         LiteralValue::Number(num) => *num,
//         _ => panic!("min function requires numeric arguments"),
//     };
//     for arg in args.iter().skip(1) {
//         match arg {
//             LiteralValue::Number(num) => {
//                 if num < &min_value {
//                     min_value = *num;
//                 }
//             }
//             _ => panic!("min function requires numeric arguments"),
//         }
//     }
//     LiteralValue::Number(min_value)
// }

// pub fn native_max(args: &Vec<LiteralValue>) -> LiteralValue {
//     if args.is_empty() {
//         panic!("max function requires at least one argument");
//     }
//     let mut max_value = match &args[0] {
//         LiteralValue::Number(num) => *num,
//         _ => panic!("max function requires numeric arguments"),
//     };
//     for arg in args.iter().skip(1) {
//         match arg {
//             LiteralValue::Number(num) => {
//                 if num > &max_value {
//                     max_value = *num;
//                 }
//             }
//             _ => panic!("max function requires numeric arguments"),
//         }
//     }
//     LiteralValue::Number(max_value)
// }

// pub fn native_log(args: &Vec<LiteralValue>) -> LiteralValue {
//     if args.len() != 1 {
//         panic!("log function takes exactly one argument");
//     }
//     match &args[0] {
//         LiteralValue::Number(num) => {
//             if *num <= 0.0 {
//                 panic!("log function argument must be greater than 0");
//             }
//             LiteralValue::Number(num.ln())
//         }
//         _ => panic!("log function requires a numeric argument"),
//     }
// }

// pub fn native_log2(args: &Vec<LiteralValue>) -> LiteralValue {
//     if args.len() != 1 {
//         panic!("log2 function takes exactly one argument");
//     }
//     match &args[0] {
//         LiteralValue::Number(num) => {
//             if *num <= 0.0 {
//                 panic!("log2 function argument must be greater than 0");
//             }
//             LiteralValue::Number(num.log2())
//         }
//         _ => panic!("log2 function requires a numeric argument"),
//     }
// }

// pub fn native_log10(args: &Vec<LiteralValue>) -> LiteralValue {
//     if args.len() != 1 {
//         panic!("log10 function takes exactly one argument");
//     }
//     match &args[0] {
//         LiteralValue::Number(num) => {
//             if *num <= 0.0 {
//                 panic!("log10 function argument must be greater than 0");
//             }
//             LiteralValue::Number(num.log10())
//         }
//         _ => panic!("log10 function requires a numeric argument"),
//     }
// }

// pub fn native_ceil(args: &Vec<LiteralValue>) -> LiteralValue {
//     if args.len() != 1 {
//         panic!("\n ceil function takes exactly one argument");
//     }
//     match &args[0] {
//         LiteralValue::Number(n) => LiteralValue::Number(n.ceil()),
//         _ => panic!("\n ceil function requires a numeric argument"),
//     }
// }

pub fn native_input(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n input function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::StringValue(n) => {
            println!("{}", n.to_string());
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            LiteralValue::StringValue(format!("{}", input))
        }
        _ => panic!("\n input function requires a string argument"),
    }
}
pub fn native_typeof(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n typeof function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::Number(_n) => LiteralValue::StringValue("number".to_string()),
        LiteralValue::StringValue(_n) => LiteralValue::StringValue("string".to_string()),
        LiteralValue::Nil => LiteralValue::StringValue("null".to_string()),
        LiteralValue::False => LiteralValue::StringValue("boolean".to_string()),
        LiteralValue::True => LiteralValue::StringValue("boolean".to_string()),
        LiteralValue::ArrayValue(_n) => LiteralValue::StringValue("array".to_string()),
        _ => panic!("\n uknown type"),
    }
}
pub fn native_len(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n len function takes exactly one argument");
    }
    match &args[0] {
        LiteralValue::StringValue(n) => LiteralValue::StringValue(n.len().to_string()),
        LiteralValue::ArrayValue(n) => LiteralValue::StringValue(n.len().to_string()),
        _ => {
            panic!("\n len function requires array or string argument")
        }
    }
}
pub fn native_push(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        panic!("\n push function takes exactly two arguments");
    }
    match &args[0] {
        LiteralValue::ArrayValue(arr) => {
            let mut arr = arr.clone();
            arr.push(args[1].clone());
            LiteralValue::ArrayValue(arr)
        }
        _ => {
            panic!("\n push function requires array as first argument")
        }
    }
}
pub fn native_join(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        panic!("\n join function takes exactly two arguments");
    }
    match (&args[0], &args[1]) {
        (LiteralValue::ArrayValue(arr), LiteralValue::StringValue(join_str)) => {
            let mut strings = Vec::new();
            for val in arr.iter() {
                match val {
                    LiteralValue::Number(num) => strings.push(num.to_string()),
                    LiteralValue::StringValue(s) => strings.push(s.clone()),
                    _ => panic!("\n join function requires an array of strings or numbers"),
                }
            }
            let joined = strings.join(join_str);
            LiteralValue::StringValue(joined)
        },
        _ => panic!("\n join function requires an array as the first argument and a string as the second argument"),
    }
}
pub fn native_pop(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n pop function takes exactly one argument");
    }
    if let LiteralValue::ArrayValue(arr) = &args[0] {
        let mut arr = arr.clone();
        arr.pop();
        LiteralValue::ArrayValue(arr)
    } else {
        panic!("\n pop function requires an array as the argument")
    }
}

pub fn native_shift(args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        panic!("\n shift function takes exactly one argument");
    }
    if let LiteralValue::ArrayValue(arr) = &args[0] {
        let mut arr = arr.clone();
        if arr.is_empty() {
            panic!("\n shift function cannot remove from an empty array");
        }
        arr.remove(0);
        LiteralValue::ArrayValue(arr)
    } else {
        panic!("\n shift function requires an array as the argument")
    }
}
