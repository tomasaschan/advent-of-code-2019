use super::{Builder, IntcodeComputer};

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

impl Builder {
    pub fn new() -> Builder {
        Builder {
            _program: None,
            _init_hook: None,
            _exit_hook: None,
        }
    }

    pub fn program(&mut self, program: &Vec<i128>) -> &mut Self {
        self._program = Some(program.clone());
        self
    }

    pub fn parse(&mut self, input: &String) -> &mut Self {
        self.program(
            &input
                .split(",")
                .filter(|s| s.len() > 0)
                .map(|s| {
                    s.parse::<i128>()
                        .expect(&format!("invalid instruction: '{}'", s))
                })
                .collect(),
        )
    }

    pub fn init_hook(&mut self, hook: &Vec<i128>) -> &mut Self {
        self._init_hook = Some(hook.clone());
        self
    }

    pub fn exit_hook(&mut self, hook: &Vec<i128>) -> &mut Self {
        self._exit_hook = Some(hook.clone());
        self
    }

    pub fn run(&self, input: Receiver<i128>, output: Sender<i128>) {
        let mut computer = IntcodeComputer::new(
            self._program.as_ref().expect("No program set!"),
            self._init_hook.clone(),
            self._exit_hook.clone(),
            input,
            output,
        );

        thread::spawn(move || {
            computer.run();
        });
    }

    pub fn run_noninteractive(&self, input: &mut dyn Iterator<Item = i128>) -> Vec<i128> {
        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();

        self.run(in_rx, out_tx);

        for i in input {
            match in_tx.send(i) {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        let mut output = Vec::new();
        for o in out_rx.iter() {
            // println!("Got output: {}", o);
            output.push(o);
        }
        output
    }
}
