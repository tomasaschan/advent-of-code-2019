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
