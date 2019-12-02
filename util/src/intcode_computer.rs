#[derive(Debug)]
struct IntcodeComputer {
    memory: Vec<u32>,
    instruction_pointer: usize,
}

pub fn run(memory: Vec<u32>) -> u32 {
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
}

fn step(computer: &mut IntcodeComputer) {
    let op = computer.memory[computer.instruction_pointer];
    if op == 99 {
        return;
    }
    let a = computer.memory[computer.memory[computer.instruction_pointer + 1] as usize];
    let b = computer.memory[computer.memory[computer.instruction_pointer + 2] as usize];
    let c = match op {
        1 => a + b,
        2 => a * b,
        _ => panic!("invalid opcode {}", op),
    };

    let ci = computer.memory[computer.instruction_pointer + 3];
    computer.memory[ci as usize] = c;
    computer.instruction_pointer = computer.instruction_pointer + 4;
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
