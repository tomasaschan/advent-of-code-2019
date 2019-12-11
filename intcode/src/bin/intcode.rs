use intcode::IntcodeComputer;
use rustyline;
use rustyline::error::ReadlineError;
use std::collections::HashMap;
use std::sync::mpsc::channel;

pub fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();

    let mut computer = IntcodeComputer::new(&Vec::new(), None, None, in_rx, out_tx);

    println!("Welcome to Intcode Debugger!");
    loop {
        match rl.readline("dbg> ") {
            Ok(cmd) => handle(cmd, &mut computer),
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn handle(input: String, computer: &mut IntcodeComputer) {
    if input.trim().len() == 0 {
        return;
    }
    let cmd: Vec<&str> = input.split_whitespace().collect();
    println!("{}", cmd[0]);
}
