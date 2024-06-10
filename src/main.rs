mod execution;

extern crate core;

use std::env;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Pointer, Write};
use std::fs;

use std::path::Path;
use std::str::FromStr;
use crate::execution::ExecutionContext;

fn load_file(filepath: &str) -> Result<String, String> {
    match fs::read_to_string(Path::new(&OsString::from(filepath))) {
        Err(e) => {
            return Err(format!("Cannot load file: {e}"));
        },
        Ok(val) => Ok(val)
    }
}

fn main() {
    let mut key_args: Vec<String> = Vec::new();
    let mut pos_args: Vec<String> = Vec::new();

    for arg in env::args().skip(1) {
        if arg.starts_with('-') {
            if arg.starts_with("--") {
                key_args.push(arg[2..arg.len()].to_string())
            }else {
                for char in arg[1..arg.len()].chars() {
                    key_args.push(char.to_string());
                }
            }
        }else {
            pos_args.push(arg);
        }
    }

    if pos_args.len() == 0 {
        eprintln!("No file specified");
        return;
    }

    match load_file(&pos_args[0]) {
        Ok(val) => {
            let mut ec = ExecutionContext::new(&val);

            ec.run(true)
        },
        Err(err) => eprintln!("{err}")
    }
}