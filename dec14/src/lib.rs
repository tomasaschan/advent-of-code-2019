use queues::{IsQueue, Queue};
use std::cmp::max;
use std::collections::HashMap;

type Substance = String;
type Quantity = i128;
type Recipie = (Quantity, Vec<(Substance, Quantity)>);
type Cookbook = HashMap<Substance, Recipie>;
type Inventory = HashMap<Substance, Quantity>;
const ONE_TRILLION: Quantity = 1_000_000_000_000;

pub fn solve_a(input: &String) -> Quantity {
    let cookbook = parse(&input);
    ore_for_fuel(1, &cookbook)
}

pub fn solve_b(input: &String) -> Quantity {
    let cookbook = parse(&input);
    maximize_fuel(&cookbook)
}

fn fuel() -> Substance {
    Substance::from("FUEL")
}
fn ore() -> Substance {
    Substance::from("ORE")
}

fn ore_for_fuel(amount: Quantity, cookbook: &Cookbook) -> Quantity {
    let mut inventory = Inventory::new();
    let mut queue = Queue::<Substance>::new();

    queue.add(fuel()).unwrap();
    *inventory.entry(fuel()).or_default() -= amount;

    while let Ok(next) = queue.remove() {
        if next == ore() {
            continue;
        }
        let stock = inventory.remove(&next).unwrap_or_default();
        let (yld, recipie) = cookbook.get(&next).unwrap();

        let multiplier = max(0, ((-stock as f64) / (*yld as f64)).ceil() as i128);
        for (ingredient, quantity) in recipie {
            queue.add(ingredient.clone()).unwrap();
            *inventory.entry(ingredient.clone()).or_default() -= multiplier * quantity;
        }
        inventory.insert(next, stock + yld * multiplier);
    }

    let ore_needed = -inventory.remove(&ore()).unwrap_or_default();
    assert!(inventory
        .iter()
        .all(|(ingredient, stock)| stock >= &0 || *ingredient == ore()));
    ore_needed
}

fn maximize_fuel<'a>(cookbook: &Cookbook) -> Quantity {
    // 1. Find bounds
    let mut upper = 1_000;
    while ore_for_fuel(upper, cookbook) < ONE_TRILLION {
        upper *= 2;
    }
    let mut lower = upper / 2;
    let mut mid = (upper + lower + 1) / 2;

    // 2. Binary search
    while upper > lower {
        if ore_for_fuel(mid, cookbook) > ONE_TRILLION {
            upper = mid - 1;
            mid = (upper + lower + 1) / 2;
        } else {
            lower = mid;
            mid = (upper + lower + 1) / 2;
        }
    }

    upper
}

fn parse(input: &Substance) -> Cookbook {
    input.split("\n").map(parse_recipie).collect()
}

fn parse_recipie(line: &str) -> (Substance, Recipie) {
    let recipie = line
        .split(" => ")
        .map(Substance::from)
        .collect::<Vec<Substance>>();
    let ingredients = recipie[0]
        .split(", ")
        .map(Substance::from)
        .map(parse_part)
        .collect();
    let (product, yld) = parse_part(recipie[1].clone());
    (product, (yld, ingredients))
}

fn parse_part(part: Substance) -> (Substance, Quantity) {
    let parts: Vec<Substance> = part.split(" ").map(Substance::from).collect();
    (parts[1].clone(), parts[0].parse().unwrap())
}
