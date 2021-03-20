mod world;

use std::collections::{HashSet, VecDeque};
use util::map_2d::{move_to, Coord, Direction, WorldMap};
use world::{parse, Tile, Wall};

pub fn solve_a(input: &String) -> usize {
    let map = parse(&input);
    find_shortest_path(&map, false)
}

pub fn solve_b(input: &String) -> usize {
    let map = parse(&input);
    find_shortest_path(&map, true)
}

fn nexts(p: &Coord, t: &Tile, lvl: i128, recursive: bool) -> Vec<(Coord, i128)> {
    let mut ns = Vec::new();
    for d in Direction::all() {
        match t {
            Tile::Portal(to, dir, io) => {
                if *dir == d {
                    ns.push((
                        *to,
                        match io {
                            Wall::Inner => lvl + if recursive { 1 } else { 0 },
                            Wall::Outer => lvl - if recursive { 1 } else { 0 },
                        },
                    ));
                } else {
                    ns.push((move_to(d, p), lvl));
                }
            }
            _ => ns.push((move_to(d, p), lvl)),
        }
    }
    ns
}

fn find_shortest_path(map: &WorldMap<Tile>, recursive: bool) -> usize {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    q.push_back((map.find_single(Tile::Entrance), 0, 0));

    while let Some((p, lvl, s)) = q.pop_front() {
        if let Some(t) = map.get(&p) {
            match t {
                Tile::Exit => {
                    if lvl == 0 {
                        return s;
                    } else {
                        seen.insert((p, lvl));
                        let neighbors = nexts(&p, &t, lvl, recursive);
                        for (n, lvl) in neighbors.into_iter().filter(|(n, l)| {
                            *l >= 0 && map.get(&n).unwrap_or(&Tile::Wall).passable()
                        }) {
                            if !seen.contains(&(n, lvl)) {
                                q.push_back((n, lvl, s + 1));
                            }
                        }
                    }
                }
                t => {
                    seen.insert((p, lvl));
                    let neighbors = nexts(&p, &t, lvl, recursive);
                    for (n, lvl) in neighbors
                        .into_iter()
                        .filter(|(n, l)| *l >= 0 && map.get(&n).unwrap_or(&Tile::Wall).passable())
                    {
                        if !seen.contains(&(n, lvl)) {
                            q.push_back((n, lvl, s + 1));
                        }
                    }
                }
            }
        } else {
            panic!("Walked off-grid at {:?}", p)
        }
    }
    panic!("found no way from entrance to exit!")
}
