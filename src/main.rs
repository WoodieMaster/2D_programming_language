mod execution;
mod definitions;

extern crate core;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
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
    let mut pos_args: Vec<String> = Vec::new();

    let mut debug = false;
    let mut simulation_time = 0u64;

    let mut args = env::args();

    args.next();

    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            match &arg[2..arg.len()] {
                "debug" => debug = true,
                "simulate" => {
                    match args.next() {
                        None => {
                            eprintln!("No simulation speed specified");
                            return;
                        }
                        Some(val) => {
                            match val.parse::<u64>() {
                                Ok(num) => simulation_time = num,
                                Err(_) => {
                                    eprintln!("Invalid value for simulation speed");
                                    return;
                                }
                            }
                        }
                    }
                },
                other => {
                    eprintln!("Unknown argument '{other}'");
                    return;
                }
            }
        }else if arg.starts_with('-') {
            for ch in arg[2..arg.len()].chars() {
                match ch {
                    _ => {
                        eprintln!("Unknown argument '{ch}'");
                        return;
                    }
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

            ec.run(simulation_time, debug);
        },
        Err(err) => eprintln!("{err}")
    }
}