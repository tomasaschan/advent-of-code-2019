use super::{memory::Memory, IntcodeComputer};
use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

impl IntcodeComputer {
    pub fn new(
        program: &Vec<i128>,
        init_hook: Option<Vec<i128>>,
        exit_hook: Option<Vec<i128>>,
        input_hook: Option<Vec<i128>>,
        input: Receiver<i128>,
        output: Sender<i128>,
        input_timeout: Option<Duration>,
    ) -> IntcodeComputer {
        let mut computer = IntcodeComputer {
            program: program.clone(),
            init_hook,
            exit_hook,
            input_hook,
            memory: Memory::new(),
            instruction_pointer: 0,
            input,
            output,
            input_timeout,
        };
        computer.reset();
        computer
    }

    pub fn reset(&mut self) {
        self.memory = Memory::build(
            &self.program,
            &self.init_hook,
            &self.exit_hook,
            &self.input_hook,
        );
        self.instruction_pointer = 0;
    }
}
