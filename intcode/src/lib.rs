use memory::Memory;
use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

pub struct IntcodeComputer {
    program: Vec<i128>,
    init_hook: Option<Vec<i128>>,
    exit_hook: Option<Vec<i128>>,
    input_hook: Option<Vec<i128>>,
    memory: Memory,
    input: Receiver<i128>,
    output: Sender<i128>,
    instruction_pointer: i128,
    input_timeout: Option<Duration>,
}

pub struct Builder {
    _program: Option<Vec<i128>>,
    _init_hook: Option<Vec<i128>>,
    _exit_hook: Option<Vec<i128>>,
    _input_hook: Option<Vec<i128>>,
    _input_timeout: Option<Duration>,
}

pub mod builder;
pub mod debug;
pub mod hooks;
pub mod init;
pub mod memory;
pub mod ops;
pub mod parser;
pub mod run;
