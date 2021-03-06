use crate::game::interrupts::INTERRUPTS;
use crate::game::memory::Memory;
use crate::game::registers::Registers;
use crate::game::registers::REGISTERS;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const INSTRUCTION_TICKS: &[u8] = &[
  2, 6, 4, 4, 2, 2, 4, 4, 10, 4, 4, 4, 2, 2, 4, 4, // 0x0_
  2, 6, 4, 4, 2, 2, 4, 4, 4, 4, 4, 4, 2, 2, 4, 4, // 0x1_
  0, 6, 4, 4, 2, 2, 4, 2, 0, 4, 4, 4, 2, 2, 4, 2, // 0x2_
  4, 6, 4, 4, 6, 6, 6, 2, 0, 4, 4, 4, 2, 2, 4, 2, // 0x3_
  2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0x4_
  2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0x5_
  2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0x6_
  4, 4, 4, 4, 4, 4, 2, 4, 2, 2, 2, 2, 2, 2, 4, 2, // 0x7_
  2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0x8_
  2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0x9_
  2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0xa_
  2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0xb_
  0, 6, 0, 6, 0, 8, 4, 8, 0, 2, 0, 0, 0, 6, 4, 8, // 0xc_
  0, 6, 0, 0, 0, 8, 4, 8, 0, 8, 0, 0, 0, 0, 4, 8, // 0xd_
  6, 6, 4, 0, 0, 8, 4, 8, 8, 2, 8, 0, 0, 0, 4, 8, // 0xe_
  6, 6, 4, 2, 0, 8, 4, 8, 6, 4, 8, 2, 0, 0, 4, 8, // 0xf_
];

#[derive(Default, Debug)]
pub struct Cpu {
  stopped: bool,
}

