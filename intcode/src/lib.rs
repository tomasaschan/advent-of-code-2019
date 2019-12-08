use std::sync::mpsc::{Receiver, Sender};

pub struct IntcodeComputer {
    program: Vec<i32>,
    init_hook: Option<Vec<i32>>,
    exit_hook: Option<Vec<i32>>,
    memory: Vec<i32>,
    input: Receiver<i32>,
    output: Sender<i32>,
    instruction_pointer: usize,
}

pub struct Builder {
    _program: Option<Vec<i32>>,
    _init_hook: Option<Vec<i32>>,
    _exit_hook: Option<Vec<i32>>,
}

pub mod builder;
pub mod init;
pub mod ops;
pub mod run;
