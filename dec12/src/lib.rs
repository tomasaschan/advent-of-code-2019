#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate regex;

use regex::Regex;

use num::integer::lcm;
use std::ops::{Add, AddAssign};
use std::str::FromStr;
use std::string::ToString;

pub fn solve_a(input: &String) -> i32 {
    let mut moons = parse(input);
    simulate(&mut moons, 1000);
    tot_energy(&moons)
}

pub fn solve_b(input: &String) -> u128 {
    let moons = parse(input);

    let mut periods: [u128; 3] = [0, 0, 0];

    for i in 0..3 {
        let (xs, vs) = dir(i, &moons);
        periods[i] = find_period(&xs, &vs);
    }
    lcm(periods[0], lcm(periods[1], periods[2]))
}

fn dir(i: usize, moons: &[Moon; 4]) -> ([i32; 4], [i32; 4]) {
    let mut ps = [0, 0, 0, 0];
    let mut vs = [0, 0, 0, 0];
    for m in 0..4 {
        ps[m] = moons[m].p.0[i];
        vs[m] = moons[m].v.0[i];
    }
    (ps, vs)
}

fn find_period(xs_init: &[i32; 4], vs_init: &[i32; 4]) -> u128 {
    let (mut xs, mut vs): ([i32; 4], [i32; 4]) = ([0, 0, 0, 0], [0, 0, 0, 0]);
    xs.clone_from_slice(xs_init);
    vs.clone_from_slice(vs_init);
    let mut steps = 0;

    loop {
        steps += 1;
        let mut fs: [i32; 4] = [0, 0, 0, 0];
        for j in 0..4 {
            for i in 0..4 {
                fs[j] += (xs[i] - xs[j]).signum();
            }
        }
        for i in 0..4 {
            vs[i] += fs[i];
            xs[i] += vs[i];
        }

        if &xs == xs_init && &vs == vs_init {
            break steps;
        }
    }
}

fn parse(input: &String) -> [Moon; 4] {
    let mut moons: [Moon; 4] = [Moon::empty(), Moon::empty(), Moon::empty(), Moon::empty()];
    match input
        .split('\n')
        .map(Moon::from_str)
        .collect::<Result<Vec<Moon>, <Moon as FromStr>::Err>>()
    {
        Ok(ms) => moons.copy_from_slice(&ms[0..4]),
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
    moons
}

fn apply_gravity(moons: &mut [Moon; 4]) {
    for b in moons.clone().iter() {
        for a in moons.iter_mut() {
            a.f += V([
                (b.p.0[0] - a.p.0[0]).signum(),
                (b.p.0[1] - a.p.0[1]).signum(),
                (b.p.0[2] - a.p.0[2]).signum(),
            ]);
        }
    }
    for m in moons.iter_mut() {
        m.v += m.f;
        m.f = V::zero();
    }
}

fn apply_time(moons: &mut [Moon; 4]) {
    for m in moons.iter_mut() {
        m.p += m.v;
    }
}

fn step(mut moons: &mut [Moon; 4]) {
    apply_gravity(&mut moons);
    apply_time(&mut moons);
}

fn simulate(mut moons: &mut [Moon; 4], time: usize) {
    for _ in 1..=time {
        step(&mut moons);
    }
}

fn tot_energy(moons: &[Moon; 4]) -> i32 {
    moons
        .iter()
        .map(|m| {
            let pot = m.p.0[0].abs() + m.p.0[1].abs() + m.p.0[2].abs();
            let kin = m.v.0[0].abs() + m.v.0[1].abs() + m.v.0[2].abs();
            pot * kin
        })
        .sum()
}

#[derive(Debug, Clone, Copy, Hash)]
struct V([i32; 3]);

impl PartialEq for V {
    fn eq(&self, other: &V) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1] && self.0[2] == other.0[2]
    }
}
impl Eq for V {}
impl V {
    pub fn zero() -> V {
        V([0, 0, 0])
    }
}

#[derive(Debug, Clone, Copy)]
struct Moon {
    p: V,
    v: V,
    f: V,
}

impl Moon {
    pub fn empty() -> Moon {
        Moon {
            p: V([0, 0, 0]),
            v: V([0, 0, 0]),
            f: V([0, 0, 0]),
        }
    }
}

impl FromStr for Moon {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("<x=(-?\\d+), y=(-?\\d+), z=(-?\\d+)>").unwrap();
        }

        match RE
            .captures_iter(s)
            .map(|c| {
                let x = c[1].parse()?;
                let y = c[2].parse()?;
                let z = c[3].parse()?;
                Ok(V([x, y, z]))
            })
            .collect::<Result<Vec<V>, std::num::ParseIntError>>()
        {
            Ok(v) => Ok(Moon {
                p: v.get(0).expect("no matches").clone(),
                v: V([0, 0, 0]),
                f: V([0, 0, 0]),
            }),
            Err(_) => Err("Failed to parse"),
        }
    }
}

impl ToString for Moon {
    fn to_string(&self) -> String {
        format!("pos={}, vel={}", self.p.to_string(), self.v.to_string())
    }
}
impl ToString for V {
    fn to_string(&self) -> String {
        format!(
            "<x={:>3}, y={:>3}, z={:>3}>",
            self.0[0], self.0[1], self.0[2]
        )
    }
}

impl Add for V {
    type Output = V;

    fn add(self, other: V) -> Self::Output {
        V([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl AddAssign for V {
    fn add_assign(&mut self, other: V) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
        self.0[2] += other.0[2];
    }
}
