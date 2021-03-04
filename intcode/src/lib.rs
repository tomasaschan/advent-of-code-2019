use memory::Memory;
use ops::read::EmptyInputBehavior;
use std::sync::mpsc::{Receiver, Sender};

pub struct IntcodeComputer {
    program: Vec<i128>,
    init_hook: Option<Vec<i128>>,
    exit_hook: Option<Vec<i128>>,
    input_hook: Option<Vec<i128>>,
    memory: Memory,
    input: Receiver<i128>,
    output: Sender<i128>,
    instruction_pointer: i128,
    empty_input_behavior: EmptyInputBehavior,
    silent: bool,
}

pub struct Builder {
    _program: Option<Vec<i128>>,
    _init_hook: Option<Vec<i128>>,
    _exit_hook: Option<Vec<i128>>,
    _input_hook: Option<Vec<i128>>,
    _initial_input: Vec<i128>,
    _silent: bool,
    _empty_input_behavior: EmptyInputBehavior,
}

pub struct AsciiBuilder {
    _builder: Builder,
}

pub mod ascii;
pub mod builder;
pub mod debug;
pub mod hooks;
pub mod init;
pub mod memory;
pub mod ops;
pub mod parser;
pub mod run;
