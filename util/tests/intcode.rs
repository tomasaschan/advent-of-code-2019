use std::iter::{empty, once, repeat};
use util::intcode::*;

#[test]
fn addmul_tests() {
  let mut computer = IntcodeComputer::new([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec());
  computer.run(&mut empty());
  assert_eq!(computer.unsafe_read(0), 3500);

  computer = IntcodeComputer::new([1, 0, 0, 0, 99].to_vec());
  computer.run(&mut empty());
  assert_eq!(computer.unsafe_read(0), 2);

  computer = IntcodeComputer::new([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec());
  computer.run(&mut empty());
  assert_eq!(computer.unsafe_read(0), 30);
}

#[test]
fn jmp_tests() {
  let mut computer =
    IntcodeComputer::new([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9].to_vec());
  computer.run(&mut once(0));
  assert_eq!(computer.output.into_iter().last().unwrap(), 0);

  computer = IntcodeComputer::new([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1].to_vec());
  computer.run(&mut once(1));
  assert_eq!(computer.output.into_iter().last().unwrap(), 1);
}

#[test]
fn cmp_tests() {
  let mut sevens = repeat(7);
  let mut eights = repeat(8);

  for program in [
    [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(),
    [3, 3, 1108, -1, 8, 3, 4, 3, 99].to_vec(),
  ]
  .to_vec()
  {
    let mut computer = IntcodeComputer::new(program.clone());
    computer.run(&mut eights);
    assert_eq!(computer.output.into_iter().last().unwrap(), 1);

    computer = IntcodeComputer::new(program.clone());
    computer.run(&mut sevens);
    assert_eq!(computer.output.into_iter().last().unwrap(), 0);
  }

  for program in [
    [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(),
    [3, 3, 1107, -1, 8, 3, 4, 3, 99].to_vec(),
  ]
  .to_vec()
  {
    let mut computer = IntcodeComputer::new(program.clone());
    computer.run(&mut eights);
    assert_eq!(computer.output.into_iter().last().unwrap(), 0);

    computer = IntcodeComputer::new(program.clone());
    computer.run(&mut sevens);
    assert_eq!(computer.output.into_iter().last().unwrap(), 1);
  }
}
