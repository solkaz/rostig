#[path = "./cpu.rs"]
pub mod cpu;
#[path = "./memory.rs"]
pub mod memory;
#[path = "./registers.rs"]
pub mod registers;

pub struct Game {
  pub cpu: cpu::Cpu,
  pub memory: memory::Memory,
  pub registers: registers::Registers,
}

impl Game {
  pub fn step(&mut self) {
    // Read the next instruction.
    // `next_command` will increment the PC.
    let instruction: u8 = self.memory.read_byte(self.registers.next_command());

    cpu::cpu_switch(instruction, &mut self.memory, &mut self.registers);
  }
}
