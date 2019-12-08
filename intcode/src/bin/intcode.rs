use rustyline;
use rustyline::error::ReadlineError;

pub fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline("dbg> ") {
            Ok(cmd) => println!("got {}", cmd),
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
