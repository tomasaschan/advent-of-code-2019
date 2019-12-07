use std::iter::{from_fn, once};

use permutohedron::Heap;

use util::intcode::*;
use util::io::get_input;

mod b_channels;
use b_channels::solve_b;

fn main() {
    let input = get_input();
    println!("a: {}", solve_a(input.clone()));
    println!("b: {}", solve_b(input.clone()));
}

fn solve_a(input: String) -> i32 {
    let program = IntcodeComputer::parse_program(input);

    let mut settings = vec![0, 1, 2, 3, 4];
    let permutations = Heap::new(&mut settings);

    let mut max_output = 0;

    for permutation in permutations {
        let mut signal = 0;
        for setting in permutation {
            if let Some(s) = IntcodeComputer::run_noninteractive(
                &program,
                &mut once(setting).chain(once(signal)),
            )
            .last()
            {
                signal = *s
            } else {
                panic!(
                    "setting {}, input signal {}, yielded no output!",
                    setting, signal
                )
            }
        }
        if signal > max_output {
            max_output = signal;
        }
    }

    max_output
}
