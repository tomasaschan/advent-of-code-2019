use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct WorldMap<T> {
    terrain: HashMap<(i32, i32), T>,
}

impl<T> WorldMap<T> {
    pub fn new() -> WorldMap<T> {
        WorldMap::<T> {
            terrain: HashMap::new(),
        }
    }

    pub fn update<F: Fn(Option<&T>) -> T>(&mut self, coord: (i32, i32), f: F) {
        let v = self.terrain.get(&coord);
        let new_v = f(v);
        self.terrain.insert(coord, new_v);
    }

    pub fn get(&self, coord: &(i32, i32)) -> Option<&T> {
        self.terrain.get(coord)
    }

    pub fn get_terrain(&self) -> HashMap<&(i32, i32), &T> {
        HashMap::from_iter((&self.terrain).into_iter())
    }

    pub fn corners(&self) -> ((i32, i32), (i32, i32)) {
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
pub fn above(pos: (i32, i32)) -> (i32, i32) {
    let (x, y) = pos;
    (x, y + 1)
}
pub fn below(pos: (i32, i32)) -> (i32, i32) {
    let (x, y) = pos;
    (x, y - 1)
}
pub fn left_of(pos: (i32, i32)) -> (i32, i32) {
    let (x, y) = pos;
    (x - 1, y)
}
pub fn right_of(pos: (i32, i32)) -> (i32, i32) {
    let (x, y) = pos;
    (x + 1, y)
}
pub fn turn_left(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}
pub fn turn_right(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
pub fn move_to(d: &Direction, p: (i32, i32)) -> (i32, i32) {
    match d {
        Direction::Up => above(p),
        Direction::Left => left_of(p),
        Direction::Down => below(p),
        Direction::Right => right_of(p),
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
