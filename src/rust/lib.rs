extern crate web_sys;
use wasm_bindgen::prelude::*;

mod game;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  Ok(())
}

#[wasm_bindgen]
pub fn load_cartridge(loaded: &[u8]) {
  match game::validate_cartridge(loaded) {
    Ok(cartridge) => {
      let mut game_instance = game::new_game(cartridge);
      let mut i: u8 = 0;
      loop {
        if i == 100 {
          break;
        } else {
          game_instance.step();
          i += 1;
        }
      }
    }
    Err(_) => (log!("Error!")),
  }
}
