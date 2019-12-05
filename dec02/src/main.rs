extern crate itertools;
extern crate util;

use itertools::Itertools;
use std::iter::empty;
use util::intcode_computer::IntcodeComputer;
use util::io;

fn main() {
    let input = io::get_input();
    let computer = IntcodeComputer::parse(input);

    println!("a: {}", solve_a(computer.clone()));
    println!("b: {}", solve_b(computer.clone()));
}

fn solve_a(mut computer: IntcodeComputer) -> i32 {
    computer.set_at(1, 12);
    computer.set_at(2, 2);

    computer.run(&mut empty());
    computer.get_at(0)
}

fn solve_b(mut computer: IntcodeComputer) -> i32 {
    for (a, b) in (0..99).cartesian_product(0..99) {
        computer.set_at(1, a);
        computer.set_at(2, b);
        computer.clone().run(&mut empty());
        if computer.get_at(0) == 19690720 {
            return 100 * a + b;
        }
    }
    panic!("found no solution");
}
