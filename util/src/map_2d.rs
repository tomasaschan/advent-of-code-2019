use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}
