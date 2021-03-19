use std::clone::Clone;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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

impl<T: PartialEq + Display> Display for WorldMap<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let ((xlo, ylo), (xhi, yhi)) = self.corners();

        for _ in xlo..=-1 {
            f.write_str("-")?;
        }
        f.write_str("\n")?;

        let mut x_places = (max(-xlo, xhi) as f64).log10() as i32 + 1;
        let y_places = (max(-ylo, yhi) as f64).log10() as i32 + 1;
        let y_width = y_places + if ylo < 0 { 2 } else { 1 };

        while x_places > 0 {
            f.write_str(&" ".repeat(y_width as usize))?;
            for x in xlo..=xhi {
                f.write_str(&format!(
                    "{}",
                    x.abs() / 10_i32.pow(x_places as u32 - 1) % 10
                ))?;
            }
            f.write_str("\n")?;
            x_places -= 1;
        }
        f.write_str(&"\n")?;

        let max_yw = max(format!("{}", ylo).len(), format!("{}", yhi).len());
        for y in ylo..=yhi {
            let yw = format!("{}", y).len();
            f.write_str(&" ".repeat(max_yw - yw))?;
            f.write_str(&format!("{} ", y))?;

            for x in xlo..=xhi {
                match self.get(&(x, y)) {
                    Some(t) => f.write_str(&format!("{}", t)),
                    None => f.write_str(" "),
                }?;
            }

            f.write_str(&"\n")?;
        }

        Ok(())
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

    pub fn find_single(&self, needle: T) -> Coord {
        let all = self.find(needle);
        let loc = all.first().unwrap();
        loc.clone()
    }
}

impl<T: PartialEq + Copy> WorldMap<T> {
    pub fn parse<F: Copy + Fn(char) -> T>(input: &String, read: F) -> WorldMap<T> {
        WorldMap::from_iter(
            input
                .split("\n")
                .enumerate()
                .map(|(y, line)| {
                    line.chars().enumerate().map(move |(x, t)| {
                        let coord = (x as i32, y as i32);
                        (coord, read(t))
                    })
                })
                .flatten(),
        )
    }
}

impl<T: PartialEq + Copy> FromIterator<(Coord, T)> for WorldMap<T> {
    fn from_iter<I>(iter: I) -> WorldMap<T>
    where
        I: IntoIterator<Item = (Coord, T)>,
    {
        let mut map = WorldMap::<T>::new();
        for ((x, y), t) in iter {
            map.update((x, y), |_| t);
        }
        map
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
