use intcode::{
    debug::{commands::handle, debugger::*},
    ops::read::EmptyInputBehavior::Wait,
    IntcodeComputer,
};
use std::{sync::mpsc::channel, thread::spawn, time::Duration};

pub fn main() {
    let mut rl = editor::create_editor();
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();

    if rl.load_history(".intcode_hist").is_err() {
        // we don't care; just start with no history
    }

    let mut debugger = Debugger::new(
        IntcodeComputer::new(
            &Vec::new(),
            None,
            None,
            None,
            in_rx,
            out_tx,
            Wait(Duration::from_secs(3)),
        ),
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
