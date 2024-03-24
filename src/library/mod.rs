use crate::environment::*;
use crate::expressions::*;
use crate::panic;
use crate::utils::TronError;
use std::io as std_io;
use std::process::exit;
use std::process::Command;
use std::rc::Rc;

pub fn declare_function(
    name: String,
    args: usize,
    fun: impl Fn(&Vec<TronType>) -> TronType + 'static,
    environment: &mut Environment,
) {
    environment.define(
        name.clone(),
        TronType::Callable(CallableImpl::StdFunction(StdFunctionImpl {
            name: name.clone(),
            arity: args,
            function: Rc::new(fun),
        })),
    )
}

pub fn standard_library(environment: &mut Environment) {
    declare_function(
        "@print".to_string(),
        1,
        |args: &Vec<TronType>| {
            if args.len() > 0 {
                for arg in args {
                    println!("{:?}", arg);
                }
            } else {
                println!("");
            }
            TronType::Null
        },
        environment,
    );
    declare_function(
        "@panic".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() > 0 {
                for arg in args {
                    eprintln!("\x1B[31m{:?}\x1B[0m \n", arg);
                }
            } else {
                exit(1);
            }
            TronType::Null
        },
        environment,
    );
    declare_function(
        "@shift".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                if let TronType::ArrayValue(arr) = &args[0] {
                    let mut arr = arr.clone();
                    if arr.is_empty() {
                        return TronType::ArrayValue(arr);
                    }
                    arr.remove(0);
                    TronType::ArrayValue(arr)
                } else {
                    TronError::throw(
                        "E4021",
                        0,
                        vec![
                            "@shift".to_string(),
                            "array".to_string(),
                            "first".to_string(),
                        ],
                    );
                    exit(1)
                }
            } else {
                TronError::throw("E4018", 0, vec!["@shift".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );
    declare_function(
        "@pop".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                if let TronType::ArrayValue(arr) = &args[0] {
                    let mut arr = arr.clone();
                    if arr.is_empty() {
                        return TronType::ArrayValue(arr);
                    }
                    arr.pop();
                    TronType::ArrayValue(arr)
                } else {
                    TronError::throw(
                        "E4021",
                        0,
                        vec!["@pop".to_string(), "array".to_string(), "first".to_string()],
                    );
                    exit(1)
                }
            } else {
                TronError::throw("E4018", 0, vec!["@pop".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );
    declare_function(
        "@join".to_string(),
        2,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 2 {
                match (&args[0], &args[1]) {
                    (TronType::ArrayValue(arr), TronType::StringValue(join_str)) => {
                        let mut strings = Vec::new();
                        for val in arr.iter() {
                            match val {
                                TronType::Number(num) => strings.push(num.to_string()),
                                TronType::StringValue(s) => strings.push(s.clone()),
                                _ => panic("\n join() requires an array of strings or numbers"),
                            }
                        }
                        let joined = strings.join(join_str);

                        TronType::StringValue(joined)
                    }
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@join".to_string(),
                                "array".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@join".to_string(), 2.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@push".to_string(),
        2,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 2 {
                match &args[0] {
                    TronType::ArrayValue(arr) => {
                        let mut arr = arr.clone();
                        arr.push(args[1].clone());
                        TronType::ArrayValue(arr)
                    }
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@push".to_string(),
                                "array".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@push".to_string(), 2.to_string()]);
                exit(1);
            }
        },
        environment,
    );
    declare_function(
        "@length".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::StringValue(n) => TronType::Number(n.len() as f32),
                    TronType::ArrayValue(n) => TronType::Number(n.len() as f32),
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@length".to_string(),
                                "array or string".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@length".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );
    declare_function(
        "@ask".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::StringValue(n) => {
                        println!("{}", n.to_string());
                        let mut input = String::new();
                        std_io::stdin().read_line(&mut input).unwrap();
                        TronType::StringValue(format!("{}", input))
                    }
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@ask".to_string(),
                                "string".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@ask".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@typeof".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                TronType::StringValue(args[0].to_type().to_string())
            } else {
                TronError::throw("E4018", 0, vec!["@typeof".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@if".to_string(),
        3,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 3 {
                match &args[0] {
                    TronType::True => args[1].clone(),
                    TronType::False => args[2].clone(),
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@if".to_string(),
                                "boolean".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@if".to_string(), 3.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@sleep".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::Number(time) => {
                        std::thread::sleep(std::time::Duration::from_millis(*time as u64));
                        TronType::Number(*time)
                    }
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@sleep".to_string(),
                                "number".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@sleep".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );
    declare_function(
        "@cmd".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::StringValue(command) => {
                        let output = Command::new("sh").arg("-c").arg(command).output();
                        match output {
                            Ok(output) => {
                                if output.status.success() {
                                    let stdout = String::from_utf8_lossy(&output.stdout);
                                    TronType::StringValue(stdout.to_string())
                                } else {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    TronError::throw("E4004", 0, vec![stderr.to_string()]);
                                    exit(1)
                                }
                            }
                            Err(error) => {
                                TronError::throw("E4004", 0, vec![error.to_string()]);
                                exit(1)
                            }
                        }
                    }
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@cmd".to_string(),
                                "string".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@cmd".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@sin".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::Number(angle) => TronType::Number(angle.to_radians().sin()),
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@sin".to_string(),
                                "number".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@sin".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@cos".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::Number(angle) => TronType::Number(angle.to_radians().cos()),
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@cos".to_string(),
                                "number".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@cos".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@tan".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::Number(angle) => TronType::Number(angle.to_radians().tan()),
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@tan".to_string(),
                                "number".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@tan".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@round".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::Number(angle) => TronType::Number(angle.round()),
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@round".to_string(),
                                "number".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@round".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );
    declare_function(
        "@floor".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::Number(angle) => TronType::Number(angle.floor()),
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@floor".to_string(),
                                "number".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@floor".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@ceil".to_string(),
        1,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 1 {
                match &args[0] {
                    TronType::Number(angle) => TronType::Number(angle.ceil()),
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@ceil".to_string(),
                                "number".to_string(),
                                "first".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@ceil".to_string(), 1.to_string()]);
                exit(1);
            }
        },
        environment,
    );
    declare_function(
        "@pow".to_string(),
        2,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 2 {
                match (&args[0], &args[1]) {
                    (TronType::Number(base), TronType::Number(exp)) => {
                        TronType::Number(base.powf(*exp))
                    }
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec!["@pow".to_string(), "number".to_string(), "both".to_string()],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@pow".to_string(), 2.to_string()]);
                exit(1);
            }
        },
        environment,
    );

    declare_function(
        "@root".to_string(),
        2,
        |args: &Vec<TronType>| -> TronType {
            if args.len() == 2 {
                match (&args[0], &args[1]) {
                    (TronType::Number(number), TronType::Number(n)) => {
                        TronType::Number(number.powf(1.0 / n))
                    }
                    _ => {
                        TronError::throw(
                            "E4021",
                            0,
                            vec![
                                "@root".to_string(),
                                "number".to_string(),
                                "both".to_string(),
                            ],
                        );
                        exit(1)
                    }
                }
            } else {
                TronError::throw("E4018", 0, vec!["@root".to_string(), 2.to_string()]);
                exit(1);
            }
        },
        environment,
    );
}
