#![allow(dead_code)]
use std::ffi::c_char;

pub const IS_IN_GAME: *const bool = 0x0074E35C as *const bool;

// SendCommandToConsole
pub const SEND_COMMAND_TO_CONSOLE: usize = 0x004F9AB0;
pub type SendCommandToConsole = extern "C" fn(i32, i32, cmd: *const c_char);
