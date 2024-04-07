#![deny(clippy::all)]

use enigo::*;
use napi::bindgen_prelude::Int32Array;
mod mouse_button;
pub use crate::mouse_button::{get_mouse_button, get_key}; 

#[macro_use]
extern crate napi_derive;

#[napi(js_name = "Enigo")]
pub struct EnigoJs {
  enigo: Enigo,
}

#[napi]
impl EnigoJs {
  #[napi(constructor)]
  pub fn new() -> Self {
    EnigoJs {
      enigo: Enigo::new(),
    }
  }
  // KeyboardControllable
  #[napi]
  pub fn key_down(&mut self, key: i32) {
    self.enigo.key_down(get_key(key));
  }

  #[napi]
  pub fn key_up(&mut self, key: i32) {
    self.enigo.key_up(get_key(key));
  }

  #[napi]
  pub fn key_click(&mut self, key: i32) {
    self.enigo.key_click(get_key(key));
  }

  #[napi]
  pub fn key_sequence(&mut self, sequence: String) {
    self.enigo.key_sequence(&sequence);
  }

  #[napi]
  pub fn key_sequence_parse(&mut self, sequence: String) {
    self.enigo.key_sequence_parse(&sequence);
  }

  // #[napi]
  // pub fn key_sequence_parse_try(&mut self, sequence: String) -> Result<(), ParseError> {
  //   Ok(self.enigo.key_sequence_parse(&sequence))
  // }

  #[napi]
  /// Move the mouse cursor to the specified x and y coordinates.
  /// @param {number} x - x coordinates.
  /// @param {number} y - y coordinates.
  /// ## Examples
  ///
  /// ```js
  /// const enigo = new Enigo();
  /// enigo.mouseMoveTo(width / 2, height / 2);
  /// ```
  pub fn mouse_move_to(&mut self, x: i32, y: i32) {
    self.enigo.mouse_move_to(x, y);
  }

  #[napi]
  pub fn mouse_move_relative(&mut self, x: i32, y: i32) {
    self.enigo.mouse_move_relative(x, y);
  }

  #[napi]
  pub fn mouse_down(&mut self, button: i32) {
    self.enigo.mouse_down(get_mouse_button(button));
  }

  #[napi]
  pub fn mouse_up(&mut self, button: i32) {
    self.enigo.mouse_up(get_mouse_button(button));
  }

  #[napi]
  pub fn mouse_click(&mut self, button: i32) {
    self.enigo.mouse_click(get_mouse_button(button));
  }

  #[napi]
  pub fn mouse_scroll_x(&mut self, length: i32) {
    self.enigo.mouse_scroll_x(length);
  }

  #[napi]
  pub fn mouse_scroll_y(&mut self, length: i32) {
    self.enigo.mouse_scroll_y(length);
  }

  #[napi]
  pub fn main_display_size(&mut self) -> Int32Array  {
    let (width, height) = self.enigo.main_display_size();
    Int32Array::new(vec![width, height])
  }

  #[napi]
  pub fn mouse_location(&mut self) -> Int32Array {
    let (x, y) = self.enigo.mouse_location();
    Int32Array::new(vec![x, y])
  }
}

