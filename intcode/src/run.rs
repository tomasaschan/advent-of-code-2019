use super::IntcodeComputer;

fn hook_len(hook: Option<&Vec<i128>>) -> Option<i128> {
    hook.as_ref().map(|h| h.len() as i128)
}

impl IntcodeComputer {
    pub fn run(&mut self) {
        if self.init_hook.is_some() {
            self.instruction_pointer = -1
                - hook_len(self.exit_hook.as_ref()).unwrap_or(0)
                - hook_len(self.init_hook.as_ref()).unwrap();
            self.run_to_halt();
        }

        self.instruction_pointer = 0;
        self.run_to_halt();

        if self.exit_hook.is_some() {
            self.instruction_pointer = -1 - hook_len(self.exit_hook.as_ref()).unwrap();
            self.run_to_halt();
        }
    }

    fn run_to_halt(&mut self) {
        while self
            .memory
            .get(&self.instruction_pointer)
            .expect("Instruction pointer outside of memory!")
            != &99
        {
            self.step();
        }
    }
}
