use js_sys::Uint8Array;

#[path = "./cpu.rs"]
pub mod cpu;
#[path = "./gpu.rs"]
pub mod gpu;
#[path = "./interrupts.rs"]
pub mod interrupts;
#[path = "./memory.rs"]
pub mod memory;
#[path = "./registers.rs"]
pub mod registers;

pub struct Game {
  pub ticks: u32,
  pub cpu: cpu::Cpu,
  pub memory: memory::Memory,
  pub registers: registers::Registers,
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

impl Game {
  pub fn step(&mut self) {
    // Read the next instruction.
    // `next_command` will increment the PC.
    log!("PC: {:#x}", self.registers.pc);
    let instruction: u8 = self.memory.read_byte(self.registers.next_command());

    log!("Instruction executing {:#x}", instruction);
    log!("Ticks before: {}", self.ticks);

    self.cpu.step(
      instruction,
      &mut self.memory,
      &mut self.registers,
      &mut self.ticks,
    );

    log!("Ticks after: {}", self.ticks);

    log!("GPU step begin");
    gpu::GPU.step(&mut self.ticks);
    log!("GPU step end");
    log!("Interrupt step begin");
    interrupts::INTERRUPTS.step(&mut self.registers, &mut self.memory, &mut self.ticks);
    log!("Interrupt step end");
  }
}

pub fn new_game(cartridge: [u8; 0x8000]) -> Game {
  return Game {
    ticks: 0,
    cpu: Default::default(),
    registers: Default::default(),
    memory: memory::Memory {
      cartridge,
      io: [0; 0x100],
      video_ram: [0; 0x2000],
      switchable_ram: [0; 0x2000],
      write_ram: [0; 0x2000],
      hardware_ram: [0; 0x80],
      oam: [0; 0x100],
    },
  };
}

const NINTENDO_LOGO: &[u8] = &[
  0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
  0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
  0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

fn matches_nintendo_logo(buf: &[u8]) -> bool {
  return (*NINTENDO_LOGO).eq(&*buf);
}

pub fn validate_cartridge(loaded: &[u8]) -> Result<[u8; 0x8000], &'static str> {
  if loaded.len() < 100 {
    return Err("Invalid game cart");
  }
  //  else if !matches_nintendo_logo(&loaded[0x0104..0x0133]) {
  //   return Err("Validation for Nintendo Logo failed");
  // }
  else {
    let mut buffer: [u8; 0x8000] = [0u8; 0x8000];
    Uint8Array::from(loaded).copy_to(&mut buffer);
    return Ok(buffer);
  }
}
