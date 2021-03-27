use super::super::super::parser::parse;
use super::super::super::Builder;
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

    pub fn hooks(&self, args: &[&str]) {
        fn show_hook(name: &str, hook: &Option<Vec<i128>>) {
            match hook {
                Some(instrs) => {
                    let ending_in_99 = Builder::end_in_99(instrs.clone());
                    println!(
                        "{:>5}: {}",
                        name,
                        ending_in_99
                            .iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                }
                None => println!("{:>5}: <no hook>", name),
            }
        }
        show_hook("init", &self.computer.init_hook);
        show_hook("input", &self.computer.input_hook);
        show_hook("exit", &self.computer.exit_hook);
    }
}