impl Cpu {
  pub fn step(
    &mut self,
    opcode: u8,
    memory: &mut Memory,
    registers: &mut Registers,
    ticks: &mut u32,
  ) {
    if self.stopped {
      return;
    }

    match opcode {
      0x00 | 0x7f | 0x40 | 0x49 | 0x52 | 0x5b | 0x64 | 0x6d => nop(),

      0x01 => ld_bc_nn(memory.read_short(registers.pc), registers),
      0x02 => ld_bc_a(registers, memory),
      // 0x03 => inc_bc(registers),
      0x04 => inc_b(registers),
      0x05 => dec_b(registers),
      0x06 => ld_b_n(memory.read_byte(registers.pc), registers),
      // 0x07 => rlca(),
      0x08 => ld_nn_sp(memory.read_short(registers.pc), registers, memory),
      // 0x09 => add_hl_bc(registers),
      0x0a => ld_a_bc(registers, memory),
      0x0b => dec_bc(registers),
      0x0c => inc_c(registers),
      0x0d => dec_c(registers),
      0x0e => ld_c_n(memory.read_byte(registers.pc), registers),
      0x0f => rrca(registers),

      0x10 => {
        registers.pc += 1;
        self.stopped = true;
      }
      0x11 => ld_de_nn(memory.read_short(registers.pc), registers),
      0x12 => ld_de_a(registers, memory),
      0x14 => inc_d(registers),
      0x15 => dec_d(registers),
      0x16 => ld_d_n(memory.read_byte(registers.pc), registers),
      // 0x17
      // 0x18
      // 0x19
      // 0x1a
      // 0x1b
      0x1c => inc_e(registers),
      0x1d => dec_e(registers),
      0x1e => ld_e_n(memory.read_byte(registers.pc), registers),
      0x1f => rra(registers),

      0x20 => jr_nz_n(memory.read_byte(registers.pc), registers, ticks),
      0x21 => ld_hl_nn(memory.read_short(registers.pc), registers),
      // 0x22
      // 0x23
      0x24 => inc_h(registers),
      0x25 => dec_h(registers),
      0x26 => ld_h_n(memory.read_byte(registers.pc), registers),
      // 0x27
      0x28 => jr_z_n(memory.read_byte(registers.pc), registers, ticks),
      // 0x29
      // 0x2a
      // 0x2b
      0x2c => inc_l(registers),
      0x2d => dec_l(registers),
      0x2e => ld_l_n(memory.read_byte(registers.pc), registers),
      0x2f => cpl(registers),

      0x30 => jr_nc_n(memory.read_byte(registers.pc), registers, ticks),
      0x31 => ld_sp_nn(memory.read_short(registers.pc), registers),
      0x32 => ldd_hlp_a(registers, memory),
      0x33 => inc_sp(registers),
      // 0x34
      // 0x35
      // 0x36
      0x37 => scf(registers),
      0x38 => jr_c_n(memory.read_byte(registers.pc), registers, ticks),
      // 0x39
      // 0x3a
      0x3b => dec_sp(registers),
      0x3c => inc_a(registers),
      0x3d => dec_a(registers),
      0x3e => ld_a_n(memory.read_byte(registers.pc), registers),
      0x3f => ccf(registers),

      // 0x40 => nop(),
      0x41 => ld_b_c(registers),
      0x42 => ld_b_d(registers),
      0x43 => ld_b_e(registers),
      0x44 => ld_b_h(registers),
      0x45 => ld_b_l(registers),
      0x46 => ld_b_hl(registers, memory),
      0x47 => ld_b_a(registers),
      0x48 => ld_c_b(registers),
      // 0x49 => nop(),
      0x4a => ld_c_d(registers),
      0x4b => ld_c_e(registers),
      0x4c => ld_c_h(registers),
      0x4d => ld_c_l(registers),
      0x4e => ld_c_hl(registers, memory),
      0x4f => ld_c_a(registers),

      0x50 => ld_d_b(registers),
      0x51 => ld_d_c(registers),
      // 0x52 => nop(),
      0x53 => ld_d_e(registers),
      0x54 => ld_d_h(registers),
      0x55 => ld_d_l(registers),
      0x56 => ld_d_hl(registers, memory),
      0x57 => ld_d_a(registers),
      0x58 => ld_e_b(registers),
      0x59 => ld_e_c(registers),
      0x5a => ld_e_d(registers),
      // 0x5b => nop(),
      0x5c => ld_e_h(registers),
      0x5d => ld_e_l(registers),
      0x5e => ld_e_hl(registers, memory),
      0x5f => ld_e_a(registers),

      0x60 => ld_h_b(registers),
      0x61 => ld_h_c(registers),
      0x62 => ld_h_d(registers),
      0x63 => ld_h_e(registers),
      // 0x64 => nop(),
      0x65 => ld_h_l(registers),
      0x66 => ld_h_hl(registers, memory),
      0x67 => ld_h_a(registers),
      0x68 => ld_l_b(registers),
      0x69 => ld_l_c(registers),
      0x6a => ld_l_d(registers),
      0x6b => ld_l_e(registers),
      0x6c => ld_l_h(registers),
      // 0x6d => nop(),
      0x6e => ld_l_hl(registers, memory),
      0x6f => ld_l_a(registers),

      0x70 => ld_hl_b(registers, memory),
      0x71 => ld_hl_c(registers, memory),
      0x72 => ld_hl_d(registers, memory),
      0x73 => ld_hl_e(registers, memory),
      0x74 => ld_hl_h(registers, memory),
      0x75 => ld_hl_l(registers, memory),
      0x76 => halt(registers),
      0x77 => ld_hl_a(registers, memory),
      0x78 => ld_a_b(registers),
      0x79 => ld_a_c(registers),
      0x7a => ld_a_d(registers),
      0x7b => ld_a_e(registers),
      0x7c => ld_a_h(registers),
      0x7d => ld_a_l(registers),
      0x7e => ld_a_hl(registers, memory),
      // 0x7f
      0x80 => add_b(registers),
      0x81 => add_c(registers),
      0x82 => add_d(registers),
      0x83 => add_e(registers),
      0x84 => add_h(registers),
      0x85 => add_l(registers),
      0x86 => add_hl(registers, memory),
      0x87 => add_a(registers),
      0x88 => adc_b(registers),
      0x89 => adc_c(registers),
      0x8a => adc_d(registers),
      0x8b => adc_e(registers),
      0x8c => adc_h(registers),
      0x8d => adc_l(registers),
      // 0x8e=> adc_hl(registers),
      0x8f => adc_a(registers),

      0x90 => sub_b(registers),
      0x91 => sub_c(registers),
      0x92 => sub_d(registers),
      0x93 => sub_e(registers),
      0x94 => sub_h(registers),
      0x95 => sub_l(registers),
      // 0x96 => sub_hl(registers),
      0x97 => sub_a(registers),
      0x98 => sbc_b(registers),
      0x99 => sbc_c(registers),
      0x9a => sbc_d(registers),
      0x9b => sbc_e(registers),
      0x9c => sbc_h(registers),
      0x9d => sbc_l(registers),
      // 0x9e=> sbc_hl(registers),
      0x9f => sbc_a(registers),

      0xa0 => and_b(registers),
      0xa1 => and_c(registers),
      0xa2 => and_d(registers),
      0xa3 => and_e(registers),
      0xa4 => and_h(registers),
      0xa5 => and_l(registers),
      // 0xa6 => and_hl(registers),
      0xa7 => and_a(registers),
      0xa8 => xor_b(registers),
      0xa9 => xor_c(registers),
      0xaa => xor_d(registers),
      0xab => xor_e(registers),
      0xac => xor_h(registers),
      0xad => xor_l(registers),
      // 0xae xor_hl(registers),
      0xaf => xor_a(registers),

      0xb0 => or_b(registers),
      0xb1 => or_c(registers),
      0xb2 => or_d(registers),
      0xb3 => or_e(registers),
      0xb4 => or_h(registers),
      0xb5 => or_l(registers),
      // 0xb6 or_hl(registers),
      0xb7 => or_a(registers),
      0xb8 => cp_b(registers),
      0xb9 => cp_c(registers),
      0xba => cp_d(registers),
      0xbb => cp_e(registers),
      0xbc => cp_h(registers),
      0xbd => cp_l(registers),
      // 0xbe => cp_hl(registers),
      0xbf => cp_a(registers),

      0xc0 => ret_nz(registers, memory, ticks),
      0xc1 => pop_bc(registers, memory),
      0xc2 => jp_nz_nn(memory.read_short(registers.pc), registers, ticks),
      0xc3 => jp_nn(memory.read_short(registers.pc), registers),
      // 0xc4
      0xc5 => push_bc(registers, memory),
      0xc6 => add_n(memory.read_byte(registers.pc), registers),
      0xc7 => rst_0(registers, memory),
      // 0xc8
      0xc9 => ret(registers, memory),
      // 0xca => jp_z_nn(memory.read_short(registers.pc)),
      // 0xcb => cb_n(memory.read_byte(registers.pc)),
      // 0xcc => call_z_nn(memory.read_short(registers.pc)),
      // 0xcd => call_nn(memory.read_short(registers.pc)),
      0xce => adc_n(memory.read_byte(registers.pc), registers),
      0xcf => rst_08(registers, memory),

      // 0xd0 => ret_nc(),
      0xd1 => pop_de(registers, memory),
      0xd2 => jp_nc_nn(memory.read_short(registers.pc), registers, ticks),
      // 0xd3
      // 0xd4
      0xd5 => push_de(registers, memory),
      0xd6 => sub_n(memory.read_byte(registers.pc), registers),
      0xd7 => rst_10(registers, memory),
      0xd8 => ret_c(registers, memory, ticks),
      0xd9 => INTERRUPTS.return_from_interrupt(registers, memory),
      // 0xda => jp_c_nn(memory.read_short(registers.pc)),
      // 0xdb => unimplemented!,
      // 0xdd => unimplemented!,
      0xde => sbc_n(memory.read_byte(registers.pc), registers),
      0xdf => rst_18(registers, memory),

      0xe1 => pop_hl(registers, memory),
      // 0xe2 => ld_ff_c_a
      // 0xe3 => unimplemented!
      // 0xe4 => unimplemented!
      0xe5 => push_hl(registers, memory),
      0xe6 => and_n(memory.read_byte(registers.pc), registers),
      0xe7 => rst_20(registers, memory),
      0xe9 => jp_hl(registers),
      // 0xeb => unimplemented!,
      // 0xec => unimplemented!,
      // 0xed => unimplemented!,
      0xee => xor_n(memory.read_byte(registers.pc), registers),
      0xef => rst_28(registers, memory),

      // 0xf0
      0xf1 => pop_af(registers, memory),
      0xf2 => ld_a_ff_c(registers, memory),
      0xf3 => di(),
      // 0xf4 => unimplemented!,
      0xf5 => push_af(registers, memory),
      0xf6 => or_n(memory.read_byte(registers.pc), registers),
      0xf7 => rst_30(registers, memory),
      // 0xf8 => unimplemented!,
      0xf9 => ld_sp_hl(registers),
      // 0xfa
      0xfb => ei(),
      // 0xfc => unimplemented!,
      // 0xfd => unimplemented!,
      0xfe => cp_n(memory.read_byte(registers.pc), registers),
      0xff => rst_38(registers, memory),

      x => log!("Unsupported opcode! {:#x}", x),
    }

    *ticks += INSTRUCTION_TICKS[opcode as usize] as u32;
  }
}

