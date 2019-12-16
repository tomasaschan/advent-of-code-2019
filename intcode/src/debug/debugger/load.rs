use super::super::super::parser::parse;
use super::Debugger;
use std::fs;

impl Debugger {
    pub fn load(&mut self, args: &[&str]) {
        if args.len() != 1 {
            println!("Usage: load <path-to-program-file>");
            return;
        }
        let file = args[0];
        match fs::read_to_string(file) {
            Ok(p) => match parse(&p.trim()) {
                Ok(q) => {
                    self.computer.program = q.clone();
                    self.computer.reset()
                }
                Err(err) => println!("Failed to parse: {}", err),
            },
            Err(err) => println!("Failed to read {}: {}", file, err),
        }
    }
}
