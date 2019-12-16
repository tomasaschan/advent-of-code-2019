use super::parser::parse;
use super::{Builder, IntcodeComputer};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

impl Builder {
    pub fn new() -> Builder {
        Builder {
            _program: None,
            _init_hook: None,
            _exit_hook: None,
            _input_hook: None,
        }
    }

    /// Set the main program
    pub fn program(&mut self, program: &Vec<i128>) -> &mut Self {
        self._program = Some(program.clone());
        self
    }

    /// Parse an input string into the computer's main program
    pub fn parse(&mut self, input: &String) -> &mut Self {
        match parse(input) {
            Ok(program) => self.program(&program),
            Err(e) => panic!("Invalid program: {}", e),
        }
    }

    /// Provide an extra sequence of instructions to run _before_ running the main program.
    ///
    /// If present, the computer runs these instructions until the opcode is 99 (halt),
    /// then resumes running the main program. This is useful for e.g. setting specific
    /// memory slots to specific values, without modifying the program itself.
    ///
    /// For example, and init hook of 3,1,99 will make the computer read an input value
    /// and put that value in memory position 1 before running the main program.
    pub fn init_hook(&mut self, hook: Vec<i128>) -> &mut Self {
        self._init_hook = Some(Builder::end_in_99(hook));
        self
    }

    /// An extra sequence of instructions to run _after_ the main program.
    ///
    /// If present, the computer runs these instructions until the opcode is 99 (halt),
    /// after the main program itself halts. This is useful for e.g. reading specific
    /// memory slots after program execution.
    ///
    /// For example, an exit hook of 4,0,99 will post the value of memory slot 0 to the
    /// output channel after successful program execution.
    pub fn exit_hook(&mut self, hook: Vec<i128>) -> &mut Self {
        self._exit_hook = Some(Builder::end_in_99(hook));
        self
    }

    pub fn input_hook(&mut self, hook: Vec<i128>) -> &mut Self {
        self._input_hook = Some(Builder::end_in_99(hook));
        self
    }

    /// Create the IntcodeComputer with the specified parameters, and run
    /// the program in a separate thread.
    ///
    /// The only means of communication available for a running program is
    /// its input and output channels, returned by the `run` function.
    pub fn run(&self) -> (Sender<i128>, Receiver<i128>) {
        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();

        let mut computer = IntcodeComputer::new(
            self._program.as_ref().expect("No program set!"),
            self._init_hook.clone(),
            self._exit_hook.clone(),
            self._input_hook.clone(),
            in_rx,
            out_tx,
        );

        thread::spawn(move || {
            computer.run();
        });

        (in_tx, out_rx)
    }

    pub fn run_noninteractive(&self, input: &mut dyn Iterator<Item = i128>) -> Vec<i128> {
        let (in_tx, out_rx) = self.run();

        for i in input {
            match in_tx.send(i) {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        let mut output = Vec::new();
        for o in out_rx.iter() {
            output.push(o);
        }
        output
    }

    fn end_in_99(hook: Vec<i128>) -> Vec<i128> {
        let mut hook = hook.clone();
        if !hook.ends_with(&[99]) {
            hook.push(99)
        }
        hook
    }
}
