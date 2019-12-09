use intcode::Builder;
use std::iter::{once, repeat};

pub fn solve_a(input: &String) -> i128 {
    let output = Builder::new()
        .parse(&input)
        .run_noninteractive(&mut repeat(1));
    *output.last().unwrap()
}

pub fn solve_b(input: &String) -> i128 {
    let output = Builder::new()
        .parse(&input)
        .run_noninteractive(&mut once(5));
    *output.last().unwrap()
}
