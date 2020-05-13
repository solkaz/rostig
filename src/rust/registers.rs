#![allow(dead_code)]
use std::u16;

const FLAG_ZERO: u8 = 1 << 7;
const FLAG_NEGATIVE: u8 = 1 << 6;
const FLAG_HALF_CARRY: u8 = 1 << 5;
const FLAG_CARRY: u8 = 1 << 4;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Debug)]
pub struct Registers {
  pub a: u8,
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub sp: u16,
  pub pc: u16,
}

impl Default for Registers {
  fn default() -> Registers {
    return Registers {
      a: 0x01,
      f: 0xb0,
      b: 0x00,
      c: 0x13,
      d: 0x00,
      e: 0xd8,
      h: 0x01,
      l: 0x4d,
      sp: 0xfffe,
      pc: 0x100,
    };
  }
}

impl Registers {
  pub fn get_af(&self) -> u16 {
    return (self.a as u16) << 8 | self.f as u16;
  }

  pub fn set_af(&mut self, val: u16) {
    self.a = ((val & 0xFF00) >> 8) as u8;
    self.f = (val & 0xFF) as u8;
  }

  pub fn get_bc(&self) -> u16 {
    return (self.b as u16) << 8 | self.c as u16;
  }

  pub fn set_bc(&mut self, val: u16) {
    self.b = ((val & 0xFF00) >> 8) as u8;
    self.c = (val & 0xFF) as u8;
  }

  pub fn get_de(&self) -> u16 {
    return (self.d as u16) << 8 | self.e as u16;
  }

  pub fn set_de(&mut self, val: u16) {
    self.d = ((val & 0xFF00) >> 8) as u8;
    self.e = (val & 0xFF) as u8;
  }

  pub fn get_hl(&self) -> u16 {
    return (self.h as u16) << 8 | self.l as u16;
  }

  pub fn set_hl(&mut self, val: u16) {
    self.h = ((val & 0xFF00) >> 8) as u8;
    self.l = (val & 0xFF) as u8;
  }

  pub fn reset(&mut self) {
    self.a = 0x01;
    self.f = 0xb0;
    self.b = 0x00;
    self.c = 0x13;
    self.d = 0x00;
    self.e = 0xd8;
    self.h = 0x01;
    self.l = 0x4d;
    self.sp = 0xfffe;
    self.pc = 0x100;
  }

  pub fn next_command(&mut self) -> u16 {
    let instruction: u16 = self.pc;
    self.pc += 1;
    return instruction;
  }

  // Flags
  pub fn get_zero_flag(&self) -> bool {
    return (self.f & FLAG_ZERO) != 0;
  }

  pub fn set_zero_flag(&mut self, val: bool) {
    if val {
      self.f |= FLAG_ZERO;
    } else {
      self.f &= !FLAG_ZERO;
    }
  }

  pub fn get_subtract_flag(&self) -> bool {
    return (self.f & FLAG_NEGATIVE) != 0;
  }

  pub fn set_subtract_flag(&mut self, val: bool) {
    if val {
      self.f |= FLAG_NEGATIVE;
    } else {
      self.f &= !FLAG_NEGATIVE;
    }
  }

  pub fn get_half_carry_flag(&self) -> bool {
    return (self.f & FLAG_HALF_CARRY) != 0;
  }

  pub fn set_half_carry_flag(&mut self, val: bool) {
    if val {
      self.f |= FLAG_HALF_CARRY;
    } else {
      self.f &= !FLAG_HALF_CARRY;
    }
  }

  pub fn get_carry_flag(&self) -> bool {
    return (self.f & FLAG_CARRY) != 0;
  }

  pub fn set_carry_flag(&mut self, val: bool) {
    if val {
      self.f |= FLAG_CARRY;
    } else {
      self.f &= !FLAG_CARRY;
    }
  }
}

pub const REGISTERS: Registers = Registers {
  a: 0x01,
  f: 0xb0,
  b: 0x00,
  c: 0x13,
  d: 0x00,
  e: 0xd8,
  h: 0x01,
  l: 0x4d,
  sp: 0xfffe,
  pc: 0x100,
};
