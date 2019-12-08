use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::iter::Iterator;
use util::map_2d::Direction;

pub fn solve_a(input: &String) -> i32 {
    let (first, second) = parse(&input);
    fn follow(w: &Vec<Instruction>) -> HashSet<(i32, i32)> {
        HashSet::from_iter(follow_wire(&w).into_iter())
    };
    let (w1, w2) = (follow(&first), follow(&second));

    w1.intersection(&w2)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

pub fn solve_b(input: &String) -> i32 {
    let (first, second) = parse(&input);
    fn follow(is: &Vec<Instruction>) -> HashMap<(i32, i32), usize> {
        let mut map = HashMap::new();
        for (steps, pos) in follow_wire(is).into_iter().enumerate() {
            if !map.contains_key(&pos) {
                map.insert(pos, steps + 1);
            }
        }
        map
    }
    let (w1, w2) = (follow(&first), follow(&second));
    let (s1, s2): (HashSet<&(i32, i32)>, HashSet<&(i32, i32)>) = (
        HashSet::from_iter(w1.keys().into_iter()),
        HashSet::from_iter(w2.keys().into_iter()),
    );

    s1.intersection(&s2)
        .map(|p| w1.get(p).unwrap() + w2.get(p).unwrap())
        .min()
        .unwrap() as i32
}

pub fn parse(input: &String) -> (Vec<Instruction>, Vec<Instruction>) {
    fn parse_step(s: String) -> u32 {
        s.parse::<u32>().expect("invalid step size")
    }

    let paths: Vec<Vec<Instruction>> = input
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|instr| {
                    let steps = parse_step(instr[1..].to_string());
                    let direction = match &instr[0..1] {
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        "D" => Direction::Down,
                        "U" => Direction::Up,
                        _ => panic!("invalid instruction: {}", instr),
                    };
                    Instruction { direction, steps }
                })
                .collect()
        })
        .collect();
    (paths[0].clone(), paths[1].clone())
}

#[derive(Debug, Clone)]
pub struct Instruction {
    direction: Direction,
    steps: u32,
}

fn follow_wire(instructions: &Vec<Instruction>) -> Vec<(i32, i32)> {
    let mut path = Vec::<(i32, i32)>::new();
    let mut pos = (0, 0);
    for i in instructions {
        let mut s = 0;
        while s < i.steps {
            let (x, y) = pos;
            pos = match i.direction {
                Direction::Up => (x, y + 1),
                Direction::Down => (x, y - 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
            };
            path.push(pos);
            s += 1;
        }
    }

    return path;
}
