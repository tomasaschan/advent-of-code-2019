use super::{IntcodeComputer, Memory};

impl IntcodeComputer {
    pub fn run(&mut self) {
        self.load_hook(self.memory.get(Memory::INIT_HOOK_PTR));
        self.run_to_halt();
    }

    pub fn run_to_halt(&mut self) {
        loop {
            if self.memory.get(self.instruction_pointer) == 99 {
                self.handle_halt_hooks();
                if self.memory.get(self.instruction_pointer) == 99 {
                    break;
                }
            }
            self.step();
        }
    }
}