fn add(mut val: u8, registers: &mut Registers) {
  val += registers.a;
  registers.set_carry_flag((val as u16) & 0xff00 != 0);
  registers.a = val & 0xff;
  registers.set_zero_flag(registers.a != 0);
  registers.set_half_carry_flag((registers.a & 0x0f) + (val & 0x0f) > 0xf);
  registers.set_subtract_flag(false);
}

fn adc(val: u8, registers: &mut Registers) {
  let mut result: u8 = val + registers.a;
  if registers.get_carry_flag() {
    result += 1;
  }
  registers.set_carry_flag((val as u16) & 0xff00 != 0);
  registers.set_zero_flag(registers.a == val);
  registers.set_half_carry_flag((registers.a & 0x0f) + (val & 0x0f) > 0xf);
  registers.set_subtract_flag(false);
  registers.a = result & 0xff;
}

fn sub(val: u8, registers: &mut Registers) {
  registers.set_subtract_flag(true);
  registers.set_carry_flag(val > registers.a);
  registers.set_half_carry_flag((val & 0x0f) > (registers.a & 0x0f));
  registers.a -= val;
  registers.set_zero_flag(registers.a != 0);
}

fn sbc(mut val: u8, registers: &mut Registers) {
  if registers.get_carry_flag() {
    val += 1;
  }
  registers.set_subtract_flag(true);
  registers.set_carry_flag(val > registers.a);
  registers.set_zero_flag(registers.a == val);
  registers.set_half_carry_flag((val & 0x0f) > (registers.a & 0x0f));
  registers.a -= val;
}

