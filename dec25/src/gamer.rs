use super::console::Console;

pub fn play_game(console: &Console) {
    take_all_items_to_checkpoint(&console);
    go_through_checkpoint(&console);
}

fn take_all_items_to_checkpoint(console: &Console) {
    // these were determined as the shortest path to pick up all items
    // by manually exploring the cave
    for cmd in vec![
        "east",
        "take weather machine",
        "west",
        "west",
        "west",
        "take bowl of rice",
        "east",
        "north",
        "take polygon",
        "east",
        "take hypercube",
        "south",
        "take dark matter",
        "north",
        "west",
        "north",
        "take candy cane",
        "west",
        "north",
        "take manifold",
        "south",
        "west",
        "north",
        "take dehydrated water",
        "west",
    ] {
        let _o = console.send_command(cmd.to_string());
    }
}

fn carried_items(console: &Console) -> Vec<String> {
    let mut items = vec![];
    for line in console.send_command("inv".to_string()).lines() {
        if line.starts_with("- ") {
            items.push(line.trim_start_matches("- ").to_string());
        }
    }
    items
}

fn drop_all_items(console: &Console) {
    for item in carried_items(&console) {
        console.send_command(format!("drop {}", item));
    }
}

fn go_through_checkpoint(console: &Console) {
    let all_items = carried_items(&console);

    drop_all_items(&console);

    let code = test_all_combos(
        &console,
        vec![vec![]]
            .into_iter()
            .chain(combos_of_one(&all_items))
            .chain(combos_of_two(&all_items))
            .chain(combos_of_three(&all_items))
            .chain(combos_of_four(&all_items))
            .collect(),
    );

    println!("airlock code: {:?}", code.unwrap());
}

fn attempt(console: &Console, items: &Vec<String>) -> Option<usize> {
    drop_all_items(&console);
    for item in items {
        console.send_command(format!("take {}", item));
    }

    let output = console.send_command("south".to_string());
    for line in output.lines() {
        if line.contains("Alert! Droids on this ship are") {
            let heavier = line.contains("heavier");
            let lighter = line.contains("lighter");
            if heavier {
                return None;
            } else if lighter {
                return None;
            }
        }
        if line.contains("Oh, hello!") {
            let digits = line
                .trim_start_matches("\"Oh, hello! You should be able to get in by typing ")
                .split(" ")
                .collect::<Vec<&str>>()[0];
            return Some(digits.parse().unwrap());
        }
    }
    panic!("wat")
}

fn test_all_combos(console: &Console, combos: Vec<Vec<String>>) -> Option<usize> {
    for combo in combos {
        if let Some(code) = attempt(&console, &combo) {
            return Some(code);
        }
    }
    None
}
fn combos_of_one(items: &Vec<String>) -> Vec<Vec<String>> {
    let mut combos = vec![];
    for item in items {
        combos.push(vec![item.clone()]);
    }
    combos
}

fn combos_of_two(items: &Vec<String>) -> Vec<Vec<String>> {
    let mut combos = vec![];
    for i in 0..items.len() {
        for j in i + 1..items.len() {
            combos.push(vec![items[i].clone(), items[j].clone()]);
        }
    }
    combos
}

fn combos_of_three(items: &Vec<String>) -> Vec<Vec<String>> {
    let mut combos = vec![];
    for i in 0..items.len() {
        for j in i + 1..items.len() {
            for k in j + 1..items.len() {
                combos.push(vec![items[i].clone(), items[j].clone(), items[k].clone()])
            }
        }
    }
    combos
}

fn combos_of_four(items: &Vec<String>) -> Vec<Vec<String>> {
    let mut combos = vec![];

    for i in 0..items.len() {
        for j in i + 1..items.len() {
            for k in j + 1..items.len() {
                for l in k + 1..items.len() {
                    combos.push(vec![
                        items[i].clone(),
                        items[j].clone(),
                        items[k].clone(),
                        items[l].clone(),
                    ])
                }
            }
        }
    }
    combos
}
