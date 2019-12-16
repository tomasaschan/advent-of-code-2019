use super::super::super::ops::param_mode;
use super::Debugger;

impl Debugger {
    pub fn show(&self, args: &[&str]) {
        if args.len() == 0 {
            self._show(0);
        } else if let Ok(i) = args[0].parse() {
            self._show(i);
        }
    }
    fn _show(&self, offset: i128) {
        let (a, b, c, d) = (
            self.computer
                .unsafe_read(self.computer.instruction_pointer + offset),
            self.computer
                .unsafe_read(self.computer.instruction_pointer + offset + 1),
            self.computer
                .unsafe_read(self.computer.instruction_pointer + offset + 2),
            self.computer
                .unsafe_read(self.computer.instruction_pointer + offset + 3),
        );
        println!("    +--------+--------+--------+--------+");
        println!(
            "pos | {:<6} | {:<6} | {:<6} | {:<6} |",
            self.computer.instruction_pointer + offset,
            self.computer.instruction_pointer + offset + 1,
            self.computer.instruction_pointer + offset + 2,
            self.computer.instruction_pointer + offset + 3
        );
        println!("val | {:<6} | {:<6} | {:<6} | {:<6} |", a, b, c, d);
        println!(
            "itp | {:<6} | {:<6} | {:<6} | {:<6} |",
            op_of(
                self.computer
                    .unsafe_read(self.computer.instruction_pointer + offset)
            ),
            self.computer.value_at_offset(1, param_mode(a / 100, 1)),
            self.computer.value_at_offset(2, param_mode(a / 100, 2)),
            self.computer.value_at_offset(3, param_mode(a / 100, 3))
        );
        println!("    +--------+--------+--------+--------+");
    }
}
fn op_of(i: i128) -> String {
    match i % 100 {
        1 => "add",
        2 => "mul",
        3 => "inp",
        4 => "out",
        5 => "jift",
        6 => "jiff",
        7 => "lt",
        8 => "eq",
        9 => "base",
        99 => "halt",
        _ => "????",
    }
    .to_string()
}
