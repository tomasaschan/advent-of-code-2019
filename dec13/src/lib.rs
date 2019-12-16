use colored::*;
use intcode::Builder;
use std::collections::HashMap;

pub fn solve_a(input: &String) -> usize {
    let screen = run_demo(&input);
    screen.values().filter(|t| **t == 2).count()
}

pub fn solve_b(input: &String) -> i128 {
    let mut current_score = 0;
    let mut paddle_x = -1;
    let mut ball_x = -1;

    let (input, output) = Builder::new()
        .parse(input)
        .init_hook(vec![3, 0])
        .input_hook(vec![104, 98])
        .run();
    input.send(2).unwrap();

    loop {
        let a = match output.recv() {
            Ok(a) => a,
            Err(_) => break current_score,
        };

        if a != 98 {
            let x = a;
            let y = output.recv().unwrap();
            let c = output.recv().unwrap();
            if (x, y) == (-1, 0) {
                current_score = c;
            } else {
                if c == 3 {
                    paddle_x = x;
                } else if c == 4 {
                    ball_x = x;
                }
            }
        } else {
            input.send((ball_x - paddle_x).signum()).unwrap();
        }
    }
}

fn interpret(c: i128) -> colored::ColoredString {
    match c {
        0 => " ".clear(),
        1 => "#".purple(),
        2 => "#".green(),
        3 => "-".bright_blue(),
        4 => "o".yellow(),
        s => panic!("Unknown tile type {}", s),
    }
}

pub fn run_demo(input: &String) -> HashMap<(i128, i128), i128> {
    let mut display = HashMap::new();

    let (_, output) = Builder::new().parse(input).run();

    let outputs = output.iter().collect::<Vec<i128>>();

    for instruction in outputs.chunks(3) {
        match instruction {
            [x, y, c] => {
                display.insert((*x, *y), *c);
            }
            i => panic!("invalid display instruction! {:?}", i),
        }
    }

    display
}

pub fn show(display: &HashMap<(i128, i128), i128>) {
    let (mut xmin, mut xmax, mut ymin, mut ymax) = (0, 0, 0, 0);

    for ((x, y), _) in display.iter() {
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

    print!("{}", "".clear());
    print!("{}[2J", 27 as char);
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let tile = match display.get(&(x, y)) {
                None => " ".clear(),
                Some(c) => interpret(*c),
            };
            print!("{}", tile);
        }
        println!();
    }
    print!("{}", "".clear());
}
