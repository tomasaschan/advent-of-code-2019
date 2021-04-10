use intcode::*;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};

pub struct Console {
    input: Sender<String>,
    output: Receiver<String>,
}
impl Console {
    pub fn start_game(program: &String, silent: bool) -> Console {
        let (input_tx, input_rx) = channel::<String>();
        let (output_tx, output_rx) = channel::<String>();

        let (in_tx, out_rx) = Builder::new().parse(&program).block_on_input().run();

        std::thread::spawn(move || {
            while let Ok(line) = input_rx.recv() {
                for c in line.chars() {
                    if !silent {
                        print!("{}", c);
                    }
                    in_tx.send(c as i128).unwrap();
                }
                in_tx.send('\n' as i128).unwrap();
            }
        });

        std::thread::spawn(move || {
            let mut s = String::new();
            while let Ok(i) = out_rx.recv() {
                let c = std::char::from_u32(i as u32).unwrap();
                s += &format!("{}", c);
                if c == '\n' {
                    if !silent {
                        print!("{}", s);
                    }
                    output_tx.send(s).unwrap();
                    s = String::new();
                }
            }
        });

        loop {
            let o = output_rx.recv().unwrap();
            if !silent {
                print!("{}", o);
            }
            if o.trim_end() == "Command?" {
                break;
            }
        }

        Console {
            input: input_tx,
            output: output_rx,
        }
    }

    pub fn send_command(&self, cmd: String) -> String {
        let mut output = String::new();
        self.input.send(cmd).unwrap();

        loop {
            match self.output.recv() {
                Ok(o) => {
                    output += &o;
                    if o.trim_end() == "Command?" {
                        break;
                    }
                }
                Err(RecvError) => break,
            }
        }
        output
    }
}
