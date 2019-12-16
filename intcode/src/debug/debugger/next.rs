use super::{super::super::Memory, Debugger};
use queues::IsQueue;

impl Debugger {
    pub fn next(&mut self, _args: &[&str]) {
        if self.computer.instruction_pointer == 0 && !self.has_run_init_hook {
            self.computer
                .load_hook(self.computer.memory.get(Memory::INIT_HOOK_PTR));
            self.has_run_init_hook = true;
        } else if self.computer.memory.get(self.computer.instruction_pointer) == 99 {
            self.computer.handle_halt_hooks();
            if self.computer.memory.get(self.computer.instruction_pointer) == 99 {
                println!("Compuer is halted!");
            }
            return;
        } else if self.computer.memory.get(self.computer.instruction_pointer) % 100 == 3 {
            if self.input_queue.size() == 0 {
                self.prompt_for_input();
            }

            let input = self.input_queue.remove().unwrap();
            match self.input_sender.send(input) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error sending input to intcode computer: {:?}", e);
                    return;
                }
            }
        }

        self.computer.step();
    }
}
