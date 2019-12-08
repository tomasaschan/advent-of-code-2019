use super::IntcodeComputer;
use std::sync::mpsc::{Receiver, Sender};

impl IntcodeComputer {
    fn build_memory(
        program: &Vec<i32>,
        init_hook: &Option<Vec<i32>>,
        exit_hook: &Option<Vec<i32>>,
    ) -> Vec<i32> {
        match (init_hook, exit_hook) {
            (Some(init), Some(exit)) => [&program[..], &init[..], &exit[..]].concat(),
            (Some(init), None) => [&program[..], &init[..]].concat(),
            (None, Some(exit)) => [&program[..], &exit[..]].concat(),
            (None, None) => program.clone(),
        }
    }

    pub fn new(
        program: &Vec<i32>,
        init_hook: Option<Vec<i32>>,
        exit_hook: Option<Vec<i32>>,
        input: Receiver<i32>,
        output: Sender<i32>,
    ) -> IntcodeComputer {
        let mut computer = IntcodeComputer {
            program: program.clone(),
            init_hook,
            exit_hook,
            memory: Vec::new(),
            instruction_pointer: 0,
            input,
            output,
        };
        computer.reset();
        computer
    }

    pub fn reset(&mut self) {
        self.memory =
            IntcodeComputer::build_memory(&self.program, &self.init_hook, &self.exit_hook);
        self.instruction_pointer = 0;
    }
}
