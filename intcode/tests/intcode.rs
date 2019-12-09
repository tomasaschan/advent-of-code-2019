use intcode::*;
use std::iter::{empty, once, repeat};

fn get_value_at_0(program: &Vec<i128>, input: &mut dyn Iterator<Item = i128>) -> i128 {
  let output = Builder::new()
    .program(&program)
    .exit_hook(&vec![4, 0, 99])
    .run_noninteractive(input);
  *output.last().unwrap()
}
fn get_last_output(program: &Vec<i128>, input: &mut dyn Iterator<Item = i128>) -> i128 {
  let output = Builder::new().program(&program).run_noninteractive(input);
  *output.last().unwrap()
}

#[test]
fn addmul_tests() {
  let output1 = get_value_at_0(
    &vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
    &mut empty(),
  );
  assert_eq!(output1, 3500);

  let output2 = get_value_at_0(&vec![1, 0, 0, 0, 99], &mut empty());
  assert_eq!(output2, 2);

  let output3 = get_value_at_0(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &mut empty());
  assert_eq!(output3, 30);
}

#[test]
fn jmp_tests() {
  let out1 = get_last_output(
    &vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
    &mut once(0),
  );
  assert_eq!(out1, 0);

  let out2 = get_last_output(
    &vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
    &mut once(1),
  );
  assert_eq!(out2, 1);
}

#[test]
fn cmp_tests() {
  let mut sevens = repeat(7);
  let mut eights = repeat(8);

  for program in [
    vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
    vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
  ]
  .iter()
  {
    let out1 = get_last_output(&program, &mut eights);
    assert_eq!(out1, 1);

    let out2 = get_last_output(&program, &mut sevens);
    assert_eq!(out2, 0);
  }

  for program in [
    vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
    vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
  ]
  .iter()
  {
    let out1 = get_last_output(&program, &mut eights);
    assert_eq!(out1, 0);

    let out2 = get_last_output(&program, &mut sevens);
    assert_eq!(out2, 1);
  }
}
