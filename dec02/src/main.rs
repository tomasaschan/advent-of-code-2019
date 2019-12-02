extern crate itertools;
extern crate util;

use itertools::Itertools;
use util::intcode_computer::run;
use util::io;

fn main() {
  let input = io::get_input();
  let memory: Vec<u32> = parse(input);

  println!("a: {}", solve_a(memory.clone()));
  println!("b: {}", solve_b(memory.clone()));
}

fn parse(input: String) -> Vec<u32> {
  input
    .split(",")
    .map(|s| s.parse::<u32>().expect("invalid input"))
    .collect()
}

fn solve_a(mut memory: Vec<u32>) -> u32 {
  memory[1] = 12;
  memory[2] = 2;
  return run(memory);
}

fn solve_b(mut memory: Vec<u32>) -> u32 {
  for (a, b) in (0..99).cartesian_product(0..99) {
    memory[1] = a;
    memory[2] = b;
    let result = run(memory.clone());
    if result == 19690720 {
      return 100 * a + b;
    }
  }
  panic!("found no solution");
}
