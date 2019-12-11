use num::integer::gcd;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve_a(input: &String) -> usize {
    let map = build_map(input);
    let other = map.clone();
    map.into_iter()
        .map(|a| seen_by(a, &other).len())
        .max()
        .unwrap()
}

fn find_station(map: &HashSet<(i32, i32)>) -> (i32, i32) {
    let other = map.clone();
    *map.into_iter()
        .max_by_key(|a| seen_by(**a, &other).len())
        .unwrap()
}

fn seen_by(station: (i32, i32), map: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let cl = map.clone();
    map.into_iter()
        .filter_map(|asteroid| {
            if *asteroid != station && sees(station, *asteroid, &cl) {
                Some(*asteroid)
            } else {
                None
            }
        })
        .collect()
}

pub fn solve_b(input: &String) -> i32 {
    let map = build_map(input);

    let station = find_station(&map);

    let mut edges: Vec<(i32, i32)> = (&map)
        .into_iter()
        .filter(|asteroid| **asteroid != station && sees(station, **asteroid, &map))
        .map(|a| (a.0 - station.0, a.1 - station.1))
        .collect();

    edges.sort_by(compare);

    let asteroids: HashMap<(i32, i32), usize> = (&edges)
        .into_iter()
        .map(|(dx, dy)| (station.0 + dx, station.1 + dy))
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();

    let twohundredth = (&asteroids)
        .into_iter()
        .filter(|(_, i)| *i + &1 == 200)
        .nth(0)
        .unwrap()
        .0;

    twohundredth.0 * 100 + twohundredth.1 * 1
}

fn build_map(input: &String) -> HashSet<(i32, i32)> {
    HashSet::from_iter(
        input
            .split_terminator("\n")
            .enumerate()
            .flat_map(move |(y, row)| row.chars().enumerate().map(move |(x, c)| ((x, y), c)))
            .filter(move |(_, c)| c == &'#')
            .map(move |((x, y), _)| (x as i32, y as i32)),
    )
}

fn step(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);

    if dx == 0 {
        (0, dy / dy.abs())
    } else if dy == 0 {
        (dx / dx.abs(), 0)
    } else {
        assert!(dx != 0, "dx was zero!");
        assert!(dy != 0, "dy was zero!");
        let d = gcd(dx, dy);
        (dx / d, dy / d)
    }
}

fn sees(a: (i32, i32), b: (i32, i32), asteroids: &HashSet<(i32, i32)>) -> bool {
    let s = step(a, b);

    let (mut x, mut y) = (a.0 + s.0, a.1 + s.1);
    while (x, y) != b {
        if asteroids.contains(&(x, y)) {
            return false;
        }
        x += s.0;
        y += s.1;
    }
    true
}

fn compare((ax, ay): &(i32, i32), (bx, by): &(i32, i32)) -> Ordering {
    // if a is on the y-axis...
    if ax == &0 {
        // ...above the x-axis...
        if ay < &0 {
            if bx == &0 && by < &0 {
                // ...b on the upward y-axis is equal...
                Ordering::Equal
            } else {
                // ...any other b is after
                Ordering::Less
            }
        }
        // ...below the x-axis
        else {
            // ...b to the right or on the upward y-axis is before
            if bx > &0 || (bx == &0 && by > &0) {
                Ordering::Greater
            }
            // ...b on the downward y-axis is equal
            else if bx == &0 && by > &0 {
                Ordering::Equal
            }
            // ...b to the left is after
            else {
                Ordering::Less
            }
        }
    // if b is on the y-axis
    } else if bx == &0 {
        // ...above the x-axis...
        if by < &0 {
            // ...a on the upward y-axis is equal...
            if ax == &0 && ay < &0 {
                Ordering::Equal
            }
            // ...any other a is after
            else {
                Ordering::Greater
            }
        }
        // ...below the x-axis
        else {
            // ...a to the right or on the upward y-axis is before
            if ax > &0 || ax == &0 && ay < &0 {
                Ordering::Less
            }
            // ...a on the negative y-axis is equal
            else if ax == &0 && ay > &0 {
                Ordering::Equal
            }
            // ...a to the left is after
            else {
                Ordering::Greater
            }
        }
    }
    // if both a and b are on the positive side of the y-axis...
    else if ax > &0 && bx > &0 {
        // ...compare atan "backwards"
        (*ay as f32 / (*ax as f32))
            .partial_cmp(&(*by as f32 / (*bx as f32)))
            .unwrap_or(Ordering::Equal)
    }
    // if both a and b are on the negative side of the y-axis...
    else if ax < &0 && bx < &0 {
        // ...compare atan "forwards"
        (*ay as f32 / (*ax as f32))
            .partial_cmp(&(*by as f32 / (*bx as f32)))
            .unwrap_or(Ordering::Equal)
    }
    // if a and b are on different sides of the y-axis...
    else if ax * bx < 0 {
        // ...the one on the positive side is before
        (-ax.signum()).cmp(&-(bx.signum()))
    } else {
        Ordering::Equal
    }
}
