use super::{IntcodeComputer, Memory};

impl IntcodeComputer {
    pub fn load_hook(&mut self, hook_ptr: i128) {
        if hook_ptr == 0
            || self.memory.get(Memory::JUMPBACK_PTR) != Memory::JUMPBACK_PTR_UNSET_SENTINEL
        {
            return;
        }

        self.memory
            .set(Memory::JUMPBACK_PTR, self.instruction_pointer);
        self.instruction_pointer = hook_ptr;
    }

    pub fn unload_hook(&mut self) {
        match self.memory.get(Memory::JUMPBACK_PTR) {
            Memory::JUMPBACK_PTR_UNSET_SENTINEL => {
                panic!("Tried to unload hook, but no hook is loaded!");
            }
            p => {
                self.instruction_pointer = p;
                self.memory
                    .set(Memory::JUMPBACK_PTR, Memory::JUMPBACK_PTR_UNSET_SENTINEL);
            }
        }
    }

    pub fn hook_loaded(&self) -> bool {
        self.memory.get(Memory::JUMPBACK_PTR) != Memory::JUMPBACK_PTR_UNSET_SENTINEL
    }

    pub fn exit_hook_should_run(&self) -> bool {
        self.memory.get(Memory::EXIT_HOOK_PTR) != 0 && self.memory.get(Memory::EXIT_HOOK_RUN) == 0
    }
    pub fn mark_exit_hook_run(&mut self) {
        self.memory.set(Memory::EXIT_HOOK_RUN, 1);
    }

    pub fn handle_halt_hooks(&mut self) {
        if self.hook_loaded() {
            self.unload_hook();
        } else if self.exit_hook_should_run() {
            self.load_hook(self.memory.get(Memory::EXIT_HOOK_PTR));
            self.mark_exit_hook_run();
        }
    }
}
