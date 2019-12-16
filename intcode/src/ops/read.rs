use super::{
    super::{IntcodeComputer, Memory},
    param_mode,
};
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

impl IntcodeComputer {
    pub fn read(&mut self, mode: i128) {
        if self.memory.get(Memory::LAST_INPUT_INSTR) != self.instruction_pointer {
            self.memory
                .set(Memory::LAST_INPUT_INSTR, self.instruction_pointer);
            self.load_hook(self.memory.get(Memory::INPUT_HOOK_PTR));
            return;
        }

        match self.input.recv_timeout(Duration::from_secs(3)) {
            Ok(value) => {
                self.set_at_offset(1, value, param_mode(mode, 0));
                self.memory.set(
                    Memory::LAST_INPUT_INSTR,
                    Memory::JUMPBACK_PTR_UNSET_SENTINEL,
                );
                self.step_pointer(2);
            }
            Err(RecvTimeoutError::Timeout) => {
                println!(
                    "Waited for input for 3 seconds without receiving. Did you forget to send?"
                );
                return;
            }
            Err(e) => panic!("Failed to receive input: {:?}", e),
        };
    }
}
