use crate::run_file;
use std::{path::PathBuf, process::exit};

pub fn cli_run(command: &str, path: PathBuf) {
    let path_buf = path.join(command);
    let input = path_buf.to_str();
    match input {
        Some(input) => match run_file(input) {
            Ok(_) => exit(0),
            Err(_msg) => {
                exit(1);
            }
        },
        None => {
            println!("Error: Non-Unicode file path");
            exit(1);
        }
    }
}
