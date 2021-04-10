use util::io::get_input;

mod console;
mod gamer;

use console::Console;

fn main() {
    let input = get_input();

    let console = Console::start_game(&input, true);

    gamer::play_game(&console);
    //println!("b: {}", solve_b(&input));
}