fn and(val: u8, registers: &mut Registers) {
  registers.a &= val;
  registers.set_zero_flag(registers.a != 0);
  registers.set_carry_flag(false);
  registers.set_half_carry_flag(false);
  registers.set_subtract_flag(false);
}

fn or(val: u8, registers: &mut Registers) {
  registers.a |= val;
  registers.set_zero_flag(registers.a != 0);
  registers.set_carry_flag(false);
  registers.set_half_carry_flag(false);
  registers.set_subtract_flag(false);
}

fn xor(val: u8, registers: &mut Registers) {
  registers.a ^= val;
  registers.set_zero_flag(registers.a != 0);
  registers.set_carry_flag(false);
  registers.set_half_carry_flag(false);
  registers.set_subtract_flag(false);
}

fn cp(val: u8, registers: &mut Registers) {
  registers.set_zero_flag(registers.a == val);
  registers.set_carry_flag(val > registers.a);
  registers.set_half_carry_flag((val & 0x0f) > (registers.a & 0x0f));
  registers.set_subtract_flag(true);
}

fn inc(mut val: u8, registers: &mut Registers) -> u8 {
  registers.set_half_carry_flag(val & 0x0f == 0x0f);

  val += 1;

  registers.set_zero_flag(val != 0);
  registers.set_subtract_flag(false);
  return val;
}

fn dec(mut val: u8, registers: &mut Registers) -> u8 {
  registers.set_half_carry_flag(val & 0x0f == 0x0f);

  val -= 1;

  registers.set_zero_flag(val != 0);
  registers.set_subtract_flag(true);
  return val;
}

// 0x00 - NOP
// 0x7f - LD A, A
// 0x40 - LD B, B
// 0x49 - LD C, C
// 0x52 - LD D, D
// 0x5b - LD E, E
// 0x64 - LD H, H
// 0x6d - LD L, L
fn nop() {
  log!("nop")
}

// 0x01
fn ld_bc_nn(val: u16, registers: &mut Registers) {
  registers.pc += 2;
  registers.set_bc(val);
}

// 0x02
fn ld_bc_a(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_bc(), registers.a);
}

// 0x04
fn inc_b(registers: &mut Registers) {
  registers.b = inc(registers.b, registers);
}

// 0x05
fn dec_b(registers: &mut Registers) {
  registers.b = dec(registers.b, registers);
}

// 0x06
fn ld_b_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  registers.b = val;
}

// 0x08
fn ld_nn_sp(val: u16, registers: &mut Registers, memory: &mut Memory) {
  registers.pc += 2;
  memory.write_short(val, registers.sp);
}

// 0x0a
fn ld_a_bc(registers: &mut Registers, memory: &Memory) {
  registers.a = memory.read_byte(registers.get_bc());
}

// 0x0b
fn dec_bc(registers: &mut Registers) {
  registers.set_bc(registers.get_bc().wrapping_sub(1));
}

// 0x0c
fn inc_c(registers: &mut Registers) {
  registers.c = inc(registers.c, registers);
}

// 0x0d
fn dec_c(registers: &mut Registers) {
  REGISTERS.c = dec(registers.c, registers);
}

// 0x0e
fn ld_c_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  registers.c = val;
}

// 0x0e
fn rrca(registers: &mut Registers) {
  let carry: bool = (registers.a & 0x01) != 0;
  registers.set_carry_flag(carry);
  registers.a >>= 1;
  if carry {
    registers.a |= 0x80;
  }
  registers.set_half_carry_flag(false);
  registers.set_subtract_flag(false);
  registers.set_zero_flag(false);
}

// 0x10 STOP
// Implemented inline

// 0x11
fn ld_de_nn(val: u16, registers: &mut Registers) {
  registers.pc += 2;
  registers.set_de(val);
}

// 0x12
fn ld_de_a(registers: &Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_de(), registers.a);
}

// 0x14
fn inc_d(registers: &mut Registers) {
  registers.d = inc(registers.d, registers);
}

// 0x15
fn dec_d(registers: &mut Registers) {
  registers.d = dec(registers.d, registers);
}

// 0x16
fn ld_d_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  registers.d = val;
}

// 0x1c
fn inc_e(registers: &mut Registers) {
  registers.e = inc(registers.e, registers);
}

// 0x1d
fn dec_e(registers: &mut Registers) {
  registers.e = dec(registers.e, registers);
}

// 0x1e
fn ld_e_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  registers.e = val;
}

// 0x1f
fn rra(registers: &mut Registers) {
  let carry: u8;
  if registers.get_carry_flag() {
    carry = 1 << 7;
  } else {
    carry = 0;
  }

  registers.set_carry_flag(registers.a & 0x01 != 0);
  registers.a >>= 1;
  registers.a += carry;
  registers.set_half_carry_flag(false);
  registers.set_subtract_flag(false);
  registers.set_zero_flag(false);
}

// 0x20
fn jr_nz_n(val: u8, registers: &mut Registers, ticks: &mut u32) {
  if registers.get_zero_flag() {
    *ticks += 8;
  } else {
    registers.pc += val as u16;
    *ticks += 12;
  }
}

