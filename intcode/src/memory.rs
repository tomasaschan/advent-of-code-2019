use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct Memory {
    data: HashMap<i128, i128>,
}
// impl Debug for Memory {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         let write_keys: std::result::Result<(), std::fmt::Error> = keys
//             .iter()
//             .map(|k| f.write_str(&format!("{:>6}", k)))
//             .collect();
//         write_keys.and(f.write_str("\n")).and(
//             keys.iter()
//                 .map(|k| f.write_str(&format!("{:>6}", self.data[k])))
//                 .collect(),
//         )
//     }
// }

impl Memory {
    pub const RELATIVE_BASE: i128 = -1;
    pub const JUMPBACK_PTR: i128 = -2;
    // Relative base pointer is a (only?) location we can be 100% sure won't be used as an instruction ptr
    pub const JUMPBACK_PTR_UNSET_SENTINEL: i128 = Memory::RELATIVE_BASE;
    pub const INIT_HOOK_PTR: i128 = -3;
    pub const EXIT_HOOK_PTR: i128 = -4;
    pub const EXIT_HOOK_RUN: i128 = -5;
    pub const INPUT_HOOK_PTR: i128 = -6;
    pub const LAST_INPUT_INSTR: i128 = -7;
    pub const PADDING: i128 = 7;

    pub fn new() -> Memory {
        Memory {
            data: HashMap::new(),
        }
    }

    pub fn build(
        program: &Vec<i128>,
        init_hook: &Option<Vec<i128>>,
        exit_hook: &Option<Vec<i128>>,
        input_hook: &Option<Vec<i128>>,
    ) -> Memory {
        let mut data: HashMap<i128, i128> = program
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i as i128, *x))
            .collect();

        // Relative pointer base
        data.insert(Memory::RELATIVE_BASE, 0);
        // Jumpback pointer, for executing hooks mid-program
        data.insert(Memory::JUMPBACK_PTR, Memory::JUMPBACK_PTR_UNSET_SENTINEL);

        match init_hook {
            Some(hook) => {
                let loc = -Memory::PADDING - (hook.len() as i128);
                data.insert(Memory::INIT_HOOK_PTR, loc);
                Memory::insert_hook(&mut data, loc, &mut hook.clone());
            }
            None => {} // no-op
        }

        match exit_hook {
            Some(hook) => {
                let loc = data
                    .get(&Memory::INIT_HOOK_PTR)
                    .unwrap_or(&-Memory::PADDING)
                    - (hook.len() as i128);
                data.insert(Memory::EXIT_HOOK_PTR, loc);
                Memory::insert_hook(&mut data, loc, &mut hook.clone());
            }
            None => {} // no-op
        }

        match input_hook {
            Some(hook) => {
                let loc = data
                    .get(&Memory::EXIT_HOOK_PTR)
                    .or(data.get(&Memory::INIT_HOOK_PTR))
                    .unwrap_or(&-Memory::PADDING)
                    - (hook.len() as i128);
                data.insert(Memory::INPUT_HOOK_PTR, loc);
                Memory::insert_hook(&mut data, loc, &mut hook.clone());
            }
            None => {} //no-op
        }

        Memory { data }
    }

    pub fn get(&self, i: i128) -> i128 {
        *self.data.get(&i).unwrap_or(&0)
    }

    pub fn set(&mut self, i: i128, v: i128) {
        self.data.insert(i, v);
    }

    fn insert_hook(data: &mut HashMap<i128, i128>, loc: i128, hook: &mut Vec<i128>) {
        for (p, i) in hook
            .into_iter()
            .enumerate()
            .map(|(p, i)| (p as i128 + loc, *i))
        {
            data.insert(p, i);
        }
    }

    pub fn dump(&self) {
        let mut buffer =
            File::create("core.dump").expect("core dump failed; could not create file core.dump.");
        let mut keys: Vec<&i128> = self.data.keys().collect();
        keys.sort();

        for key in keys {
            writeln!(buffer, "{:<6} {:>6}", key, self.data[key]).unwrap();
        }
    }
}
