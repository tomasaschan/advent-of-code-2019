use std::iter::{once, repeat};
use util::intcode::Builder;

pub fn solve_a(input: &String) -> i32 {
    let output = Builder::new()
        .parse(&input)
        .run_noninteractive(&mut repeat(1));
    *output.last().unwrap()
}

pub fn solve_b(input: &String) -> i32 {
    let output = Builder::new()
        .parse(&input)
        .run_noninteractive(&mut once(5));
    *output.last().unwrap()
}
