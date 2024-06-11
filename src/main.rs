extern crate core;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::io;
use std::io::Write;

use crate::execution::{DebugLevel, ExecutionContext};

mod execution;
mod definitions;

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

    let mut debug_level = DebugLevel::NONE;
    let mut simulation_time = 0u64;

    let mut args = env::args();

    args.next();

    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            match &arg[2..arg.len()] {
                "debug" => match args.next() {
                    None => {
                        eprintln!("No debug mode specified");
                        return;
                    }
                    Some(val) => {
                        match val.parse::<DebugLevel>() {
                            Ok(lvl) => debug_level = lvl,
                            Err(_) => {
                                eprintln!("Invalid debug mode");
                                return;
                            }
                        }
                    }
                },
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

    let name: String = if pos_args.len() == 0 || !pos_args[0].contains('.') {
        let mut input = String::new();

        if pos_args.len() == 0 { println!("No file specified!") }
        else {
            println!("Incomplete filename '{}'", &pos_args[0]);
            input.push_str(&pos_args[0])
        }

        print!("Enter filename: ");
        let _ = io::stdout().flush();

        let stdin = io::stdin();


        if let Err(_) = stdin.read_line(&mut input) {
            eprintln!("Could not read input");
            return;
        }

        input.trim().to_string()
    }else { pos_args[0].clone() };



    match load_file(&name) {
        Ok(val) => {
            let mut ec = ExecutionContext::new(&val);

            ec.run(simulation_time, debug_level);
        },
        Err(err) => eprintln!("{err}")
    }
}