pub mod io {
    use std::io;
    use std::io::prelude::*;

    pub fn get_input() -> String {
        let mut input = String::new();

        io::stdin()
            .read_to_string(&mut input)
            .expect("Could not read input from stdin!");
        return input.trim().to_string();
    }
}
