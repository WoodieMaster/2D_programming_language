extern crate core;

use core::fmt;
use std::env;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter, Pointer, Write};
use std::fs;
use std::path::Path;
use crate::IPUpdateResult::Normal;

type CodeGrid = Vec<Vec<char>>;
fn grid_string(grid: &CodeGrid) -> String {
    let mut result = String::new();

    for line in grid {
        for char in line {
            result.push(char.clone());
        }
        result.push('\n');
    }

    return result;
}

enum IPUpdateResult {
    Normal,
    DeleteSelf,
    AddIP(InstructionPointer),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: usize,
    y: usize
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{} ; {}]",self.x, self.y))
    }
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        return Self { x, y }
    }

    fn add(&self, other: Vec2) -> Self {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct InstructionPointer {
    position: Vec2,
    movement: Vec2
}

impl InstructionPointer {
    fn default() -> Self {
        return Self {
            position: Vec2 {x: 0, y: 0},
            movement: Vec2 {x: 0, y: 1}
        }
    }

    fn new(position: Vec2, movement: Vec2) -> Self {
        return Self { position, movement }
    }

    fn update(&mut self) -> IPUpdateResult {

        return Normal;
    }
}

struct ExecutionContext {
    code_grid: CodeGrid,
    instruction_pointers: Vec<InstructionPointer>
}

impl ExecutionContext {
    fn new(code_grid: CodeGrid) -> Self {
        return Self {
            code_grid,
            instruction_pointers: vec![InstructionPointer::default()]
        }
    }

    fn run(&mut self) {
        loop {
            let mut results: Vec<IPUpdateResult> = Vec::new();

            results.reserve_exact(self.instruction_pointers.len());

            for ip in &mut self.instruction_pointers {
                results.push(ip.update())
            }

            let mut ip_idx = 0;

            for result in results {
                match result {
                    Normal => {}
                    IPUpdateResult::DeleteSelf => {
                        self.instruction_pointers.remove(ip_idx);
                        continue
                    }
                    IPUpdateResult::AddIP(ip) => self.instruction_pointers.push(ip)
                }
                ip_idx += 1;
            }

            if self.instruction_pointers.len() == 0 {
                return;
            }
        }
    }
}

const IGNORED_CHARS: &'static str = "\r";

fn load_code_grid_from_file(filepath: &str) -> Result<CodeGrid, String> {
    let code = match fs::read_to_string(Path::new(&OsString::from(filepath))) {
        Err(e) => {
            return Err(format!("Cannot load file: {e}"));
        }
        Ok(val) => val
    };

    let mut code_grid: CodeGrid = code.split('\n').map(
        |l| l.chars().filter(
            |c| !IGNORED_CHARS.contains(c.clone())
        ).collect()).collect();

    let mut line_length = 0;

    for line in &code_grid {
        if line.len() > line_length {
            line_length = line.len()
        }
    }

    for line in &mut code_grid {
        line.reserve_exact(line_length-line.len());
        for _ in 0..line_length-line.len() {
            line.push(' ');
        }
    }

    return Ok(code_grid);
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

    match load_code_grid_from_file(&pos_args[0]) {
        Ok(val) => println!("{}", grid_string(&val)),
        Err(err) => eprintln!("{err}")
    }
}