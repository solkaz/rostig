#![allow(dead_code)]

use crate::game::memory::Memory;
use crate::game::registers::Registers;
use crate::game::registers::REGISTERS;

const INTERRUPTS_VBLANK: u8 = 1 << 0;
const INTERRUPTS_LCDSTAT: u8 = 1 << 1;
const INTERRUPTS_TIMER: u8 = 1 << 2;
const INTERRUPTS_SERIAL: u8 = 1 << 3;
const INTERRUPTS_JOYPAD: u8 = 1 << 4;

#[derive(Debug)]
pub struct Interrupts {
  pub master: u8,
  pub enable: u8,
  pub flags: u8,
}

impl Default for Interrupts {
  fn default() -> Interrupts {
    return Interrupts {
      master: 1,
      enable: 0,
      flags: 0,
    };
  }
}

impl Interrupts {
  pub fn step(&mut self, registers: &mut Registers, memory: &mut Memory, ticks: &mut u32) {
    if self.master != 0 && self.enable != 0 && self.flags != 0 {
      let enabled_flags: u8 = self.enable & self.flags;
      if (enabled_flags & INTERRUPTS_VBLANK) != 0 {
        self.flags &= !INTERRUPTS_VBLANK;
        self.vblank(registers, memory, ticks);
      }
      if (enabled_flags & INTERRUPTS_LCDSTAT) != 0 {
        self.flags &= !INTERRUPTS_LCDSTAT;
        self.lcd_stat(registers, memory, ticks);
      }
      if (enabled_flags & INTERRUPTS_TIMER) != 0 {
        self.flags &= !INTERRUPTS_TIMER;
        self.timer(registers, memory, ticks);
      }
      if (enabled_flags & INTERRUPTS_SERIAL) != 0 {
        self.flags &= !INTERRUPTS_SERIAL;
        self.serial(registers, memory, ticks);
      }
      if (enabled_flags & INTERRUPTS_JOYPAD) != 0 {
        self.flags &= !INTERRUPTS_JOYPAD;
        self.joypad(registers, memory, ticks);
      }
    }
  }

  pub fn has_vblank_interrupt(&self) -> bool {
    return (self.enable & INTERRUPTS_VBLANK) != 0;
  }

  pub fn set_vblank_interrupt(&mut self) {
    self.flags |= INTERRUPTS_VBLANK;
  }

  pub fn return_from_interrupt(&mut self, registers: &mut Registers, memory: &Memory) {
    self.master = 1;
    registers.pc = memory.read_short_from_stack(registers)
  }

  fn vblank(&mut self, registers: &mut Registers, memory: &mut Memory, ticks: &mut u32) {
    self.master = 0;
    memory.write_short_to_stack(registers, registers.pc);
    registers.pc = 0x40;
    *ticks += 12;
  }

  fn lcd_stat(&mut self, registers: &mut Registers, memory: &mut Memory, ticks: &mut u32) {
    self.master = 0;
    memory.write_short_to_stack(registers, registers.pc);
    registers.pc = 0x48;
    *ticks += 12;
  }

  fn timer(&mut self, registers: &mut Registers, memory: &mut Memory, ticks: &mut u32) {
    self.master = 0;
    memory.write_short_to_stack(registers, registers.pc);
    registers.pc = 0x50;
    *ticks += 12;
  }

  fn serial(&mut self, registers: &mut Registers, memory: &mut Memory, ticks: &mut u32) {
    self.master = 0;
    memory.write_short_to_stack(registers, registers.pc);
    registers.pc = 0x58;
    *ticks += 12;
  }

  fn joypad(&mut self, registers: &mut Registers, memory: &mut Memory, ticks: &mut u32) {
    self.master = 0;
    memory.write_short_to_stack(registers, registers.pc);
    registers.pc = 0x60;
    *ticks += 12;
  }
}

pub const INTERRUPTS: Interrupts = Interrupts {
  master: 1,
  enable: 0,
  flags: 0,
};
