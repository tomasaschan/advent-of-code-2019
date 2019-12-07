use std::iter::once;

use permutohedron::Heap;

use util::intcode::*;
use util::io::get_input;

fn main() {
    let input = get_input();
    println!("a: {}", solve_a(input.clone()));
}

fn solve_a(input: String) -> i32 {
    let init = || IntcodeComputer::parse(input.clone());

    let mut amps = vec![init(), init(), init(), init(), init()];
    let mut settings = vec![0, 1, 2, 3, 4];
    let permutations = Heap::new(&mut settings);

    let mut max_output = 0;

    for permutation in permutations {
        let mut signal = 0;
        for (computer, setting) in amps.iter_mut().zip(permutation) {
            computer.reset();
            if let Some(s) = computer.run(&mut once(setting).chain(once(signal))) {
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
