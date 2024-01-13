/*

    Tron Natives

    - This file contains functions for certain native payloads (aka standart library)

*/
use crate::environment::*;
use crate::expr::*;
use crate::panic;
use rand::Rng;
use std::process::exit;
use std::rc::Rc;
/*


    include "#math";
    standart library for math


*/
pub fn include_math_natives(environment: &mut Environment) {
    /*

        description: get sinus of the number
        usage: sin(number)

    */
    environment.define(
        "sin".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "sin".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get arcsinus of the number
        usage: asin(number)

    */
    environment.define(
        "asin".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "asin".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get cosinus of the number
        usage: cos(number)

    */
    environment.define(
        "cos".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "cos".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get arccosinus of the number
        usage: acos(number)

    */
    environment.define(
        "acos".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "acos".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get tanges of the number
        usage: tan(number)

    */
    environment.define(
        "tan".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "tan".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get arctanges of the number
        usage: atan(number)

    */
    environment.define(
        "atan".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "atan".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: round the number
        usage: round(number)

    */
    environment.define(
        "round".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "round".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: floor the number
        usage: floor(number)

    */
    environment.define(
        "floor".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "floor".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: ceil the number
        usage: ceil(number)

    */
    environment.define(
        "ceil".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "ceil".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get power of the number
        usage: pow(number, number)

    */
    environment.define(
        "pow".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "pow".to_string(),
            arity: 2,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get root of the number
        usage: root(number, number)

    */
    environment.define(
        "root".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "root".to_string(),
            arity: 2,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: generate random number
        usage: random(number, number)

    */
    environment.define(
        "random".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "random".to_string(),
            arity: 2,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {    if args.len() != 2 {
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
            panic("\n random() requires a numeric arguments");
            exit(1)
        }
    }}),
        })),
    );
    /*

        description: get smallest member in the list
        usage: min(number, number, ...number)

    */
    environment.define(
        "min".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "min".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get biggest number in the list
        usage: max(number, number, ...number)

    */
    environment.define(
        "max".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "max".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get logarithm of the number
        usage: log(number)

    */
    environment.define(
        "log".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "log".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: get logarithm of base 2
        usage: log2(number)

    */
    environment.define(
        "log2".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "log2".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

       description: get logarithm of base 10
       usage: log10(number)

    */
    environment.define(
        "log10".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "log10".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: convert radians to degrees
        usage: to_degrees(number)

    */
    environment.define(
        "to_degrees".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "to_degrees".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
    /*

        description: convert degrees to radians
        usage: to_radians(number)

    */
    environment.define(
        "to_radians".to_string(),
        LiteralValue::Callable(CallableImpl::NativeFunction(NativeFunctionImpl {
            name: "to_radians".to_string(),
            arity: 1,
            fun: Rc::new(|args: &Vec<LiteralValue>| -> LiteralValue {
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
            }),
        })),
    );
}
