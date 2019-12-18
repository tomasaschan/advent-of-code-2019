use super::{
    super::{IntcodeComputer, Memory},
    param_mode,
};
use std::sync::mpsc::{RecvError, RecvTimeoutError};
use std::time::Duration;

impl IntcodeComputer {
    pub fn read(&mut self, mode: i128) {
        if self.memory.get(Memory::LAST_INPUT_INSTR) != self.instruction_pointer {
            self.memory
                .set(Memory::LAST_INPUT_INSTR, self.instruction_pointer);
            self.load_hook(self.memory.get(Memory::INPUT_HOOK_PTR));
            return;
        }

        let value = match self.input_timeout {
            Some(timeout) => match self.input.recv_timeout(Duration::from_secs(3)) {
                Ok(value) => value,
                Err(RecvTimeoutError::Timeout) => {
                    println!(
                        "Waited for input for {} seconds without receiving. Did you forget to send?",
                        timeout.as_secs()
                    );
                    return;
                }
                Err(RecvTimeoutError::Disconnected) => {
                    println!("Intcode Computer disconnected; shutting down...");
                    self.memory.set(self.instruction_pointer, 99);
                    return;
                }
            },
            None => match self.input.recv() {
                Ok(value) => value,
                Err(RecvError {}) => {
                    println!("Intcode Computer disconnected; shutting down...");
                    self.memory.set(self.instruction_pointer, 99);
                    return;
                }
            },
        };

        self.set_at_offset(1, value, param_mode(mode, 0));
        self.memory.set(
            Memory::LAST_INPUT_INSTR,
            Memory::JUMPBACK_PTR_UNSET_SENTINEL,
        );
        self.step_pointer(2);
    }
}
