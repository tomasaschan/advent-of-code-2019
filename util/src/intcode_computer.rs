use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct IntcodeComputer {
    memory: Vec<i32>,
    instruction_pointer: usize,
    pub output: Vec<i32>,
    pub debug_mode: bool,
}

impl IntcodeComputer {
    pub fn parse(input: String) -> IntcodeComputer {
        let program = input
            .split(",")
            .map(|s| s.parse::<i32>().expect("invalid input"))
            .collect();
        IntcodeComputer::new(program)
    }

    pub fn new(program: Vec<i32>) -> IntcodeComputer {
        IntcodeComputer {
            memory: program,
            instruction_pointer: 0,
            output: Vec::new(),
            debug_mode: false,
        }
    }

    pub fn set_at(&mut self, loc: usize, value: i32) {
        self.memory[loc] = value;
    }

    pub fn get_at(&self, loc: usize) -> i32 {
        self.memory[loc]
    }

    pub fn run(&mut self, stdin: &mut dyn Iterator<Item = i32>) {
        while self.memory[self.instruction_pointer] != 99 {
            self.step(stdin);
        }
    }

    fn step(&mut self, stdin: &mut dyn Iterator<Item = i32>) {
        let op = self.memory[self.instruction_pointer] % 100;
        let mode = self.memory[self.instruction_pointer] / 100;
        if self.debug_mode {
            println!(
                "at {}: handling {:?} with mode {}",
                self.instruction_pointer,
                self.memory[self.instruction_pointer..self.instruction_pointer + 4].to_vec(),
                mode
            );
        }
        match op {
            1 => self.binary_op(|a, b| a + b, mode),
            2 => self.binary_op(|a, b| a * b, mode),
            3 => self.slurp(stdin),
            4 => self.out(mode),
            5 => self.jmp_if(true, mode),
            6 => self.jmp_if(false, mode),
            7 => self.cmp(|a, b| a < b, mode),
            8 => self.cmp(|a, b| a == b, mode),
            _ => {
                let mut buffer = File::create("core.dump").expect("core dump failed");

                // this call
                write!(buffer, "{:?}", self.memory).expect("core dump failed");

                // let mut buffer = try!(File::create("hello_world.txt"));
                // write!(buffer, "{:?}", self.memory);
                panic!(
                    "invalid opcode {} at position {}. memory dumped to core.tmp",
                    op, self.instruction_pointer
                )
            }
        };
    }

    fn binary_op<F: Fn(i32, i32) -> i32>(&mut self, op: F, mode: i32) {
        let a = self.value_at_offset(1, param_mode(mode, 0));
        let b = self.value_at_offset(2, param_mode(mode, 1));
        self.set_at_offset(3, op(a, b));
        self.step_pointer(4);
    }
    fn jmp_if(&mut self, t: bool, mode: i32) {
        let a = self.value_at_offset(1, param_mode(mode, 0));

        if t ^ (a == 0) {
            let p = self.value_at_offset(2, param_mode(mode, 1));
            if self.debug_mode {
                println!(
                    "jump if {}; {} {} 0; jumping to {}",
                    t,
                    a,
                    if t { "!=" } else { "==" },
                    p
                );
            }
            self.instruction_pointer = p as usize;
        } else {
            if self.debug_mode {
                println!(
                    "jump if {}; {} {} 0; not jumping",
                    t,
                    a,
                    if t { "==" } else { "!=" }
                );
            }
            self.step_pointer(3);
        }
    }

    fn cmp<F: Fn(i32, i32) -> bool>(&mut self, comparer: F, mode: i32) {
        let a = self.value_at_offset(1, param_mode(mode, 0));
        let b = self.value_at_offset(2, param_mode(mode, 1));

        self.set_at_offset(3, if comparer(a, b) { 1 } else { 0 });
        self.step_pointer(4);
    }

    fn slurp(&mut self, stdin: &mut dyn Iterator<Item = i32>) {
        let value = match stdin.next() {
            Some(s) => s,
            None => panic!("EOF when reading from input!"),
        };

        if self.debug_mode {
            println!("read {} from input", value);
        }
        self.set_at_offset(1, value);
        self.step_pointer(2);
    }

    fn out(&mut self, mode: i32) {
        let v = self.value_at_offset(1, param_mode(mode, 0));
        self.output.push(v);
        if self.debug_mode {
            println!("wrote {} to stdout", v);
        }
        self.step_pointer(2);
    }

