use util::intcode_computer::IntcodeComputer;
use util::io::get_input;

use std::iter::{once, repeat};

fn main() {
    let input: Vec<String> = get_input().split("\n").map(|s| s.to_owned()).collect();

    let program = input[0].to_string();

    let computer = IntcodeComputer::parse(program);

    println!("a: {:?}", solve_a(computer.clone()));
    println!("b: {:?}", solve_b(computer.clone()));
}

fn solve_a(mut computer: IntcodeComputer) -> i32 {
    computer.run(&mut repeat(1));
    computer.output.into_iter().last().unwrap()
}

fn solve_b(mut computer: IntcodeComputer) -> i32 {
    computer.run(&mut once(5));
    computer.output.into_iter().last().unwrap()
}
