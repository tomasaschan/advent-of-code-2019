use super::super::IntcodeComputer;
use editor::Help;
use queues::Queue;
use rustyline::Editor;
use std::sync::mpsc::Sender;

pub struct Debugger {
    computer: IntcodeComputer,
    editor: Editor<Help>,
    input_queue: Queue<i128>,
    input_sender: Sender<i128>,
    has_run_init_hook: bool,
    ascii_mode: bool,
}

impl Debugger {
    pub fn new(
        computer: IntcodeComputer,
        input_sender: Sender<i128>,
        editor: Editor<Help>,
    ) -> Debugger {
        Debugger {
            computer,
            editor,
            input_sender,
            input_queue: Queue::new(),
            has_run_init_hook: false,
            ascii_mode: false,
        }
    }
}

pub mod ascii;
pub mod editor;
pub mod help;
pub mod hook;
pub mod load;
pub mod next;
pub mod provide_input;
pub mod reset;
pub mod show;
pub mod unsafe_read;
