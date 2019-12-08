extern crate rustyline;

use super::IntcodeComputer;
use rustyline::Editor;

impl IntcodeComputer {
    pub fn debug() {
        let mut rl = Editor::<()>::new();

        while let Some(s) = rl.readline("dbg> ") {
            println!("read {}", s);
        } else {
            println!("goodbye!");
        }
    }
}
