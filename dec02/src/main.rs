extern crate util;

use util::io;

fn main() {
    let input = io::get_input();
    let mut a: Vec<u32> = parse(input);

    a[1] = 12;
    a[2] = 2;
    println!("a: {}", run(a));
}

#[derive(Debug)]
struct IntcodeComputer {
    registers: Vec<u32>,
    position: usize,
}

pub fn parse(input: String) -> Vec<u32> {
    input
        .split(",")
        .map(|s| s.parse::<u32>().expect("invalid input"))
        .collect()
}

fn run(registers: Vec<u32>) -> u32 {
    let mut computer = IntcodeComputer {
        registers,
        position: 0,
    };

    loop {
        step(&mut computer);
        if computer.registers[computer.position] == 99 {
            break computer.registers[0];
        }
    }
}

fn step(computer: &mut IntcodeComputer) {
    let op = computer.registers[computer.position];
    if op == 99 {
        return;
    }
    let a = computer.registers[computer.registers[computer.position + 1] as usize];
    let b = computer.registers[computer.registers[computer.position + 2] as usize];
    let c = match op {
        1 => a + b,
        2 => a * b,
        _ => panic!("invalid opcode {}", op),
    };

    let ci = computer.registers[computer.position + 3];
    computer.registers[ci as usize] = c;
    computer.position = computer.position + 4;
}

#[test]
fn first_sample() {
    let input = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();
    let registers = parse(input);
    let result = run(registers);
    assert_eq!(result, 3500);
}

#[test]
fn second_sample() {
    let input = "1,0,0,0,99".to_string();
    assert_eq!(run(parse(input)), 2);
}
#[test]
fn third_sample() {
    let input = "2,3,0,3,99".to_string();
    assert_eq!(run(parse(input)), 2);
}
#[test]
fn fourth_sample() {
    let input = "2,4,4,5,99,0".to_string();
    assert_eq!(run(parse(input)), 2);
}
#[test]
fn fifth_sample() {
    let input = "1,1,1,4,99,5,6,0,99".to_string();
    assert_eq!(run(parse(input)), 30);
}
