extern crate core;

use core::fmt;
use std::fmt::{Debug, Display, Formatter, Pointer, Write};

use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use IPUpdateResult::Normal;

const IGNORED_CHARS: &'static str = "\r";

pub struct CodeGrid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize
}

impl FromStr for CodeGrid {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<char>> = code.split('\n').map(
            |l| l.chars().filter(
                |c| !IGNORED_CHARS.contains(c.clone())
            ).collect()).collect();

        let mut line_length = 0;

        for line in &grid {
            if line.len() > line_length {
                line_length = line.len()
            }
        }

        for line in &mut grid {
            line.reserve_exact(line_length-line.len());
            for _ in 0..line_length-line.len() {
                line.push(' ');
            }
        }

        return Ok(Self {
            width: grid[0].len(),
            height: grid.len(),
            grid
        });
    }
}

impl Display for CodeGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in &self.grid {
            for char in line {
                f.write_char(char.clone())?;
            }
            f.write_char('\n')?;
        }
        return Ok(());
    }
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

    fn add(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
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

    fn update(&mut self, code_grid: &CodeGrid) -> IPUpdateResult {
        let current_char = code_grid.grid[self.position.y][self.position.x];

        self.position.add(self.movement);

        if self.position.x >= code_grid.width {
            self.position.x -= code_grid.width
        }else if self.position.x < 0 {
            self.position.x += code_grid.width
        }

        if self.position.y >= code_grid.height {
            self.position.y -= code_grid.height
        }else if self.position.y < 0 {
            self.position.y += code_grid.height
        }

        return Normal
    }
}

pub struct ExecutionContext {
    code_grid: CodeGrid,
    instruction_pointers: Vec<InstructionPointer>
}

impl ExecutionContext {
    pub fn new(code: &str) -> Self {
        return Self {
            code_grid: CodeGrid::from_str(code).unwrap(),
            instruction_pointers: vec![InstructionPointer::default()]
        }
    }

    pub fn run(&mut self, mut simulation: bool) {
        while self.instruction_pointers.len() != 0 {
            if simulation {
                sleep(Duration::from_millis(100));
                if let Err(e) = clearscreen::clear() {
                    eprintln!("{e}");
                    simulation = false;
                }else {
                    println!("{self}");
                }
            }

            let mut results: Vec<IPUpdateResult> = Vec::new();

            results.reserve_exact(self.instruction_pointers.len());

            for ip in &mut self.instruction_pointers {
                results.push(ip.update(&self.code_grid))
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
        }

        println!("Program exited with code 0");
    }
}

impl Display for ExecutionContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut grid = self.code_grid.to_string();

        for ip in &self.instruction_pointers {
            let str_idx = ip.position.y * (self.code_grid.width+1) + ip.position.x;
            grid.replace_range(str_idx..=str_idx, "\x1b[32mO\x1b[0m");
        }

        return f.write_str(&grid);
    }
}