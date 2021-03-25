use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter, Write},
};
use util::map_2d::{move_to, Coord, Direction, WorldMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wall {
    Inner,
    Outer,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Open,
    Label(char),
    Entrance,
    Exit,
    Portal(Coord, Direction, Wall),
}
impl Tile {
    pub fn passable(&self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Open => true,
            Tile::Label(_) => false,
            Tile::Entrance => true,
            Tile::Exit => true,
            Tile::Portal(_, _, _) => true,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            ' ' => Tile::Wall,
            '#' => Tile::Wall,
            '.' => Tile::Open,
            l @ 'A'..='Z' => Tile::Label(l),
            c => panic!("Invalid tile char: '{}'", c),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_char(match self {
            Tile::Wall => ' ',
            Tile::Open => '.',
            Tile::Label(l) => *l,
            Tile::Entrance => 'O',
            Tile::Exit => 'X',
            Tile::Portal(_, _, _) => '@',
        })
    }
}

pub fn parse(input: &String) -> WorldMap<Tile> {
    let mut map = WorldMap::parse(&input, Tile::from);
    connect_portals(&mut map);

    map
}

fn connect_portals(map: &mut WorldMap<Tile>) {
    let mut portals = HashMap::new();

    for coord in map.find(Tile::Open) {
        if let Some((d, a, b)) = label_at(&map, &coord) {
            match (a, b) {
                ('A', 'A') => map.insert(coord, Tile::Entrance),
                ('Z', 'Z') => map.insert(coord, Tile::Exit),
                _ => {
                    portals.entry((a, b)).or_insert(vec![]).push((coord, d));
                }
            }
        }
    }

    for portal in portals.values() {
        match portal[..] {
            [(c1, d1), (c2, d2)] => {
                map.insert(c1, Tile::Portal(c2, d1, inner_or_outer(&map, &c1)));
                map.insert(c2, Tile::Portal(c1, d2, inner_or_outer(&map, &c2)));
            }
            _ => {}
        }
    }
}

fn inner_or_outer(map: &WorldMap<Tile>, (x, y): &Coord) -> Wall {
    let ((xlo, ylo), (xhi, yhi)) = map.corners();
    let result = if (x - xlo) <= 2 || (y - ylo) <= 2 || (xhi - x) <= 2 || (yhi - y) <= 2 {
        Wall::Outer
    } else {
        Wall::Inner
    };
    result
}

fn label_at(map: &WorldMap<Tile>, coord: &Coord) -> Option<(Direction, char, char)> {
    for d in Direction::all() {
        if let (Some(Tile::Open), Some(Tile::Label(a)), Some(Tile::Label(b))) = (
            map.get(&coord),
            map.get(&move_to(d, &coord)),
            map.get(&move_to(d, &move_to(d, &coord))),
        ) {
            return match d {
                Direction::Down => Some((d, *a, *b)),
                Direction::Right => Some((d, *a, *b)),
                Direction::Left => Some((d, *b, *a)),
                Direction::Up => Some((d, *b, *a)),
            };
        }
    }
    None
}
