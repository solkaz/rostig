extern crate web_sys;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod game;
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  // Your code goes here!
  console::log_1(&JsValue::from_str("Helloorld!"));

  Ok(())
}

const NINTENDO_LOGO: &[u8] = &[
  0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
  0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
  0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

#[wasm_bindgen]
pub fn load_cartridge(loaded: &[u8]) {
  match validate_cartridge(loaded) {
    Ok(cartridge) => {
      let mut game_instance = new_game(cartridge);
      log!("{:?}", game_instance.registers);
      game_instance.step();
      game_instance.step();
      game_instance.step();
    }
    Err(_) => (),
  }
}

fn matches_nintendo_logo(buf: &[u8]) -> bool {
  return NINTENDO_LOGO.eq(buf);
}

fn validate_cartridge(loaded: &[u8]) -> Result<[u8; 0x8000], &'static str> {
  if loaded.len() < 100 {
    return Err("Invalid game cart");
  } else if !matches_nintendo_logo(&loaded[0x0104..0x0133]) {
    return Err("Validation for Nintendo Logo failed");
  } else {
    let mut buffer: [u8; 0x8000] = [0u8; 0x8000];
    Uint8Array::from(loaded).copy_to(&mut buffer);
    return Ok(buffer);
  }
}

fn new_game(cartridge: [u8; 0x8000]) -> game::Game {
  return game::Game {
    cpu: game::cpu::Cpu {},
    memory: game::memory::Memory {
      cartridge,
      io: [0; 0x100],
      video_ram: [0; 0x2000],
      switchable_ram: [0; 0x2000],
      write_ram: [0; 0x2000],
      hardware_ram: [0; 0x80],
      oam: [0; 0x100],
    },
    registers: Default::default(),
  };
}
