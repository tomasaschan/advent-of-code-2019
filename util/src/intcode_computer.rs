#[derive(Debug)]
struct IntcodeComputer {
    memory: Vec<i32>,
    instruction_pointer: usize,
}

pub fn run(memory: Vec<i32>) -> i32 {
    let mut computer = IntcodeComputer {
        memory: memory,
        instruction_pointer: 0,
    };

    while computer.instruction_pointer != 99 {
        step(&mut computer);
    }

    computer.memory[0]
}

pub fn parse(input: String) -> Vec<i32> {
    input
        .split(",")
        .map(|s| s.parse::<i32>().expect("invalid input"))
        .collect()
}

fn step(computer: &mut IntcodeComputer) {
    let op = computer.memory[computer.instruction_pointer] % 100;

    match op {
        1 => binary_op(computer, |a, b| a + b, op / 100),
        2 => binary_op(computer, |a, b| a * b, op / 100),
        _ => panic!("invalid opcode {}", op),
    };
}

fn binary_op<F: Fn(i32, i32) -> i32>(computer: &mut IntcodeComputer, op: F, mode: i32) {
    let a = value_at_offset(computer, 1, param_mode(mode, 0));
    let b = value_at_offset(computer, 2, param_mode(mode, 1));
    set_at_offset(computer, 3, op(a, b), param_mode(mode, 2));
    step_pointer(computer, 4);
}

fn value_at_offset(computer: &IntcodeComputer, offset: usize, mode: ParamMode) -> i32 {
    let loc = match mode {
        ParamMode::Position => computer.memory[computer.instruction_pointer + offset] as usize,
        ParamMode::Immediate => computer.instruction_pointer + offset,
    };
    computer.memory[loc]
}
fn set_at_offset(computer: &mut IntcodeComputer, offset: usize, value: i32, mode: ParamMode) {
    let loc = match mode {
        ParamMode::Position => computer.memory[computer.instruction_pointer + offset] as usize,
        ParamMode::Immediate => computer.instruction_pointer + offset,
    };
    computer.memory[loc] = value;
}
fn step_pointer(computer: &mut IntcodeComputer, steps: usize) {
    computer.instruction_pointer = computer.instruction_pointer + steps;
}

#[derive(Debug)]
enum ParamMode {
    Immediate,
    Position,
}

fn param_mode(mode: i32, n: usize) -> ParamMode {
    let chars: Vec<char> = mode.to_string().chars().collect();
    if n < chars.len() {
        match chars[n] {
            '0' => ParamMode::Position,
            '1' => ParamMode::Immediate,
            _ => panic!("invalid parameter mode {}", chars[n]),
        }
    } else {
        ParamMode::Position
    }
}

#[test]
fn first_sample() {
    let memory = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec();
    let result = run(memory);
    assert_eq!(result, 3500);
}

#[test]
fn second_sample() {
    assert_eq!(run([1, 0, 0, 0, 99].to_vec()), 2);
}
#[test]
fn third_sample() {
    assert_eq!(run([2, 3, 0, 3, 99].to_vec()), 2);
}
#[test]
fn fourth_sample() {
    assert_eq!(run([2, 4, 4, 5, 99, 0].to_vec()), 2);
}
#[test]
fn fifth_sample() {
    assert_eq!(run([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec()), 30);
}
