use intcode::{
    debug::{commands::handle, debugger::*},
    IntcodeComputer,
};
use std::sync::mpsc::channel;
use std::thread::spawn;

pub fn main() {
    let mut rl = editor::create_editor();
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();

    if rl.load_history(".intcode_hist").is_err() {
        // we don't care; just start with no history
    }

    let mut debugger = Debugger::new(
        IntcodeComputer::new(&Vec::new(), None, None, None, in_rx, out_tx),
        in_tx,
        rl,
    );

    spawn(move || {
        for o in out_rx.iter() {
            println!("Got output: {}", o);
        }
    });

    println!("Welcome to Intcode Debugger!");
    loop {
        match debugger.readline("dbg> ", true) {
            Some(cmd) => {
                handle(&cmd, &mut debugger);
            }
            None => break,
        }
    }
}
