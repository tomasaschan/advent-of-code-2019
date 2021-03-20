use super::Debugger;

impl Debugger {
    pub fn toggle_ascii(&mut self, args: &[&str]) {
        if args.len() != 1 || (args[0] != "on" && args[0] != "off") {
            println!("Usage: ascii <on|off>");
            return;
        }
        if args[0] == "on" {
            self.ascii_mode = true;
            return;
        }

        if args[0] == "off" {
            self.ascii_mode = false;
            return;
        }
    }
}
