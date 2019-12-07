use super::IntcodeComputer;

impl IntcodeComputer {
    pub fn run(&mut self) {
        if self.init_hook.is_some() {
            self.instruction_pointer = self.program.len();
            self.run_to_halt();
        }

        self.instruction_pointer = 0;
        self.run_to_halt();

        if self.exit_hook.is_some() {
            self.instruction_pointer =
                self.program.len() + self.init_hook.as_ref().map(|hook| hook.len()).unwrap_or(0);
            self.run_to_halt();
        }
    }

    fn run_to_halt(&mut self) {
        while self.memory[self.instruction_pointer] != 99 {
            self.step();
        }
    }
}
