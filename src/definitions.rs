extern crate core;

pub const IGNORED_CHARS: &'static str = "\r";

pub mod instructions {
    pub const MOVE_UP: char = '^';
    pub const MOVE_LEFT: char = '<';
    pub const MOVE_RIGHT: char = '>';
    pub const MOVE_DOWN: char = 'v';
    pub const REMOVE_IP: char = ';';
    pub const SPLIT_IP_HORIZONTAL: char = '_';
    pub const SPLIT_IP_VERTICAL: char = '|';
    pub const TOGGLE_STRING_MODE: char = '"';
    pub const PRINT_STACK_CHAR: char = '.';
    pub const STRING_ESCAPE: char = '\\';
    pub const FLAG_BRANCH: char = '?';
}

pub mod flags {
    pub type FlagType = u8;

    pub const EMPTY_STACK: FlagType = 0x1;
    pub const AC_EMPTY_STACK: char = '_';
}