    fn value_at_offset(&mut self, offset: usize, mode: ParamMode) -> i32 {
        let loc = match mode {
            ParamMode::Position => self.memory[self.instruction_pointer + offset] as usize,
            ParamMode::Immediate => self.instruction_pointer + offset,
        };
        let value = self.memory[loc];
        if self.debug_mode {
            println!("read {} from {}", value, loc);
        }
        value
    }
    fn set_at_offset(&mut self, offset: usize, value: i32) {
        let loc = self.memory[self.instruction_pointer + offset] as usize;
        self.memory[loc] = value;
        if self.debug_mode {
            println!("wrote {} to {}", value, loc);
        }
    }

    fn step_pointer(&mut self, steps: usize) {
        self.instruction_pointer = self.instruction_pointer + steps;
    }
}

#[derive(Debug, PartialEq)]
enum ParamMode {
    Immediate,
    Position,
}

fn param_mode(mut mode: i32, mut n: usize) -> ParamMode {
    loop {
        if n == 0 {
            break match mode % 10 {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                _ => panic!("invalid parameter mode {}", n),
            };
        }
        mode /= 10;
        n -= 1;
    }
}

use std::iter::{empty, once, repeat};

#[test]
fn param_mode_test() {
    assert_eq!(param_mode(0, 0), ParamMode::Position);
    assert_eq!(param_mode(0, 1), ParamMode::Position);
    assert_eq!(param_mode(0, 2), ParamMode::Position);
    assert_eq!(param_mode(1, 0), ParamMode::Immediate);
    assert_eq!(param_mode(1, 1), ParamMode::Position);
    assert_eq!(param_mode(1, 2), ParamMode::Position);
    assert_eq!(param_mode(10, 0), ParamMode::Position);
    assert_eq!(param_mode(10, 1), ParamMode::Immediate);
    assert_eq!(param_mode(10, 2), ParamMode::Position);
    assert_eq!(param_mode(11, 0), ParamMode::Immediate);
    assert_eq!(param_mode(11, 1), ParamMode::Immediate);
    assert_eq!(param_mode(11, 2), ParamMode::Position);
    assert_eq!(param_mode(110, 0), ParamMode::Position);
    assert_eq!(param_mode(110, 1), ParamMode::Immediate);
    assert_eq!(param_mode(110, 2), ParamMode::Immediate);
    assert_eq!(param_mode(111, 0), ParamMode::Immediate);
    assert_eq!(param_mode(111, 1), ParamMode::Immediate);
    assert_eq!(param_mode(111, 2), ParamMode::Immediate);
}

#[test]
fn addmul_tests() {
    let mut computer = IntcodeComputer::new([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec());
    computer.run(&mut empty());
    assert_eq!(computer.get_at(0), 3500);

    computer = IntcodeComputer::new([1, 0, 0, 0, 99].to_vec());
    computer.run(&mut empty());
    assert_eq!(computer.get_at(0), 2);

    computer = IntcodeComputer::new([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec());
    computer.run(&mut empty());
    assert_eq!(computer.get_at(0), 30);
}

#[test]
fn jmp_tests() {
    let mut computer =
        IntcodeComputer::new([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9].to_vec());
    computer.run(&mut once("0".to_owned()));
    assert_eq!(computer.stdout.last().unwrap(), "0");

    computer = IntcodeComputer::new([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1].to_vec());
    computer.run(&mut once("1".to_owned()));
    assert_eq!(computer.stdout.last().unwrap(), "1");
}

#[test]
fn cmp_tests() {
    let mut sevens = repeat("7".to_owned());
    let mut eights = repeat("8".to_owned());
    let mut nines = repeat("9".to_owned());

    for program in [
        [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(),
        [3, 3, 1108, -1, 8, 3, 4, 3, 99].to_vec(),
    ]
    .to_vec()
    {
        let mut computer = IntcodeComputer::new(program.clone());
        computer.run(&mut eights);
        assert_eq!(computer.stdout.last().unwrap(), "1");

        computer = IntcodeComputer::new(program.clone());
        computer.run(&mut sevens);
        assert_eq!(computer.stdout.last().unwrap(), "0");
    }

    for program in [
        [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(),
        [3, 3, 1107, -1, 8, 3, 4, 3, 99].to_vec(),
    ]
    .to_vec()
    {
        let mut computer = IntcodeComputer::new(program.clone());
        computer.run(&mut eights);
        assert_eq!(computer.stdout.last().unwrap(), "0");

        computer = IntcodeComputer::new(program.clone());
        computer.run(&mut sevens);
        assert_eq!(computer.stdout.last().unwrap(), "1");
    }
}
