use super::*;

use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

impl AsciiBuilder {
    pub fn new() -> AsciiBuilder {
        AsciiBuilder {
            _builder: Builder::new(),
        }
    }

    pub fn program(&mut self, program: &Vec<i128>) -> &mut Self {
        self._builder.program(program);
        self
    }

    pub fn parse(&mut self, input: &String) -> &mut Self {
        self._builder.parse(input);
        self
    }

    pub fn init_hook(&mut self, hook: Vec<i128>) -> &mut Self {
        self._builder.init_hook(hook);
        self
    }

    pub fn exit_hook(&mut self, hook: Vec<i128>) -> &mut Self {
        self._builder.exit_hook(hook);
        self
    }

    pub fn input_timeout(&mut self, timeout: Duration) -> &mut Self {
        self._builder.input_timeout(timeout);
        self
    }

    pub fn block_on_input(&mut self) -> &mut Self {
        self._builder.block_on_input();
        self
    }

    pub fn input_default(&mut self, default: i128) -> &mut Self {
        self._builder.input_default(default);
        self
    }

    pub fn initial_input(&mut self, input: Vec<char>) -> &mut Self {
        self._builder
            .initial_input(input.into_iter().map(atoi).collect());
        self
    }

    pub fn silent(&mut self) -> &mut Self {
        self._builder.silent();
        self
    }

    pub fn run(&self) -> (Sender<char>, Receiver<char>) {
        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();

        let (input, output) = self._builder.run();

        thread::spawn(move || {
            for c in in_rx.iter() {
                let i = atoi(c);
                input
                    .send(i)
                    .expect(&format!("Failed to forward input {} ({})", c, i));
            }
        });

        thread::spawn(move || {
            for i in output.iter() {
                let c = itoa(i);
                out_tx
                    .send(c)
                    .expect(&format!("Failed to forward output {} ({})", i, c));
            }
        });

        (in_tx, out_rx)
    }

    pub fn run_noninteractive<I>(&self, input: &mut I) -> Vec<char>
    where
        I: Iterator<Item = char>,
    {
        return self
            ._builder
            .run_noninteractive(&mut input.map(atoi))
            .into_iter()
            .map(itoa)
            .collect();
    }
}

fn itoa(i: i128) -> char {
    std::char::from_u32(i as u32).unwrap()
}
fn atoi(c: char) -> i128 {
    c as i128
}
