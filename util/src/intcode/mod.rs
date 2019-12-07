#[derive(Clone)]
pub struct IntcodeComputer {
  initial_memory: Vec<i32>,
  memory: Vec<i32>,
  instruction_pointer: usize,
}

pub mod init;
pub mod ops;
pub mod run;
