use util::intcode::IntcodeComputer;
use util::io::get_input;

use std::iter::{once, repeat};

fn main() {
    let program = IntcodeComputer::parse_program(get_input());

    println!("a: {:?}", solve_a(&program));
    println!("b: {:?}", solve_b(&program));
}

fn solve_a(program: &Vec<i32>) -> i32 {
    let output = IntcodeComputer::run_noninteractive(program, &mut repeat(1));
    *output.last().unwrap()
}

fn solve_b(program: &Vec<i32>) -> i32 {
    let output = IntcodeComputer::run_noninteractive(program, &mut once(5));
    *output.last().unwrap()
}
