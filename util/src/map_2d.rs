use std::clone::Clone;
use std::collections::HashMap;
use std::iter::FromIterator;

pub type Coord = (i32, i32);

#[derive(Debug, Clone)]
pub struct WorldMap<T> {
    terrain: HashMap<Coord, T>,
}

impl<T> WorldMap<T> {
    pub fn new() -> WorldMap<T> {
        WorldMap::<T> {
            terrain: HashMap::new(),
        }
    }

    pub fn update<F: Fn(Option<&T>) -> T>(&mut self, coord: Coord, f: F) {
        let v = self.terrain.get(&coord);
        let new_v = f(v);
        self.terrain.insert(coord, new_v);
    }

    pub fn insert(&mut self, coord: Coord, value: T) {
        self.terrain.insert(coord, value);
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        self.terrain.get(coord)
    }

    pub fn get_terrain(&self) -> HashMap<&Coord, &T> {
        HashMap::from_iter((&self.terrain).into_iter())
    }

    /**
     * returns the upper-left and lower-right corners of the map,
     * (xmin,ymin), (xmax,ymax)
     */
    pub fn corners(&self) -> (Coord, Coord) {
        let (mut xmin, mut xmax, mut ymin, mut ymax) = (0, 0, 0, 0);

        for ((x, y), _) in self.terrain.iter() {
            if *x < xmin {
                xmin = *x;
            }
            if *y < ymin {
                ymin = *y;
            }
            if *x > xmax {
                xmax = *x;
            }
            if *y > ymax {
                ymax = *y;
            }
        }

        ((xmin, ymin), (xmax, ymax))
    }
}

impl<T: PartialEq> WorldMap<T> {
    pub fn find(&self, needle: T) -> Vec<Coord> {
        self.terrain
            .iter()
            .filter(|(_, v)| *v == &needle)
            .map(|(coord, _)| *coord)
            .collect()
    }
}

pub fn above(pos: Coord) -> Coord {
    let (x, y) = pos;
    (x, y + 1)
}
pub fn below(pos: Coord) -> Coord {
    let (x, y) = pos;
    (x, y - 1)
}
pub fn left_of(pos: Coord) -> Coord {
    let (x, y) = pos;
    (x - 1, y)
}
pub fn right_of(pos: Coord) -> Coord {
    let (x, y) = pos;
    (x + 1, y)
}

pub fn turn_around(d: Direction) -> Direction {
    turn_left(turn_left(d))
}

pub fn turn_left(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}
pub fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
pub fn move_to(d: Direction, p: Coord) -> Coord {
    match d {
        Direction::Up => above(p),
        Direction::Left => left_of(p),
        Direction::Down => below(p),
        Direction::Right => right_of(p),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}
