use std::collections::HashMap;
use util::io::get_input;

fn main() {
    let wires = parse(get_input());

    println!("a: {}", solve_a(wires));
}

fn solve_a(wires: Vec<Vec<Instruction>>) -> i32 {
    let mut world = WorldMap::<i32>::new();
    for (i, wire) in wires.iter().enumerate() {
        map_wire(&mut world, 1 << i, wire)
    }

    let mut crossings: Vec<(i32, i32)> = world
        .terrain
        .iter()
        .filter(|(_, v)| **v == 3i32)
        .map(|(pos, _)| *pos)
        .collect();
    crossings.sort_by_key(|(x, y)| x.abs() + y.abs());
    let (x, y) = crossings.first().unwrap();

    x.abs() + y.abs()
}

fn parse(input: String) -> Vec<Vec<Instruction>> {
    fn parse_step(s: String) -> u32 {
        s.parse::<u32>().expect("invalid step size")
    }

    input
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
        .collect()
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u32,
}

fn map_wire(world: &mut WorldMap<i32>, id: i32, path: &Vec<Instruction>) {
    let mut pos = (0, 0);
    for instruction in path {
        let mut s = instruction.steps;
        let mv = match &instruction.direction {
            Direction::Up => above,
            Direction::Down => below,
            Direction::Left => left_of,
            Direction::Right => right_of,
        };
        while s > 0 {
            pos = mv(pos);
            world.update(pos, |x| match x {
                Some(w) => {
                    if *w < id {
                        *w + id
                    } else {
                        *w
                    }
                }
                None => id,
            });
            s -= 1;
        }
    }
}

fn above(pos: (i32, i32)) -> (i32, i32) {
    let (x, y) = pos;
    (x, y + 1)
}
fn below(pos: (i32, i32)) -> (i32, i32) {
    let (x, y) = pos;
    (x, y - 1)
}
fn left_of(pos: (i32, i32)) -> (i32, i32) {
    let (x, y) = pos;
    (x - 1, y)
}
fn right_of(pos: (i32, i32)) -> (i32, i32) {
    let (x, y) = pos;
    (x + 1, y)
}

#[derive(Debug)]
struct WorldMap<T> {
    terrain: HashMap<(i32, i32), T>,
}

impl<T> WorldMap<T> {
    fn new() -> WorldMap<T> {
        WorldMap::<T> {
            terrain: HashMap::new(),
        }
    }

    fn update<F: Fn(Option<&T>) -> T>(&mut self, coord: (i32, i32), f: F) {
        let v = self.terrain.get(&coord);
        let new_v = f(v);
        self.terrain.insert(coord, new_v);
    }
}