// 0x21
fn ld_hl_nn(val: u16, registers: &mut Registers) {
  registers.pc += 2;
  registers.set_hl(val);
}

// 0x24
fn inc_h(registers: &mut Registers) {
  registers.h = inc(registers.h, registers);
}

// 0x25
fn dec_h(registers: &mut Registers) {
  registers.h = dec(registers.h, registers);
}

// 0x26
fn ld_h_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  registers.h = val;
}

// 0x28
fn jr_z_n(val: u8, registers: &mut Registers, ticks: &mut u32) {
  if registers.get_zero_flag() {
    registers.pc += val as u16;
    *ticks += 12;
  } else {
    *ticks += 8;
  }
}

// 0x2c
fn inc_l(registers: &mut Registers) {
  registers.l = inc(registers.l, registers);
}

// 0x2d
fn dec_l(registers: &mut Registers) {
  registers.l = dec(registers.l, registers);
}

// 0x2e
fn ld_l_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  registers.l = val;
}

// 0x2f CPL
fn cpl(registers: &mut Registers) {
  registers.a = !registers.a;
  registers.set_subtract_flag(true);
  registers.set_half_carry_flag(true);
}

// 0x30
fn jr_nc_n(val: u8, registers: &mut Registers, ticks: &mut u32) {
  if registers.get_carry_flag() {
    *ticks += 8;
  } else {
    registers.pc += val as u16;
    *ticks += 12;
  }
}

// 0x31
fn ld_sp_nn(val: u16, registers: &mut Registers) {
  registers.pc += 2;
  registers.sp = val;
}

// 0x32
fn ldd_hlp_a(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_hl(), registers.a);
}

// 0x33
fn inc_sp(registers: &mut Registers) {
  registers.sp += 1;
}

// 0x37
fn scf(registers: &mut Registers) {
  registers.set_carry_flag(true);
  registers.set_half_carry_flag(false);
  registers.set_subtract_flag(false);
}

// 0x38
fn jr_c_n(val: u8, registers: &mut Registers, ticks: &mut u32) {
  if registers.get_carry_flag() {
    registers.pc += val as u16;
    *ticks += 12;
  } else {
    *ticks += 8;
  }
}

// 0x3b
fn dec_sp(registers: &mut Registers) {
  registers.sp -= 1;
}

// 0x3c
fn inc_a(registers: &mut Registers) {
  registers.a = inc(registers.a, registers);
}

// 0x3d
fn dec_a(registers: &mut Registers) {
  registers.a = dec(registers.a, registers);
}

// 0x3e
fn ld_a_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  registers.a = val;
}

// 0x3f
fn ccf(registers: &mut Registers) {
  registers.set_carry_flag(registers.get_carry_flag());
  registers.set_subtract_flag(false);
  registers.set_half_carry_flag(false);
}

// 0x41 LD B, C
fn ld_b_c(registers: &mut Registers) {
  registers.b = registers.c;
}

// 0x42 LD B, D
fn ld_b_d(registers: &mut Registers) {
  registers.b = registers.d;
}

// 0x43 LD B, E
fn ld_b_e(registers: &mut Registers) {
  registers.b = registers.e;
}

// 0x44 LD B, H
fn ld_b_h(registers: &mut Registers) {
  registers.b = registers.h;
}

// 0x45 LD B, L
fn ld_b_l(registers: &mut Registers) {
  registers.b = registers.l;
}

// 0x46 LD B, (HL)
fn ld_b_hl(registers: &mut Registers, memory: &Memory) {
  registers.b = memory.read_byte(registers.get_hl())
}

// 0x47 LD B, A
fn ld_b_a(registers: &mut Registers) {
  registers.b = registers.a;
}

// 0x48 LD C, B
fn ld_c_b(registers: &mut Registers) {
  registers.c = registers.b;
}

// 0x4a LD C, D
fn ld_c_d(registers: &mut Registers) {
  registers.c = registers.d;
}

// 0x4b LD C, E
fn ld_c_e(registers: &mut Registers) {
  registers.c = registers.e;
}

// 0x4c LD C, H
fn ld_c_h(registers: &mut Registers) {
  registers.c = registers.h;
}

// 0x4d LD C, L
fn ld_c_l(registers: &mut Registers) {
  registers.c = registers.l;
}

// 0x4e LD C, (HL)
fn ld_c_hl(registers: &mut Registers, memory: &Memory) {
  registers.c = memory.read_byte(registers.get_hl())
}

// 0x4f LD C, A
fn ld_c_a(registers: &mut Registers) {
  registers.c = registers.a;
}

// 0x50 LD D, B
fn ld_d_b(registers: &mut Registers) {
  registers.d = registers.b;
}

// 0x51 LD D, C
fn ld_d_c(registers: &mut Registers) {
  registers.d = registers.c;
}

// 0x53 LD D, E
fn ld_d_e(registers: &mut Registers) {
  registers.d = registers.e;
}

