# Advent of Code 2019

![](https://github.com/tomasaschan/advent-of-code-2019/workflows/Run%20checks/badge.svg)

These are my solutions to Advent of Code 2019. Learning [rust](https://rust-lang.org)! ⚙

## Building and running

- [Install tooling](https://www.rust-lang.org/tools/install)

- E.g. `cargo run --bin aoc-2019-01 < input/dec01.txt`.  
  Solvers are named `aoc-2019-DD` and take input from stdin.

- Optional: after `cargo install cargo-watch`, you can
  ```sh
  cargo watch -s "cargo run --release --bin aoc-2019-01 < input/dec01.txt"
  ```
  to re-run automatically on change.

## Getting input

Input can be fetched programmatically using `./get-input.sh DD`; it
assumes the existence of a file `session.secret` holding the value
of your AoC session cookie. No idea for how long that is valid.
