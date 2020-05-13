use crate::game::gpu::GPU;
use crate::game::interrupts::INTERRUPTS;
use crate::game::registers::Registers;

pub struct Memory {
  // the game being played
  pub cartridge: [u8; 0x8000],
  pub video_ram: [u8; 0x2000],
  pub io: [u8; 0x100],
  // Addresses A000-BF00
  pub switchable_ram: [u8; 0x2000],
  // Addresses E000-FE00 & C000-DE00
  pub write_ram: [u8; 0x2000],
  pub hardware_ram: [u8; 0x80],
  pub oam: [u8; 0x100],
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

impl Memory {
  pub fn read_byte(&self, address: u16) -> u8 {
    let address_as_usize: usize = address as usize;
    match address {
      0..=0x7fff => self.cartridge[address_as_usize],
      0x8000..=0x9fff => self.video_ram[address_as_usize - 0x8000],
      0xa000..=0xbfff => self.switchable_ram[address_as_usize - 0xa000],
      0xc000..=0xdfff => self.write_ram[address_as_usize - 0xc000],
      0xe000..=0xfdff => self.write_ram[address_as_usize - 0xe000],
      0xfe00..=0xfeff => self.oam[address_as_usize - 0xfe00],
      0xff40 => GPU.control,
      0xff42 => GPU.scroll_y,
      0xff43 => GPU.scroll_x,
      0xff44 => GPU.scanline,
      0xff0f => INTERRUPTS.flags,
      0xffff => INTERRUPTS.enable,
      0xff00..=0xff7f => self.io[address_as_usize - 0xff00],
      0xff80..=0xfffe => self.hardware_ram[address_as_usize - 0xff00],
    }
  }

  pub fn read_short(&self, address: u16) -> u16 {
    let a: u16 = self.read_byte(address + 1).into();
    let b: u16 = self.read_byte(address).into();
    return b | (a << 8);
  }

  pub fn read_short_from_stack(&self, registers: &mut Registers) -> u16 {
    let val: u16 = self.read_short(registers.sp);
    registers.sp = registers.sp.wrapping_add(2);
    return val;
  }

  pub fn write_byte(&mut self, address: u16, val: u8) {
    let address_as_usize: usize = address as usize;
    log!("Address: {:#x}", address);
    match address {
      0..=0x7fff => panic!(
        "Attempting to write to location {}, which is read-only memory",
        address_as_usize
      ),
      0x8000..=0x9fff => self.video_ram[address_as_usize - 0x8000] = val,
      0xa000..=0xbfff => self.switchable_ram[address_as_usize - 0xa000] = val,
      0xc000..=0xdfff => self.write_ram[address_as_usize - 0xc000] = val,
      0xe000..=0xfdff => self.write_ram[address_as_usize - 0xe000] = val,

      0xff40 => GPU.control = val,
      0xff42 => GPU.scroll_y = val,
      0xff43 => GPU.scroll_x = val,
      0xff46 => {
        // Copy
        let source_addr: u16 = (val as u16) << 8;
        for idx in 0..160 {
          self.write_byte(0xfe00 + idx, self.read_byte(source_addr + idx))
        }
      }
      // 0xff44 => panic!("Attempting to write to memory address 0xff44, which is read-only memory"),
      0xfe00..=0xfeff => self.oam[address_as_usize - 0xfe00] = val,
      0xff0f => INTERRUPTS.flags = val,
      0xffff => INTERRUPTS.enable = val,
      0xff00..=0xff7f => self.io[address_as_usize - 0xff00] = val,
      0xff80..=0xfffe => self.hardware_ram[address_as_usize - 0xff00] = val,
    }
  }

  pub fn write_short(&mut self, address: u16, val: u16) {
    self.write_byte(address, (val & 0x00ff) as u8);
    self.write_byte(address + 1, ((val & 0xff00) >> 8) as u8);
  }

  pub fn write_short_to_stack(&mut self, registers: &mut Registers, val: u16) {
    registers.sp -= 2;
    self.write_short(registers.sp, val)
  }
}
