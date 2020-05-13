use crate::game::interrupts::INTERRUPTS;
use crate::game::memory::Memory;

pub struct Gpu {
  pub control: u8,
  pub scroll_x: u8,
  pub scroll_y: u8,
  pub scanline: u8,
  pub tick: u32,

  last_ticks: u32,
  mode: GpuMode,
}

enum GpuMode {
  Hblank,
  Vblank,
  Oam,
  Vram,
}

impl Gpu {
  pub fn step(&mut self, ticks: &mut u32) {
    self.tick = *ticks - self.last_ticks;
    self.last_ticks = *ticks;

    use GpuMode::*;
    match self.mode {
      Hblank => {
        if self.tick >= 204 {
          self.scanline += 1;

          if self.scanline == 143 {
            if INTERRUPTS.has_vblank_interrupt() {
              INTERRUPTS.set_vblank_interrupt();
            }

            self.mode = Vblank;
          } else {
            self.mode = Oam;
          }
          self.tick -= 204;
        }
      }
      Vblank => {
        if self.tick >= 456 {
          self.scanline += 1;

          if self.scanline == 153 {
            self.scanline = 0;
            self.mode = Oam;
          }
          self.tick -= 456;
        }
      }
      Oam => {
        if self.tick >= 80 {
          self.mode = Vram;
          self.tick -= 80;
        }
      }
      Vram => {
        if self.tick >= 172 {
          self.mode = Hblank;
          self.tick -= 172;
        }
      }
    }
  }
}

pub const GPU: Gpu = Gpu {
  control: 0,
  scroll_x: 0,
  scroll_y: 0,
  scanline: 0,
  tick: 0,

  last_ticks: 0,
  mode: GpuMode::Hblank,
};

const TILES: [[[u8; 8]; 8]; 384] = [[[0; 8]; 8]; 384];

struct Sprite {
  x: u8,
  y: u8,
  tile: u8,
}

#[derive(Clone, Copy)]
struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

const BACKGROUND_PALETTE: [Color; 4] = [Color { r: 0, g: 0, b: 0 }; 4];
const SPRITE_PALETTE: [[Color; 4]; 2] = [[Color { r: 0, g: 0, b: 0 }; 4]; 2];

fn update_tile(memory: &Memory, _addr: &mut u16) {
  let addr = *_addr & 0x1ffe;

  let tile: u16 = (addr >> 4) & 511;
  let y: u8 = ((addr >> 1) & 7) as u8;

  let mut bit_index: u8;
  for idx in 0..8 {
    bit_index = 1 << (7 - idx);

    let a: u8 = if ((*memory).video_ram[addr as usize] & bit_index) != 0 {
      1
    } else {
      0
    };

    let b: u8 = if ((*memory).video_ram[(addr as usize) + 1] & bit_index) != 0 {
      2
    } else {
      0
    };

    TILES[tile as usize][y as usize][idx] = a + b;
  }
}
