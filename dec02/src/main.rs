extern crate util;

use util::intcode_computer::run;
use util::io;

fn main() {
  let input = io::get_input();
  let mut memory: Vec<u32> = parse(input);

  memory[1] = 12;
  memory[2] = 2;
  println!("a: {}", run(memory.clone()));
  let mut answer: i64 = -1;
  for a in 0..99 {
    for b in 0..99 {
      memory[1] = a;
      memory[2] = b;
      let result = run(memory.clone());
      if result == 19690720 {
        answer = (100 * a + b) as i64;
        break;
      }
    }
    if answer > 0 {
      break;
    }
  }

  println!("b: {}", answer)
}

fn parse(input: String) -> Vec<u32> {
  input
    .split(",")
    .map(|s| s.parse::<u32>().expect("invalid input"))
    .collect()
}
