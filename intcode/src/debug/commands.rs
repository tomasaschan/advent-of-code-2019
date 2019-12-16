use super::debugger::Debugger;

pub fn handle(input: &String, debugger: &mut Debugger) {
    if input.trim().len() == 0 {
        return;
    }
    let cmd: Vec<&str> = input.split_whitespace().collect();
    match cmd[0] {
        "l" | "load" => debugger.load(&cmd[1..]),
        "h" | "hook" => debugger.hook(&cmd[1..]),
        "reset" => debugger.reset(&cmd[1..]),
        "s" | "show" => debugger.show(&cmd[1..]),
        "n" | "next" => debugger.next(&cmd[1..]),
        "in" | "input" => debugger.provide_input(&cmd[1..]),
        _ => println!("Unknown command {}", cmd[0]),
    }
}
