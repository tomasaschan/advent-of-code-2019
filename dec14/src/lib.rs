use std::collections::HashMap;

type Recipie<'a> = (usize, Vec<(&'a str, usize)>);
type Cookbook<'a> = HashMap<&'a str, Recipie<'a>>;

pub fn solve_a(input: &String) -> usize {
    let cookbook = parse(&input);
    let mut needs = HashMap::<&str, usize>::new();
    let mut extras = HashMap::<&str, usize>::new();
    produce_1_fuel(&mut needs, &mut extras, &cookbook);
    *needs.get("ORE").unwrap()
}

pub fn solve_b(input: &String) -> usize {
    let one_trillion = 1000000000000;
    let cookbook = parse(&input);
    let mut needs = HashMap::<&str, usize>::new();
    let mut extras = HashMap::<&str, usize>::new();

    // Produce 1 FUEL to see how much ORE we need for that
    produce_1_fuel(&mut needs, &mut extras, &cookbook);
    let ore = needs.remove("ORE").unwrap();

    assert!(needs.is_empty());

    // Do that for as many times as we can, until we don't have enough ORE to produce one more
    let mut fuel = one_trillion / ore;
    for (_, c) in extras.iter_mut() {
        *c *= fuel;
    }
    // We'll have some ORE left over within the 1 trillion allowed
    *extras.entry("ORE").or_default() += one_trillion - fuel * ore;

    // Produce more fule using the extra resources, until we need more ORE
    let answer = loop {
        produce_1_fuel(&mut needs, &mut extras, &cookbook);
        if needs.contains_key("ORE") {
            break fuel;
        } else {
            assert!(needs.is_empty());
            fuel += 1;
        }
    };
    answer
}

fn produce_1_fuel<'a>(
    needs: &mut HashMap<&'a str, usize>,
    extras: &mut HashMap<&'a str, usize>,
    cookbook: &'a Cookbook,
) {
    needs.insert("FUEL", 1);

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