// 0x54 LD D, H
fn ld_d_h(registers: &mut Registers) {
  registers.d = registers.h;
}

// 0x55 LD D, L
fn ld_d_l(registers: &mut Registers) {
  registers.d = registers.l;
}

// 0x56 LD D, (HL)
fn ld_d_hl(registers: &mut Registers, memory: &Memory) {
  registers.d = memory.read_byte(registers.get_hl())
}

// 0x57
fn ld_d_a(registers: &mut Registers) {
  registers.d = registers.a;
}

// 0x58
fn ld_e_b(registers: &mut Registers) {
  registers.e = registers.b;
}

// 0x59
fn ld_e_c(registers: &mut Registers) {
  registers.e = registers.c;
}

// 0x5a
fn ld_e_d(registers: &mut Registers) {
  registers.e = registers.d;
}

// 0x5c
fn ld_e_h(registers: &mut Registers) {
  registers.e = registers.h;
}

// 0x5d
fn ld_e_l(registers: &mut Registers) {
  registers.e = registers.l;
}

// 0x5e LD E, (HL)
fn ld_e_hl(registers: &mut Registers, memory: &Memory) {
  registers.e = memory.read_byte(registers.get_hl())
}

// 0x5f
fn ld_e_a(registers: &mut Registers) {
  registers.e = registers.a;
}

// 0x60 LD D, B
fn ld_h_b(registers: &mut Registers) {
  registers.h = registers.b;
}

// 0x61 LD D, C
fn ld_h_c(registers: &mut Registers) {
  registers.h = registers.c;
}

// 0x63 LD D, E
fn ld_h_d(registers: &mut Registers) {
  registers.h = registers.d;
}

// 0x64 LD D, H
fn ld_h_e(registers: &mut Registers) {
  registers.h = registers.e;
}

// 0x65
fn ld_h_l(registers: &mut Registers) {
  registers.h = registers.l;
}

// 0x66
fn ld_h_hl(registers: &mut Registers, memory: &Memory) {
  registers.h = memory.read_byte(registers.get_hl())
}

// 0x67
fn ld_h_a(registers: &mut Registers) {
  registers.h = registers.a;
}

// 0x68
fn ld_l_b(registers: &mut Registers) {
  registers.l = registers.b;
}

// 0x69
fn ld_l_c(registers: &mut Registers) {
  registers.l = registers.c;
}

// 0x6a
fn ld_l_d(registers: &mut Registers) {
  registers.l = registers.d;
}

// 0x6b
fn ld_l_e(registers: &mut Registers) {
  registers.l = registers.e;
}

// 0x6c
fn ld_l_h(registers: &mut Registers) {
  registers.l = registers.h;
}

// 0x6e
fn ld_l_hl(registers: &mut Registers, memory: &Memory) {
  registers.l = memory.read_byte(registers.get_hl())
}

// 0x6f
fn ld_l_a(registers: &mut Registers) {
  registers.l = registers.a;
}

// 0x70
fn ld_hl_b(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_hl(), registers.b)
}
// 0x71
fn ld_hl_c(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_hl(), registers.c)
}
// 0x72
fn ld_hl_d(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_hl(), registers.d)
}

// 0x73
fn ld_hl_e(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_hl(), registers.e)
}

// 0x74
fn ld_hl_h(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_hl(), registers.h)
}

// 0x75
fn ld_hl_l(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_hl(), registers.l)
}

// 0x76
fn halt(registers: &mut Registers) {
  if INTERRUPTS.master != 0 {
    registers.pc += 1;
  }
}

// 0x77
fn ld_hl_a(registers: &mut Registers, memory: &mut Memory) {
  memory.write_byte(registers.get_hl(), registers.a)
}

// 0x78
fn ld_a_b(registers: &mut Registers) {
  registers.a = registers.b;
}

// 0x79
fn ld_a_c(registers: &mut Registers) {
  registers.a = registers.c;
}

// 0x7a
fn ld_a_d(registers: &mut Registers) {
  registers.a = registers.d;
}

// 0x7b
fn ld_a_e(registers: &mut Registers) {
  registers.a = registers.e;
}

// 0x7c
fn ld_a_h(registers: &mut Registers) {
  registers.a = registers.h;
}

// 0x7d
fn ld_a_l(registers: &mut Registers) {
  registers.a = registers.l;
}

// 0x7e
fn ld_a_hl(registers: &mut Registers, memory: &Memory) {
  registers.a = memory.read_byte(registers.get_hl())
}

// 0x80
fn add_b(registers: &mut Registers) {
  add(registers.b, registers);
}

// 0x81
fn add_c(registers: &mut Registers) {
  add(registers.c, registers);
}

// 0x82
fn add_d(registers: &mut Registers) {
  add(registers.d, registers);
}

// 0x83
fn add_e(registers: &mut Registers) {
  add(registers.e, registers);
}

// 0x84
fn add_h(registers: &mut Registers) {
  add(registers.h, registers);
}

// 0x85
fn add_l(registers: &mut Registers) {
  add(registers.l, registers);
}

