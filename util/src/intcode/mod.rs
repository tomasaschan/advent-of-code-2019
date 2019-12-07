#[derive(Clone)]
pub struct IntcodeComputer {
  initial_memory: Vec<i32>,
  memory: Vec<i32>,
  instruction_pointer: usize,
  pub output: Vec<i32>,
  pub debug_mode: bool,
}

pub mod init;
pub mod ops;
