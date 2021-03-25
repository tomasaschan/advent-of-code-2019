use super::keys::*;
use std::{
    fmt::{Display, Error, Formatter, Write},
    iter::FromIterator,
};
use util::map_2d::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tile {
    Entrance,
    OpenPassage,
    StoneWall,
    Door(KeyId),
    Key(KeyId),
}

impl Tile {
    pub fn passable(&self, keys: &KeyRing) -> bool {
        match self {
            Tile::Entrance => true,
            Tile::OpenPassage => true,
            Tile::StoneWall => false,
            Tile::Door(d) => keys.contains(d),
            Tile::Key(_) => true,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Tile::StoneWall,
            '.' => Tile::OpenPassage,
            ' ' => Tile::OpenPassage,
            '@' => Tile::Entrance,
            k @ 'a'..='z' => Tile::Key(KeyId::from(k)),
            d @ 'A'..='Z' => Tile::Door(KeyId::from(d)),
            c => panic!("Unexpected map character: {}", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Tile::Entrance => f.write_str("@"),
            Tile::OpenPassage => f.write_str(" "),
            Tile::StoneWall => f.write_str("#"),
            Tile::Door(d) => f.write_char(d.to_door_char()),
            Tile::Key(k) => f.write_char(k.to_key_char()),
        }
    }
}

pub struct World {
    pub map: WorldMap<Tile>,
    pub keys: KeyRing,
}

impl World {}

impl From<&String> for World {
    fn from(s: &String) -> World {
        let map = WorldMap::from_iter(
            s.split("\n")
                .enumerate()
                .map(|(y, line)| {
                    line.chars().enumerate().map(move |(x, t)| {
                        let coord = (x as i32, y as i32);
                        (coord, Tile::from(t))
                    })
                })
                .flatten(),
        );

        let keys = KeyRing::from(s.chars().filter(|c| c.is_ascii_lowercase()));

        World {
            map: map,
            keys: keys,
        }
    }
}
