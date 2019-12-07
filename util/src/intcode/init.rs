use super::IntcodeComputer;

impl IntcodeComputer {
  pub fn parse_program(input: String) -> Vec<i32> {
    input
      .split(",")
      .filter(|s| s.len() > 0)
      .map(|s| {
        s.parse::<i32>()
          .expect(&format!("invalid instruction: '{}'", s))
      })
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

  pub fn reset(&mut self) {
    self.memory = self.initial_memory.clone();
    self.instruction_pointer = 0;
    self.output = Vec::new();
  }
}
