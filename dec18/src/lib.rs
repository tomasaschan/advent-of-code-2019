pub mod keys;
mod world;

use keys::{KeyId, KeyRing};
use std::{
    cmp::Reverse,
    collections::{binary_heap::BinaryHeap, HashSet, VecDeque},
    iter::FromIterator,
};
use util::map_2d::{above, below, left_of, right_of, Coord, WorldMap};
use world::*;

pub fn solve_a(input: &String) -> usize {
    let map = WorldMap::from_iter(
        input
            .split("\n")
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().map(move |(x, t)| {
                    let coord = (x as i32, y as i32);
                    (coord, Tile::from(t))
                })
            })
            .flatten(),
    );

    let all_keys = KeyRing::from(input.chars().filter(|c| c.is_ascii_lowercase()));
    let entrance = map.find_single(Tile::Entrance);

    let mut seen = HashSet::new();
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, KeyRing::new(), entrance)));

    while let Some(Reverse((steps, keyring, pos))) = q.pop() {
        if keyring == all_keys {
            return steps;
        }
        if !seen.insert((pos.clone(), keyring.clone())) {
            continue;
        }

        for (np, k, s) in keys_reachable(&map, pos, &keyring) {
            let nks = keyring.with(&k);
            if !seen.contains(&(np, nks.clone())) {
                q.push(Reverse((steps + s, nks.clone(), np)));
            }
        }
    }
    panic!("Didn't find a way to pick up all keys!")
}

pub fn solve_b(input: &String) -> usize {
    let mut map = WorldMap::parse(&input, Tile::from);

    let entrance = map.find_single(Tile::Entrance);
    for c in vec![
        entrance,
        above(entrance),
        below(entrance),
        right_of(entrance),
        left_of(entrance),
    ] {
        map.insert(c, Tile::StoneWall);
    }
    let (a, b, c, d) = (
        above(right_of(entrance)),
        above(left_of(entrance)),
        below(right_of(entrance)),
        below(left_of(entrance)),
    );

    println!("{}", map);

    let all_keys = KeyRing::from(input.chars().filter(|c| c.is_ascii_lowercase()));

    let mut seen = HashSet::new();
    let mut q = BinaryHeap::new();

    q.push(Reverse((0, KeyRing::new(), a, b, c, d)));

    while let Some(Reverse((steps, keyring, a, b, c, d))) = q.pop() {
        if keyring == all_keys {
            return steps;
        }

        if !seen.insert((keyring.clone(), a, b, c, d)) {
            continue;
        }

        for (np, k, s) in keys_reachable(&map, a, &keyring) {
            let nks = keyring.with(&k);
            if !seen.contains(&(nks.clone(), np, b, c, d)) {
                q.push(Reverse((steps + s, nks, np, b, c, d)));
            }
        }

        for (np, k, s) in keys_reachable(&map, b, &keyring) {
            let nks = keyring.with(&k);
            if !seen.contains(&(nks.clone(), a, np, c, d)) {
                q.push(Reverse((steps + s, nks, a, np, c, d)));
            }
        }

        for (np, k, s) in keys_reachable(&map, c, &keyring) {
            let nks = keyring.with(&k);
            if !seen.contains(&(nks.clone(), a, b, np, d)) {
                q.push(Reverse((steps + s, nks, a, b, np, d)));
            }
        }

        for (np, k, s) in keys_reachable(&map, d, &keyring) {
            let nks = keyring.with(&k);
            if !seen.contains(&(nks.clone(), a, b, c, np)) {
                q.push(Reverse((steps + s, nks, a, b, c, np)));
            }
        }
    }
    panic!("Didn't find a way to pick up all keys!");
}

fn keys_reachable(
    map: &WorldMap<Tile>,
    pos: Coord,
    keyring: &KeyRing,
) -> Vec<(Coord, KeyId, usize)> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    let mut nexts = vec![];

    q.push_back((pos, 0));

    while let Some((p, s)) = q.pop_front() {
        seen.insert(p);

        for np in vec![above(p), below(p), right_of(p), left_of(p)] {
            if seen.contains(&np) {
                continue;
            }

            let tile = map.get(&np).unwrap();

            if !tile.passable(&keyring) {
                continue;
            }
            match tile {
                Tile::Key(k) => {
                    if keyring.contains(k) {
                        q.push_back((np, s + 1));
                    } else {
                        nexts.push((np, k.clone(), s + 1))
                    }
                }
                _ => q.push_back((np, s + 1)),
            }
        }
    }

    nexts
}
