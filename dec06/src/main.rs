use util::io::get_input;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = get_input();

    println!("a: {}", solve_a(&parse(input.clone())));
    println!("b: {}", solve_b(&parse(input.clone())));
}

fn solve_a(orbits: &HashMap<String, String>) -> u32 {
    orbits
        .keys()
        .map(|orbiter| orbit_checksum(orbiter, orbits))
        .sum()
}

fn solve_b(orbits: &HashMap<String, String>) -> i32 {
    let mut pos_me = "YOU";
    let mut pos_st = "SAN";
    let mut from_me: HashSet<String> = HashSet::new();
    let mut from_st: HashSet<String> = HashSet::new();

    loop {
        if let Some(inflection_pt) = from_me.intersection(&from_st).next() {
            break steps_to("YOU".to_string(), inflection_pt.to_string(), orbits) - 1
                + steps_to("SAN".to_string(), inflection_pt.to_string(), orbits)
                - 1;
        }
        if let Some(next) = orbits.get(pos_me) {
            from_me.insert(next.to_string());
            pos_me = next;
        }
        if let Some(next) = orbits.get(pos_st) {
            from_st.insert(next.to_string());
            pos_st = next;
        }
    }
}

fn orbit_checksum(orbiter: &String, orbits: &HashMap<String, String>) -> u32 {
    if let Some(next) = orbits.get(orbiter) {
        return 1 + orbit_checksum(next, orbits);
    } else {
        return 0;
    }
}

fn steps_to(orbiter: String, orbitee: String, orbits: &HashMap<String, String>) -> i32 {
    if orbiter == orbitee {
        0
    } else if let Some(next) = orbits.get(&orbiter.to_string()) {
        1 + steps_to(next.to_string(), orbitee, orbits)
    } else {
        panic!("asked for path between two stations that do not orbit each-other!");
    }
}

fn parse(input: String) -> HashMap<String, String> {
    let mut orbits = HashMap::new();
    for line in input.split("\n") {
        if let Some(delim) = line.find(')') {
            let orbitee: String = (&line[0..delim]).to_owned();
            let orbiter: String = (&line[delim + 1..]).to_owned();
            assert!(
                !orbits.contains_key(&orbiter),
                format!("{} is already in orbit!", orbiter)
            );
            orbits.insert(orbiter, orbitee);
        }
    }
    orbits
}
