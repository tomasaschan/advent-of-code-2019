#!/bin/bash

set -Eueo pipefail

./get-input.sh $1

if ! [ -d dec$1 ]; then
  cargo new dec$1
  sed -i -e "s/name = \"dec$1\"/name = \"aoc-2019-$1\"/" dec$1/Cargo.toml
  echo "util = { path = \"../util\" }
intcode = { path = \"../intcode\" }" >> dec$1/Cargo.toml

  printf "use aoc_2019_$1::solve_a;
use util::io::get_input;

fn main() {
    let input = get_input();

    println!(\"a: {}\", solve_a(&input));
    //println!(\"b: {}\", solve_b(&input));
}
" > dec$1/src/main.rs
  printf "pub fn solve_a(input: &String) -> i128 {
    0
}
" > dec$1/src/lib.rs
fi

if ! [ -f dec$1/src/lib.rs ]; then
  touch dec$1/src/lib.rs
fi

if ! [ -d dec$1/tests ]; then
  mkdir dec$1/tests
printf "use aoc_2019_$1::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file(\"../input/dec$1.txt\");

    assert_eq!(solve_a(&input), 0);
}

//#[test]
//fn solves_b() {
//    let input = read_file(\"../input/dec$1.txt\");
//
//    assert_eq!(solve_b(&input), ...);
//}
" >dec$1/tests/dec$1.rs
fi
