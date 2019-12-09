use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

pub struct IntcodeComputer {
    program: Vec<i128>,
    init_hook: Option<Vec<i128>>,
    exit_hook: Option<Vec<i128>>,
    memory: HashMap<i128, i128>,
    input: Receiver<i128>,
    output: Sender<i128>,
    instruction_pointer: i128,
}

pub struct Builder {
    _program: Option<Vec<i128>>,
    _init_hook: Option<Vec<i128>>,
    _exit_hook: Option<Vec<i128>>,
}

pub mod builder;
pub mod init;
pub mod ops;
pub mod run;
