use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use super::IntcodeComputer;

impl IntcodeComputer {
  pub fn run(program: &Vec<i32>, input: Receiver<i32>, output: Sender<i32>) {
    IntcodeComputer::run_with_extras(program, &vec![], input, output);
  }

  pub fn run_noninteractive(program: &Vec<i32>, input: &mut dyn Iterator<Item = i32>) -> Vec<i32> {
    IntcodeComputer::run_with_extras_noninteractive(program, &vec![], input)
  }

  pub fn run_with_extras(
    program: &Vec<i32>,
    extras: &Vec<i32>,
    input: Receiver<i32>,
    output: Sender<i32>,
  ) {
    let mut computer = IntcodeComputer::new(&[&program[..], &extras[..]].concat());
    let len = if !extras.is_empty() {
      Some(program.len())
    } else {
      None
    };
    thread::spawn(move || {
      while computer.memory[computer.instruction_pointer] != 99 {
        computer.step(&input, &output);
      }
      if let Some(l) = len {
        computer.set_pointer(l);
        while computer.memory[computer.instruction_pointer] != 99 {
          computer.step(&input, &output);
        }
      }
    });
  }

  pub fn run_with_extras_noninteractive(
    program: &Vec<i32>,
    extras: &Vec<i32>,
    input: &mut dyn Iterator<Item = i32>,
  ) -> Vec<i32> {
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();

    IntcodeComputer::run_with_extras(program, extras, in_rx, out_tx);

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
}