// 0x86
fn add_hl(registers: &mut Registers, memory: &Memory) {
  add(memory.read_byte(registers.get_hl()), registers);
}

// 0x87
fn add_a(registers: &mut Registers) {
  add(registers.a, registers);
}

// 0x88
fn adc_b(registers: &mut Registers) {
  adc(registers.b, registers);
}

// 0x89
fn adc_c(registers: &mut Registers) {
  adc(registers.c, registers);
}

// 0x8a
fn adc_d(registers: &mut Registers) {
  adc(registers.d, registers);
}

// 0x8b
fn adc_e(registers: &mut Registers) {
  adc(registers.e, registers);
}

// 0x8c
fn adc_h(registers: &mut Registers) {
  adc(registers.h, registers);
}

// 0x8d
fn adc_l(registers: &mut Registers) {
  adc(registers.l, registers);
}

// 0x8f
fn adc_a(registers: &mut Registers) {
  adc(registers.a, registers);
}

// 0x90
fn sub_b(registers: &mut Registers) {
  sub(registers.b, registers);
}

// 0x91
fn sub_c(registers: &mut Registers) {
  sub(registers.c, registers);
}

// 0x92
fn sub_d(registers: &mut Registers) {
  sub(registers.d, registers);
}

// 0x93
fn sub_e(registers: &mut Registers) {
  sub(registers.e, registers);
}

// 0x94
fn sub_h(registers: &mut Registers) {
  sub(registers.h, registers);
}

// 0x95
fn sub_l(registers: &mut Registers) {
  sub(registers.l, registers);
}

// 0x97
fn sub_a(registers: &mut Registers) {
  sub(registers.a, registers);
}

// 0x98
fn sbc_b(registers: &mut Registers) {
  sbc(registers.b, registers);
}

// 0x99
fn sbc_c(registers: &mut Registers) {
  sbc(registers.c, registers);
}

// 0x9a
fn sbc_d(registers: &mut Registers) {
  sbc(registers.d, registers);
}

// 0x9b
fn sbc_e(registers: &mut Registers) {
  sbc(registers.e, registers);
}

// 0x9c
fn sbc_h(registers: &mut Registers) {
  sbc(registers.h, registers);
}

// 0x9d
fn sbc_l(registers: &mut Registers) {
  sbc(registers.l, registers);
}

// 0x9f
fn sbc_a(registers: &mut Registers) {
  sbc(registers.a, registers);
}

// 0xa0
fn and_b(registers: &mut Registers) {
  and(registers.b, registers);
}

// 0xa1
fn and_c(registers: &mut Registers) {
  and(registers.c, registers);
}

// 0xa2
fn and_d(registers: &mut Registers) {
  and(registers.d, registers);
}

// 0xa3
fn and_e(registers: &mut Registers) {
  and(registers.e, registers);
}

// 0xa4
fn and_h(registers: &mut Registers) {
  and(registers.h, registers);
}

// 0xa5
fn and_l(registers: &mut Registers) {
  and(registers.l, registers);
}

// 0xa7
fn and_a(registers: &mut Registers) {
  and(registers.a, registers);
}

// 0xa8
fn xor_b(registers: &mut Registers) {
  xor(registers.b, registers);
}

// 0xa9
fn xor_c(registers: &mut Registers) {
  xor(registers.c, registers);
}

// 0xaa
fn xor_d(registers: &mut Registers) {
  xor(registers.d, registers);
}

// 0xab
fn xor_e(registers: &mut Registers) {
  xor(registers.e, registers);
}

// 0xac
fn xor_h(registers: &mut Registers) {
  xor(registers.h, registers);
}

// 0xad
fn xor_l(registers: &mut Registers) {
  xor(registers.l, registers);
}

// 0xaf
fn xor_a(registers: &mut Registers) {
  xor(registers.a, registers);
}

// 0xb0
fn or_b(registers: &mut Registers) {
  or(registers.b, registers);
}

// 0xb1
fn or_c(registers: &mut Registers) {
  or(registers.c, registers);
}

// 0xb2
fn or_d(registers: &mut Registers) {
  or(registers.d, registers);
}

// 0xb3
fn or_e(registers: &mut Registers) {
  or(registers.e, registers);
}

// 0xb4
fn or_h(registers: &mut Registers) {
  or(registers.h, registers);
}

// 0xb5
fn or_l(registers: &mut Registers) {
  or(registers.l, registers);
}

// 0xb7
fn or_a(registers: &mut Registers) {
  or(registers.a, registers);
}

// 0xb8
fn cp_b(registers: &mut Registers) {
  cp(registers.b, registers);
}

// 0xb9
fn cp_c(registers: &mut Registers) {
  cp(registers.c, registers);
}

// 0xba
fn cp_d(registers: &mut Registers) {
  cp(registers.d, registers);
}

// 0xbb
fn cp_e(registers: &mut Registers) {
  cp(registers.e, registers);
}

// 0xbc
fn cp_h(registers: &mut Registers) {
  cp(registers.h, registers);
}

