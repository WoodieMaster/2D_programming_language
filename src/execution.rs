extern crate core;

use core::fmt;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use def::flags;
use def::flags::FlagType;
use def::instructions as inst;

use crate::definitions as def;

type Stack = Vec<i32>;

pub struct CodeGrid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize
}

impl CodeGrid {
    fn get(&self, position: Vec2) -> char {
        return self.grid[position.y as usize][position.x as usize];
    }
}

impl FromStr for CodeGrid {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<char>> = code.split('\n').map(
            |l| l.chars().filter(
                |c| !def::IGNORED_CHARS.contains(c.clone())
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

#[derive(Clone, PartialEq, Eq)]
enum IPUpdateResult {
    Normal,
    DeleteSelf,
    AddIP(InstructionPointer),
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum IPMode {
    Normal, String, StringEscape
}

impl Display for IPMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            IPMode::Normal => "normal",
            IPMode::String => "string",
            IPMode::StringEscape => "string_escape"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vec2 {
    x: isize,
    y: isize
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{} ; {}]",self.x, self.y))
    }
}

impl Vec2 {
    fn add(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct InstructionPointer {
    position: Vec2,
    movement: Vec2,
    mode: IPMode,
    flags: FlagType
}

impl Display for InstructionPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{{\n\tpos: {}\n\tmovement: {}\n\tmode: {}\n\tflags: {}\n}}", self.position, self.movement, self.mode, self.flags))
    }
}

impl InstructionPointer {
    fn default() -> Self {
        return Self {
            position: Vec2 {x: 0, y: 0},
            movement: Vec2 {x: 0, y: 1},
            mode: IPMode::Normal,
            flags: 0
        }
    }

    fn new(movement: Vec2, position: Vec2) -> Self {
        return Self {
            movement, position,
            mode: IPMode::Normal,
            flags: 0
        }
    }

    fn load_flag(&mut self, code_grid: &CodeGrid) -> bool {
        self.update_pos(code_grid);

        return match code_grid.get(self.position) {
            flags::AC_EMPTY_STACK => self.flags & flags::EMPTY_STACK != 0,
            _ => false
        }
    }

    fn update_pos(&mut self, code_grid: &CodeGrid) {
        self.position.add(self.movement);

        self.position.x %= code_grid.width as isize;
        if self.position.x < 0 {
            self.position.x += code_grid.width as isize
        }

        self.position.y %= code_grid.height as isize;
        if self.position.y < 0 {
            self.position.y += code_grid.height as isize
        }
    }

    fn execute_instruction(&mut self, instruction: char, code_grid: &CodeGrid, stack: &mut Stack) -> IPUpdateResult {
        let mut new_flags: FlagType = 0;
        match instruction {
            inst::MOVE_UP => self.movement = Vec2 {x: 0, y: -1},
            inst::MOVE_DOWN => self.movement = Vec2 {x: 0, y: 1},
            inst::MOVE_LEFT => self.movement = Vec2 {x: -1, y: 0},
            inst::MOVE_RIGHT => self.movement = Vec2 {x: 1, y: 0},
            inst::REMOVE_IP => return IPUpdateResult::DeleteSelf,
            inst::SPLIT_IP_HORIZONTAL => {
                self.movement = Vec2 {x: 1, y: 0};

                let mut new_ip = InstructionPointer::new(
                    Vec2 {x: -1, y: 0},
                    self.position
                );

                new_ip.update_pos(code_grid);

                self.update_pos(code_grid);
                return IPUpdateResult::AddIP(new_ip);
            },
            inst::SPLIT_IP_VERTICAL => {
                self.movement = Vec2 {x: 0, y: 1};

                let mut new_ip = InstructionPointer::new(
                    Vec2 {x: 0, y: 1},
                    self.position
                );

                new_ip.position.add(new_ip.movement);
                self.position.add(self.movement);
                return IPUpdateResult::AddIP(new_ip);
            },
            inst::PRINT_STACK_CHAR => {
                match stack.pop() {
                    None => { new_flags |= flags::EMPTY_STACK }
                    Some(num) => print!("{}", char::from_u32(num as u32).unwrap_or('�'))
                }
            },
            inst::TOGGLE_STRING_MODE => self.mode = IPMode::String,
            inst::FLAG_BRANCH => {
                if !self.load_flag(code_grid) {
                    self.update_pos(code_grid);
                }
            }
            _ => {}
        }
        self.flags = new_flags;
        return IPUpdateResult::Normal;
    }

    fn update(&mut self, code_grid: &CodeGrid, stack: &mut Stack) -> IPUpdateResult {
        let current_char = code_grid.get(self.position);

        match self.mode {
            IPMode::Normal => match self.execute_instruction(current_char, code_grid, stack) {
                IPUpdateResult::Normal => {}
                other => return other
            },
            IPMode::String => {
                match current_char {
                    inst::TOGGLE_STRING_MODE => self.mode = IPMode::Normal,
                    inst::STRING_ESCAPE => self.mode = IPMode::StringEscape,
                    other => stack.push(other as i32)
                }
            },
            IPMode::StringEscape => {
                self.mode = IPMode::String;
                match current_char {
                    inst::TOGGLE_STRING_MODE => stack.push(inst::TOGGLE_STRING_MODE as i32),
                    inst::STRING_ESCAPE => stack.push(inst::STRING_ESCAPE as i32),
                    _ => match self.execute_instruction(current_char, code_grid, stack) {
                        IPUpdateResult::Normal => {}
                        other => return other
                    }
                }
            }
        }

        self.update_pos(code_grid);

        return IPUpdateResult::Normal
    }
}

pub struct ExecutionContext {
    code_grid: CodeGrid,
    instruction_pointers: Vec<InstructionPointer>,
    stack: Stack
}

impl ExecutionContext {
    pub fn new(code: &str) -> Self {
        return Self {
            code_grid: CodeGrid::from_str(code).unwrap(),
            instruction_pointers: vec![InstructionPointer::default()],
            stack: Vec::new()
        }
    }

    pub fn run(&mut self, simulation_time: u64, debug: bool) {
        let start_time = std::time::SystemTime::now();

        while self.instruction_pointers.len() != 0 {
            if simulation_time > 0 {
                sleep(Duration::from_millis(simulation_time));

                let text = self.to_string();

                if let Err(e) = clearscreen::clear() {
                    eprintln!("{e}");
                    return;
                }

                println!("{text}\nstack:{:?}\n\nips:",self.stack);

                for ip in &self.instruction_pointers {
                    println!("{}",ip);
                }
            }

            let mut results: Vec<IPUpdateResult> = Vec::new();

            results.reserve_exact(self.instruction_pointers.len());

            for ip in &mut self.instruction_pointers {
                results.push(ip.update(&self.code_grid, &mut self.stack))
            }

            let mut ip_idx = 0;

            for result in results {
                match result {
                    IPUpdateResult::Normal => {}
                    IPUpdateResult::DeleteSelf => {
                        self.instruction_pointers.remove(ip_idx);
                        continue
                    }
                    IPUpdateResult::AddIP(ip) => self.instruction_pointers.push(ip)
                }
                ip_idx += 1;
            }
        }

        if debug {
            let end_time = std::time::SystemTime::now();

            match end_time.duration_since(start_time) {
                Ok(dur) => println!("\nProgram finished with exit code 0 in {:?}", dur),
                Err(_) => println!("\nProgram finished with exit code 0")
            }

            println!("Stack dump: {:?}", self.stack);
        }
    }
}

impl Display for ExecutionContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut grid_str = String::new();

        let mut ip_positions:HashSet<Vec2> = HashSet::new();

        for ip in &self.instruction_pointers {
            ip_positions.insert(ip.position);
        }

        let mut pos = Vec2 {x: 0, y: 0};
        for line in &self.code_grid.grid {
            pos.x = 0;
            for _char in line {
                let char = _char.clone();

                if ip_positions.contains(&pos) {
                    if char == inst::REMOVE_IP { grid_str.push_str("\x1b[41m"); }
                    else { grid_str.push_str("\x1b[46m"); }

                    grid_str.push(char);
                    grid_str.push_str("\x1b[0m");
                }else { grid_str.push(char); }

                pos.x += 1;
            }
            pos.y += 1;
            grid_str.push('\n');
        }

        return f.write_str(&grid_str);
    }
}