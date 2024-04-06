#![deny(clippy::all)]

use enigo::*;
use napi::bindgen_prelude::Int32Array;

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

  #[napi]
  pub fn mouse_click(&mut self, button: String) {
    let button_mapping = [("left", MouseButton::Left), ("right", MouseButton::Right), ("middle", MouseButton::Middle)];
    let button = button_mapping.iter().find(|(name, _)| *name == button).unwrap().1;
    self.enigo.mouse_click(button);
  }

  #[napi]
  pub fn mouse_move_to(&mut self, x: i32, y: i32) {
    self.enigo.mouse_move_to(x, y);
  }

  #[napi]
  pub fn mouse_location(&mut self) -> Int32Array {
    let (x, y) = self.enigo.mouse_location();
    Int32Array::new(vec![x, y])
  }
}

