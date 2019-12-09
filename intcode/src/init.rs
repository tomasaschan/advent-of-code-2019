use super::IntcodeComputer;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::mpsc::{Receiver, Sender};

impl IntcodeComputer {
    fn build_memory(
        program: &Vec<i128>,
        init_hook: &Option<Vec<i128>>,
        exit_hook: &Option<Vec<i128>>,
    ) -> HashMap<i128, i128> {
        match (init_hook, exit_hook) {
            (Some(init), Some(exit)) => HashMap::from_iter(
                [&init[..], &exit[..], &[0], &program[..]]
                    .concat()
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        (
                            (i as i128 - init.len() as i128 - exit.len() as i128 - 1),
                            *x,
                        )
                    }),
            ),
            (Some(init), None) => HashMap::from_iter(
                [&init[..], &[0], &program[..]]
                    .concat()
                    .iter()
                    .enumerate()
                    .map(|(i, x)| ((i as i128 - init.len() as i128 - 1), *x)),
            ),
            (None, Some(exit)) => HashMap::from_iter(
                [&exit[..], &[0], &program[..]]
                    .concat()
                    .iter()
                    .enumerate()
                    .map(|(i, x)| ((i as i128 - exit.len() as i128 - 1), *x)),
            ),
            (None, None) => HashMap::from_iter(
                [&[0], &program[..]]
                    .concat()
                    .iter()
                    .enumerate()
                    .map(|(i, x)| ((i as i128 - 1), *x)),
            ),
        }
    }

    pub fn new(
        program: &Vec<i128>,
        init_hook: Option<Vec<i128>>,
        exit_hook: Option<Vec<i128>>,
        input: Receiver<i128>,
        output: Sender<i128>,
    ) -> IntcodeComputer {
        let mut computer = IntcodeComputer {
            program: program.clone(),
            init_hook,
            exit_hook,
            memory: HashMap::new(),
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
