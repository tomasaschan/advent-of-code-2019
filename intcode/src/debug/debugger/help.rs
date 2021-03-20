use super::Debugger;

impl Debugger {
    pub fn print_help(&self) {
        println!(
            "{}",
            vec![
                "Available commands:\n",
                "l  | load\tload <path-to-program-file>",
                "h  | hook\thook <init|exit|input> <comma-separated ints>",
                "s  | show",
                "n  | next",
                "in | input\tinput <comma-separated ints>",
                "     ascii\tascii <on|off>",
                "     reset"
            ]
            .join("\n")
        )
    }
}
