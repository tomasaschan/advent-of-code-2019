use super::{super::super::parser::parse, Debugger};
use queues::IsQueue;

impl Debugger {
    pub fn provide_input(&mut self, args: &[&str]) {
        if args.len() != 1 {
            println!(
                "Usage: input <input> where <input> is a comma-separated list of integers to send."
            );
            return;
        }

        self.add_input_to_queue(args[0]);
    }

    pub fn prompt_for_input(&mut self) {
        loop {
            match self.readline(
                &format!("Input (at {}): ", self.computer.instruction_pointer),
                false,
            ) {
                Some(input) => {
                    self.add_input_to_queue(&input);
                    break;
                }
                None => {}
            }
        }
    }

    fn add_input_to_queue(&mut self, input: &str) {
        match parse(input) {
            Ok(is) => {
                for i in is {
                    match self.input_queue.add(i) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Failed to add {} input to queue; try again or quit :/", i);
                        }
                    }
                }
            }
            Err(e) => println!("Invalid input: {:?}", e),
        }
    }
}
