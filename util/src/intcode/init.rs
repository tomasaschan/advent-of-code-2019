use super::IntcodeComputer;

impl IntcodeComputer {
  pub fn parse_program(input: String) -> Vec<i32> {
    input
      .split(",")
      .map(|s| s.parse::<i32>().expect("invalid input"))
      .collect()
  }

  pub fn parse(input: String) -> IntcodeComputer {
    IntcodeComputer::new(IntcodeComputer::parse_program(input))
  }

  pub fn new(program: Vec<i32>) -> IntcodeComputer {
    IntcodeComputer {
      initial_memory: program.clone(),
      memory: program,
      instruction_pointer: 0,
      output: Vec::new(),
      debug_mode: false,
    }
  }
}
