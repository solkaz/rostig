#![allow(dead_code)]
use std::u16;

const FLAG_ZERO: u8 = 1 << 7;
const FLAG_NEGATIVE: u8 = 1 << 6;
const FLAG_HALF_CARRY: u8 = 1 << 5;
const FLAG_CARRY: u8 = 1 << 4;

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

pub enum Register {
  A,
  F,
  B,
  C,
  D,
  E,
  H,
  L,
}

pub enum CombinationRegister {
  // Combination registers
  Af,
  Bc,
  De,
  Hl,
  Pc,
  Sp,
}

fn combine_u8s(a: &u8, b: &u8) -> u16 {
  return u16::from_le_bytes([*a, *b]);
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
  pub fn get_register(&self, register: Register) -> &u8 {
    match register {
      Register::A => &self.a,
      Register::F => &self.f,
      Register::B => &self.b,
      Register::C => &self.c,
      Register::D => &self.d,
      Register::E => &self.e,
      Register::H => &self.h,
      Register::L => &self.l,
    }
  }

  pub fn set_register(&mut self, register: Register, value: u8) {
    match register {
      Register::A => self.a = value,
      Register::F => self.f = value,
      Register::B => self.b = value,
      Register::C => self.c = value,
      Register::D => self.d = value,
      Register::E => self.e = value,
      Register::H => self.h = value,
      Register::L => self.l = value,
    }
  }

  pub fn get_double_register(&self, register: CombinationRegister) -> u16 {
    match register {
      CombinationRegister::Af => combine_u8s(&self.a, &self.f),
      CombinationRegister::Bc => combine_u8s(&self.b, &self.c),
      CombinationRegister::De => combine_u8s(&self.d, &self.e),
      CombinationRegister::Hl => combine_u8s(&self.h, &self.l),
      CombinationRegister::Pc => self.pc,
      CombinationRegister::Sp => self.sp,
    }
  }

  pub fn set_double_register(&mut self, register: CombinationRegister, value: u16) {
    match register {
      CombinationRegister::Pc => self.pc = value,
      CombinationRegister::Sp => self.sp = value,
      _ => (),
    }
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
    let instruction: u16 = self.get_double_register(CombinationRegister::Pc);
    self.set_double_register(CombinationRegister::Pc, instruction + 1);
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