// 0xbd
fn cp_l(registers: &mut Registers) {
  cp(registers.l, registers);
}

// 0xbf CP a
fn cp_a(registers: &mut Registers) {
  cp(registers.a, registers);
}

// 0xc0
fn ret_nz(registers: &mut Registers, memory: &Memory, ticks: &mut u32) {
  if registers.get_zero_flag() {
    *ticks += 8;
  } else {
    let val = memory.read_short_from_stack(registers);
    registers.pc += val;
    *ticks += 20;
  }
}

// 0xc1 POP BC
fn pop_bc(registers: &mut Registers, memory: &Memory) {
  let val: u16 = memory.read_short_from_stack(registers);
  registers.set_bc(val);
}

// 0xc2
fn jp_nz_nn(val: u16, registers: &mut Registers, ticks: &mut u32) {
  registers.pc += 2;
  if registers.get_zero_flag() {
    *ticks += 12;
  } else {
    registers.pc += val;
    *ticks += 16;
  }
}

// 0xc3 JP nn
fn jp_nn(val: u16, registers: &mut Registers) {
  registers.pc = val;
}

// 0xc5
fn push_bc(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.get_bc())
}

// 0xc6 ADD n
fn add_n(n: u8, registers: &mut Registers) {
  registers.pc += 1;
  add(n, registers)
}

// 0xc7
fn rst_0(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.pc);
  registers.pc = 0x0000;
}

// 0xc9
fn ret(registers: &mut Registers, memory: &Memory) {
  registers.pc = memory.read_short_from_stack(registers);
}

// 0xce ADC n
fn adc_n(n: u8, registers: &mut Registers) {
  registers.pc += 1;
  adc(n, registers)
}

// 0xcf
fn rst_08(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.pc);
  registers.pc = 0x0008;
}

// 0xd1 POP DE
fn pop_de(registers: &mut Registers, memory: &Memory) {
  let val: u16 = memory.read_short_from_stack(registers);
  registers.set_de(val);
}

// 0xd2 JP NC, nn
fn jp_nc_nn(val: u16, registers: &mut Registers, ticks: &mut u32) {
  if !registers.get_carry_flag() {
    registers.pc = val;
    *ticks += 16;
  } else {
    registers.pc += 2;
    *ticks += 12;
  }
}

// 0xd5
fn push_de(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.get_de())
}

// 0xd6 SUB n
fn sub_n(n: u8, registers: &mut Registers) {
  registers.pc += 1;
  sub(n, registers)
}

// 0xd7
fn rst_10(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.pc);
  registers.pc = 0x0010;
}

// 0xd8
fn ret_c(registers: &mut Registers, memory: &mut Memory, ticks: &mut u32) {
  if registers.get_carry_flag() {
    let val = memory.read_short_from_stack(registers);
    registers.pc = val;
    *ticks += 20;
  } else {
    *ticks += 8;
  }
}

// 0xde SBC n
fn sbc_n(n: u8, registers: &mut Registers) {
  registers.pc += 1;
  sbc(n, registers)
}

// 0xdf
fn rst_18(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.pc);
  registers.pc = 0x0018;
}

// 0xe1
fn pop_hl(registers: &mut Registers, memory: &mut Memory) {
  let val = memory.read_short_from_stack(registers);
  registers.set_hl(val);
}

// 0xe5
fn push_hl(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.get_hl());
}

// 0xe6 AND n
fn and_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  and(val, registers);
}

// 0xe7
fn rst_20(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.pc);
  registers.pc = 0x0020;
}

// 0xe9 JP HL
fn jp_hl(registers: &mut Registers) {
  registers.pc = registers.get_hl();
}

// 0xee XOR n
fn xor_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  xor(val, registers);
}

// 0xef
fn rst_28(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.pc);
  registers.pc = 0x0028;
}

// 0xf1
fn pop_af(registers: &mut Registers, memory: &mut Memory) {
  let val = memory.read_short_from_stack(registers);
  registers.set_af(val);
}

// 0xf2 LD A, (C + $$FF00)
fn ld_a_ff_c(registers: &mut Registers, memory: &Memory) {
  ld_a_n(memory.read_byte(0xff00) + registers.c, registers);
}

// 0xf3
fn di() {
  INTERRUPTS.master = 0;
}

// 0xf5
fn push_af(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.get_af());
}

// 0xf6 OR n
fn or_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  or(val, registers);
}

// 0xf7
fn rst_30(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.pc);
  registers.pc = 0x0030;
}
// 0xf9
fn ld_sp_hl(registers: &mut Registers) {
  registers.sp = registers.get_hl();
}

// 0xfb
fn ei() {
  INTERRUPTS.master = 1;
}

// 0xfe
fn cp_n(val: u8, registers: &mut Registers) {
  registers.pc += 1;
  cp(val, registers);
}

// 0xff
fn rst_38(registers: &mut Registers, memory: &mut Memory) {
  memory.write_short_to_stack(registers, registers.pc);
  registers.pc = 0x0038;
}
