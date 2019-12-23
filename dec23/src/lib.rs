use intcode::Builder;
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

pub fn solve_a(input: &String) -> i128 {
    let mut connections = HashMap::new();

    for i in 0..50 {
        let (input, output) = Builder::new()
            .parse(&input)
            .initial_input(vec![i])
            .input_default(-1)
            .silent()
            .run();
        connections.insert(i, (input, output));
    }

    read_y_to_255(&connections)
}

fn read_y_to_255(connections: &HashMap<i128, (Sender<i128>, Receiver<i128>)>) -> i128 {
    loop {
        for i in 0..50 {
            let (_, output) = &connections[&i];
            if let Some(addr) = match output.try_recv() {
                Ok(value) => Some(value),
                Err(_) => None,
            } {
                let x = output.recv().unwrap();
                let y = output.recv().unwrap();

                if addr == 255 {
                    return y;
                } else if let Some((input, _)) = connections.get(&addr) {
                    input.send(x).unwrap();
                    input.send(y).unwrap();
                }
            }
        }
    }
}
