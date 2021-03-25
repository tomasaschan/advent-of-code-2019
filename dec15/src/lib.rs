use intcode::Builder;
// use queues::{IsQueue, Queue};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use util::map_2d::*;

pub fn solve_a(input: &String) -> usize {
    let mut world = WorldMap::<i128>::new();

    explore(&mut world, input);
    find_shortest_path(world.clone())
}

pub fn solve_b(input: &String) -> usize {
    let mut world = WorldMap::<i128>::new();
    explore(&mut world, input);

    let locs = world.find(2);
    let loc = *locs.get(0).unwrap();
    fill(world.clone(), loc)
}

fn find_shortest_path(mut world: WorldMap<i128>) -> usize {
    let mut exploration_queue = BinaryHeap::new();
    exploration_queue.push((Reverse(0), (0, 0)));

    loop {
        match exploration_queue.pop() {
            Some((Reverse(s), p)) => {
                if world.get(&p) == Some(&2) {
                    break s;
                }

                world.insert(p, 3);

                for mv in Direction::all().into_iter().filter(|d| {
                    let x = world.get(&move_to(*d, &p));
                    x == Some(&1) || x == Some(&2)
                }) {
                    let next = move_to(mv, &p);
                    exploration_queue.push((Reverse(s + 1), next));
                }
            }
            None => break 0,
        }
    }
}

fn explore(world: &mut WorldMap<i128>, input: &String) {
    let mut bot = (0, 0);
    let mut exploration_stack = Vec::new();

    let (i, o) = Builder::new().parse(input).block_on_input().run();

    loop {
        for mv in Direction::all()
            .into_iter()
            .filter(|d| world.get(&move_to(*d, &bot)).is_none())
        {
            exploration_stack.push(Move::Explore(mv));
        }

        match exploration_stack.pop() {
            Some(Move::Explore(ed)) => {
                i.send(command(ed)).unwrap();
                match o.recv().unwrap() {
                    0 => world.insert(move_to(ed, &bot), 0),
                    1 => {
                        bot = move_to(ed, &bot);
                        world.insert(bot, 1);
                        exploration_stack.push(Move::Backtrack(turn_around(ed)))
                    }
                    2 => {
                        bot = move_to(ed, &bot);
                        world.insert(bot, 2);
                        exploration_stack.push(Move::Backtrack(turn_around(ed)))
                    }
                    _ => panic!("Unknown output from bot: {}", 2),
                }
            }
            Some(Move::Backtrack(bd)) => {
                i.send(command(bd)).unwrap();
                match o.recv().unwrap() {
                    1 => bot = move_to(bd, &bot),
                    o => {
                        draw(&world);
                        panic!("Tried to backtrack into a non-empty square! (from {:?} moving {:?} yielded output {})", bot, bd, o);
                    }
                }
            }
            None => break,
        }
    }
}

fn fill(world: WorldMap<i128>, start: (i32, i32)) -> usize {
    let mut filled = HashMap::new();
    filled.insert(start, 0 as usize);

    let mut fill_queue = BinaryHeap::new();
    fill_queue.push((Reverse(0), start));

    loop {
        match fill_queue.pop() {
            Some((Reverse(s), loc)) => {
                for next in Direction::all().iter().map(|d| move_to(*d, &loc)) {
                    let terrain = world.get(&next);
                    if (terrain != Some(&1) && terrain != Some(&2)) || filled.contains_key(&next) {
                        continue;
                    }
                    filled.insert(next, s + 1);
                    fill_queue.push((Reverse(s + 1), next));
                }
            }
            None => break,
        }
    }
    *filled.values().max().unwrap()
}

fn command(dir_to_move: Direction) -> i128 {
    match dir_to_move {
        Direction::Up => 1,
        Direction::Down => 2,
        Direction::Left => 3,
        Direction::Right => 4,
    }
}

enum Move {
    Backtrack(Direction),
    Explore(Direction),
}

fn draw(world: &WorldMap<i128>) {
    let ((xmin, ymin), (xmax, ymax)) = world.corners();
    for y_unadj in ymin..=ymax {
        let y = ymax - (y_unadj - ymin);
        for x in xmin..=xmax {
            match world.get(&(x, y)) {
                Some(2) => print!("{}", termion::color::Fg(termion::color::Blue)),
                _ => print!("{}", termion::style::Reset),
            };
            print!(
                "{}",
                if (x, y) == (0, 0) {
                    format!("O")
                } else {
                    match world.get(&(x, y)) {
                        Some(0) => format!("#"),
                        Some(1) => format!("."),
                        Some(2) => format!("X"),
                        Some(3) => format!(
                            "{}{}{}",
                            termion::color::Fg(termion::color::Green),
                            "P",
                            termion::color::Fg(termion::color::Reset)
                        ),
                        _ => format!(" "),
                    }
                }
            );
        }
        println!();
    }
}
