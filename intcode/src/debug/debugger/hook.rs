use super::super::super::parser::parse;
use super::Debugger;

impl Debugger {
    pub fn hook(&mut self, args: &[&str]) {
        fn set_hook<F: FnOnce(Vec<i128>)>(s: &str, f: F) {
            match parse(s) {
                Ok(hook) => f(hook),
                Err(_) => println!("Invalid hook definition; provide the hook as a comma-separated list of intcode instructions.")
            }
        }
        if args.len() != 2 {
            println!("Usage: hook <init|exit|input> <instruction>");
            return;
        }
        match args[0] {
            "init" => set_hook(args[1], |hook| self.computer.init_hook = Some(hook)),
            "exit" => set_hook(args[1], |hook| self.computer.exit_hook = Some(hook)),
            "input" => set_hook(args[1], |hook| self.computer.input_hook = Some(hook)),
            _ => println!("Invalid hook type; valid values are init, exit or input."),
        }
    }
}
