extern crate itertools;
extern crate util;

use itertools::Itertools;
use std::iter::empty;
use util::intcode::IntcodeComputer;
use util::io;

fn main() {
    let input = io::get_input();
    let program = IntcodeComputer::parse_program(input);

    println!("a: {}", solve_a(program.clone()));
    println!("b: {}", solve_b(program.clone()));
}

fn solve_a(mut program: Vec<i32>) -> i32 {
    program[1] = 12;
    program[2] = 2;

    let output = run_and_read_0(&mut program);
    output
}

fn solve_b(mut program: Vec<i32>) -> i32 {
    for (a, b) in (0..99).cartesian_product(0..99) {
        program[1] = a;
        program[2] = b;
        let out = run_and_read_0(&mut program);
        if out == 19690720 {
            return 100 * a + b;
        }
    }
    panic!("found no solution");
}

fn run_and_read_0(program: &mut Vec<i32>) -> i32 {
    let output =
        IntcodeComputer::run_with_extras_noninteractive(&program, &vec![4, 0, 99], &mut empty());

    *output.last().expect("No output from program")
}
