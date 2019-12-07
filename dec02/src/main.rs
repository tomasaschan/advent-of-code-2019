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
    println!("b: {}", solve_b(&program.clone()));
}

fn solve_a(mut program: Vec<i32>) -> i32 {
    program[1] = 12;
    program[2] = 2;
    let mut computer = IntcodeComputer::new(program);
    computer.run(&mut empty());
    computer.unsafe_read(0)
}

fn solve_b(program: &Vec<i32>) -> i32 {
    for (a, b) in (0..99).cartesian_product(0..99) {
        let mut p = program.clone();
        p[1] = a;
        p[2] = b;
        let mut computer = IntcodeComputer::new(p);
        computer.run(&mut empty());
        if computer.unsafe_read(0) == 19690720 {
            return 100 * a + b;
        }
    }
    panic!("found no solution");
}
