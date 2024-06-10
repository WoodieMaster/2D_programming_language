extern crate core;

pub const IGNORED_CHARS: &'static str = "\r";

pub mod instructions {
    pub const MOVE_UP: char = '^';
    pub const MOVE_LEFT: char = '<';
    pub const MOVE_RIGHT: char = '>';
    pub const MOVE_DOWN: char = 'v';
    pub const REMOVE_IP: char = '#';
    pub const SPLIT_IP_HORIZONTAL: char = '_';
    pub const SPLIT_IP_VERTICAL: char = '|';
}