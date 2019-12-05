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

    loop {
        step(&mut computer);
        if computer.memory[computer.instruction_pointer] == 99 {
            break computer.memory[0];
        }
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
    let op = computer.memory[computer.instruction_pointer];
    if op == 99 {
        return;
    }
    match op {
        1 => binary_op(computer, |a, b| a + b),
        2 => binary_op(computer, |a, b| a * b),
        _ => panic!("invalid opcode {}", op),
    };
}

fn binary_op<F: Fn(i32, i32) -> i32>(computer: &mut IntcodeComputer, op: F) {
    let a = value_at_offset(computer, 1);
    let b = value_at_offset(computer, 2);
    set_at_offset(computer, 3, op(a, b));
    step_pointer(computer, 4);
}

fn value_at_offset(computer: &IntcodeComputer, offset: usize) -> i32 {
    computer.memory[computer.memory[computer.instruction_pointer + offset] as usize]
}
fn set_at_offset(computer: &mut IntcodeComputer, offset: usize, value: i32) {
    let ci = computer.memory[computer.instruction_pointer + offset];
    computer.memory[ci as usize] = value;
}
fn step_pointer(computer: &mut IntcodeComputer, steps: usize) {
    computer.instruction_pointer = computer.instruction_pointer + steps;
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
