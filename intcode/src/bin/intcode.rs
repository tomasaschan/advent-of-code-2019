use clap::{App, Arg};
use intcode::{ops::read::EmptyInputBehavior, parser, IntcodeComputer};
use std::{sync::mpsc::channel, thread::spawn};

pub fn main() {
    let parsed = App::new("Intcode Computer")
        .version("1.0")
        .arg(Arg::with_name("ascii").long("ascii"))
        .arg(
            Arg::with_name("program")
                .short("p")
                .long("program")
                .value_name("program")
                .required(true),
        )
        .get_matches();

    match (parsed.value_of("program"), parsed.is_present("ascii")) {
        (Some(program_file), ascii) => run(program_file, ascii).unwrap(),
        (None, _) => {
            println!("{}", parsed.usage());
        }
    }
}

fn run(program_file: &str, ascii: bool) -> Result<(), String> {
    let file_contents = std::fs::read_to_string(program_file).map_err(|e| e.to_string())?;
    let program = parser::parse(&file_contents.trim())?;

    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();

    let mut computer = IntcodeComputer::new(
        &program,
        None,
        None,
        None,
        in_rx,
        out_tx,
        false,
        EmptyInputBehavior::BlockWithPrompt,
    );

    let computer_h = spawn(move || computer.run_to_halt());

    let output_h = spawn(move || {
        for i in out_rx.iter() {
            if ascii {
                match std::char::from_u32(i as u32) {
                    Some(c) => print!("{}", c),
                    None => print!("{}", i),
                }
            } else {
                print!("{}", i);
            }
        }
    });

    spawn(move || loop {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .expect("Failed to read from stdin!");
        if s == "" {
            break;
        }
        for i in interpret_input(&s, ascii) {
            in_tx.send(i).expect("Failed to send input to channel!");
        }
    });
    computer_h.join().map_err(|e| format!("{:?}", e))?;
    println!("(intcode computer halted)");
    output_h.join().map_err(|e| format!("{:?}", e))?;

    println!();
    Ok(())
}

fn interpret_input(s: &String, ascii: bool) -> Vec<i128> {
    if ascii {
        s.chars().map(|c| c as i128).collect()
    } else {
        let i = s
            .parse()
            .expect(&format!("Failed to parse integer from {}", s));
        vec![i]
    }
}
