use std::process::exit;

pub struct TronError {
    pub code: String,
    pub line: usize,
    pub message: String,
}

impl TronError {
    pub fn throw(error_code: &str, line: usize, args: Vec<String>) {
        let message: String = match args.len() {
            0 => match error_code {
                "E0001" => "failed to run file".to_string(),
                "E0002" => "failed to run command".to_string(),
                "E0003" => "unsupported platform".to_string(),
                "E1001" => "unterminated string".to_string(),
                "E2002" => "failed to parse block statement".to_string(),
                "E2004" => "function can't have more than 32 arguments".to_string(),
                "E2005" => "invalid assigment target".to_string(),
                "E3003" => "failed to read local variable".to_string(),
                "E3004" => "failed to resolve a variable in a too deep level".to_string(),
                "E3005" => "failed to define a variable in a too deep level".to_string(),
                "E3006" => "return isn't allowed outside of a function".to_string(),
                "E3007" => "break isn't allowed outside of a loop".to_string(),
                "E4006" => "failed to make function".to_string(),
                "E4009" => "array index is out of bounds".to_string(),
                "E4010" => "failed to perform operation on array".to_string(),
                "E4012" => "immutable variables can't be re-declared".to_string(),
                "E4013" => "failed to call".to_string(),
                "E4014" => "function call argument count doesn't match parameter count".to_string(),
                "E4017" => "invalid function output type".to_string(),
                _ => "uknwon error".to_string(),
            },
            1 => {
                let e1002 = format!("unrecognized character: {}", args[0]);
                let e1003 = format!("unsupported character: {}", args[0]);
                let e1004 = format!("failted to scan tokens: \n {}", args[0]);
                let e2001 = format!("failed to parse statements: \n {}", args[0]);
                let e2003 = format!("unexpected token:  {}", args[0]);
                let e3001 = format!("failed to resolve {} statement: incorrect type", args[0]);
                let e3002 = format!("variable {} already exists", args[0]);
                let e4004 = format!("failed to execute command: \n {}", args[0]);
                let e4005 = format!("failed to find library: {}", args[0]);
                let e4008 = format!("failed to create type from {}", args[0]);
                let e4011 = format!("variable {} has not been declared", args[0]);
                let e4016 = format!("invalid operator {}", args[0]);

                match error_code {
                    "E1002" => e1002,
                    "E1003" => e1003,
                    "E1004" => e1004,
                    "E2001" => e2001,
                    "E2003" => e2003,
                    "E3001" => e3001,
                    "E3002" => e3002,
                    "E4004" => e4004,
                    "E4005" => e4005,
                    "E4008" => e4008,
                    "E4011" => e4011,
                    "E4016" => e4016,
                    _ => "uknwon error".to_string(),
                }
            }
            2 => {
                let e4007 = format!("failed to unwrap {} as {}", args[0], args[1]);
                let e4015 = format!("{} is not implemented for {}", args[0], args[1]);
                let e4018 = format!("{} requires at least {} arguments", args[0], args[1]);
                let e4019 = format!("{} requires more than {} arguments", args[0], args[1]);
                let e4020 = format!("{} exactly exactly {} arguments", args[0], args[1]);
                match error_code {
                    "E4007" => e4007,
                    "E4015" => e4015,
                    "E4018" => e4018,
                    "E4019" => e4019,
                    "E4020" => e4020,
                    _ => "uknwon error".to_string(),
                }
            }
            3 => {
                let e4001 = format!(
                    "{}() is expecting {} arguments, but got{}",
                    args[0], args[1], args[2]
                );
                let e4002 = format!("{}({}: {})", args[0], args[1], args[2]);
                let e4018 = format!(
                    "{} expects {} type as {} argument",
                    args[0], args[1], args[2]
                );

                match error_code {
                    "E4001" => e4001,
                    "E4002" => e4002,
                    "E4018" => e4018,
                    _ => "uknwon error".to_string(),
                }
            }
            _ => {
                let e4003 = format!(
                    "{} {} is expecting {} type, but got {}",
                    args[0], args[1], args[2], args[3]
                );

                if error_code == "E4003" {
                    e4003
                } else {
                    "uknwon error".to_string()
                }
            }
        };

        if line == 0 {
            eprintln!(
                "[\x1B[91;1m{}\x1B[0m] \x1B[91;1m{} \x1B[0m",
                error_code, message
            );
        } else {
            eprintln!(
                "[\x1B[91;1m{}\x1B[0m] \x1B[91;1m{} \x1B[0m(\x1B[96mline {}\x1B[0m)",
                error_code, message, line
            );
        }

        match error_code {
            "E1001" => exit(1),
            "E4018" => exit(1),
            "E4019" => exit(1),
            "E4020" => exit(1),
            "E4021" => exit(1),
            _ => {}
        }
    }
}
