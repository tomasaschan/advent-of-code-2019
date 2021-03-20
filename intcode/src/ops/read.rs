use super::{
    super::{ops::read::EmptyInputBehavior::*, IntcodeComputer, Memory},
    param_mode,
};
use std::sync::mpsc::{RecvError, RecvTimeoutError, TryRecvError};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum EmptyInputBehavior {
    Block,
    BlockWithPrompt,
    Wait(Duration),
    Default(i128),
}

impl IntcodeComputer {
    pub fn read(&mut self, mode: i128) {
        if self.memory.get(Memory::LAST_INPUT_INSTR) != self.instruction_pointer {
            self.memory
                .set(Memory::LAST_INPUT_INSTR, self.instruction_pointer);
            self.load_hook(self.memory.get(Memory::INPUT_HOOK_PTR));
            return;
        }

        if let Some(value) = match self.empty_input_behavior {
            Wait(timeout) => self.read_with_timeout(timeout),
            Default(default) => self.read_with_default(default),
            Block => self.read_and_block(),
            BlockWithPrompt => self.read_and_block_with_prompt(),
        } {
            self.set_at_offset(1, value, param_mode(mode, 0));
            self.memory.set(
                Memory::LAST_INPUT_INSTR,
                Memory::JUMPBACK_PTR_UNSET_SENTINEL,
            );
            self.step_pointer(2);
        }
    }

    fn read_and_block(&mut self) -> Option<i128> {
        match self.input.recv() {
            Ok(value) => Some(value),
            Err(RecvError {}) => {
                self.shut_down();
                return None;
            }
        }
    }

    fn read_and_block_with_prompt(&mut self) -> Option<i128> {
        match self.input.try_recv() {
            Ok(value) => Some(value),
            Err(TryRecvError::Empty) => {
                print!("> ");
                self.read_and_block()
            }
            Err(TryRecvError::Disconnected) => {
                self.shut_down();
                None
            }
        }
    }

    fn read_with_default(&mut self, default: i128) -> Option<i128> {
        match self.input.try_recv() {
            Ok(value) => Some(value),
            Err(TryRecvError::Empty) => Some(default),
            Err(TryRecvError::Disconnected) => {
                self.shut_down();
                return None;
            }
        }
    }

    fn read_with_timeout(&mut self, timeout: Duration) -> Option<i128> {
        match self.input.recv_timeout(Duration::from_secs(3)) {
            Ok(value) => Some(value),
            Err(RecvTimeoutError::Timeout) => {
                if !self.silent {
                    println!("Waited for input for {} seconds without receiving. Did you forget to send?",timeout.as_secs());
                }
                return None;
            }
            Err(RecvTimeoutError::Disconnected) => {
                self.shut_down();
                return None;
            }
        }
    }

    fn shut_down(&mut self) {
        if !self.silent {
            println!("Intcode Computer disconnected; shutting down...");
        }
        self.memory.set(self.instruction_pointer, 99);
    }
}
