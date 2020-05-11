use std::{collections::HashMap, iter::FromIterator};

type Recipie<'a> = (usize, Vec<(&'a str, usize)>);
type Cookbook<'a> = HashMap<&'a str, Recipie<'a>>;

pub fn solve_a(input: &String) -> usize {
    let cookbook = parse(&input);
    let mut extras = HashMap::new();
    produce(
        &mut HashMap::<&str, usize>::from_iter(vec![("FUEL", 1)]),
        &mut extras,
        &cookbook,
    )
}

pub fn solve_b(input: &String) -> usize {
    let one_trillion: usize = 1_000_000_000_000;
    let cookbook = parse(&input);
    let mut extras = HashMap::<&str, usize>::new();

    let ore_for_one = produce(
        &mut HashMap::<&str, usize>::from_iter(vec![("FUEL", 1)]),
        &mut extras,
        &cookbook,
    );

    let ore_for_extas = {
        let mut ore = 0;
        loop {
            let mut extra_extras = HashMap::new();
            println!(
                "checking cost for producing {} more things...",
                extras.len()
            );
            ore += produce(&mut extras, &mut extra_extras, &cookbook);
            break ore;
            println!(
                "cost was {}, produced {} extra things",
                ore,
                extra_extras.len()
            );
            break 0;
            if extra_extras.is_empty() {
                break 0;
            } else {
                extras = extra_extras.clone();
            }
        }
    };

    one_trillion / (ore_for_one - ore_for_extas)
}

fn produce<'a>(
    needs: &mut HashMap<&'a str, usize>,
    extras: &mut HashMap<&'a str, usize>,
    cookbook: &'a Cookbook,
) -> usize {
    while let Some((next, needed)) = match needs.iter_mut().filter(|(k, _)| *k != &"ORE").next() {
        Some((next, q)) => Some((*next, *q)),
        None => None,
    } {
        needs.remove(next);
        let (yld, recipie) = cookbook.get(next).unwrap();
        let have = extras.remove(next).unwrap_or(0);

        let mut multiplier = 0;
        while multiplier * yld + have < needed {
            multiplier += 1;
            if multiplier > 10000 {
                panic!(
                    "need {} {}, have {}, recipie is {:?}. Multiplier too large!",
                    needed, next, have, recipie
                );
            }
        }
        if multiplier > 0 {
            for (ingredient, qqq) in recipie {
                *needs.entry(ingredient).or_default() += multiplier * *qqq;
            }
        }
        *extras.entry(next).or_default() = multiplier * yld + have - needed;
    }
    let ore = needs.remove("ORE").unwrap_or_default();
    assert!(needs.is_empty());
    ore
}

fn parse(input: &String) -> Cookbook {
    input.split("\n").map(parse_recipie).collect()
}

fn parse_recipie(line: &str) -> (&str, Recipie) {
    let recipie = line.split(" => ").collect::<Vec<&str>>();
    let ingredients = recipie[0].split(", ").map(parse_part).collect();
    let (product, yld) = parse_part(recipie[1]);
    (product, (yld, ingredients))
}

fn parse_part(part: &str) -> (&str, usize) {
    let parts: Vec<&str> = part.split(" ").collect();
    (parts[1], parts[0].parse().unwrap())
}
