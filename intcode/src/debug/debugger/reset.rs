use super::Debugger;

impl Debugger {
    pub fn reset(&mut self, _: &[&str]) {
        self.computer.reset()
    }
}
