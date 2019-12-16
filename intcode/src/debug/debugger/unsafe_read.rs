use super::super::super::IntcodeComputer;

impl IntcodeComputer {
    pub fn unsafe_read(&self, i: i128) -> i128 {
        self.memory.get(i)
    }
}
