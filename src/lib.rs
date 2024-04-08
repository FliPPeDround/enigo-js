#![deny(clippy::all)]

use enigo::*;
use napi::bindgen_prelude::{Int32Array, Uint16Array};
mod enum_mapping;
pub use crate::enum_mapping::*;

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
      enigo: Enigo::new(&Settings::default()).unwrap(),
    }
  }

  // #[napi]
  // /// Get the delay per keypress.
  // /// 
  // /// ### Example
  // /// ```js
  // /// import { Enigo } from '@enigo-js/core'
  // /// 
  // /// const enigo = new Enigo();
  // /// const delay = enigo.getDelay();
  // /// console.log(delay);
  // /// ```
  // /// 
  // /// @returns {number} delay - The delay per keypress in milliseconds.
  // pub fn get_delay(&self) -> u32 {
  //   self.enigo.delay()
  // }

  // #[napi]
  // /// Set the delay per keypress.
  // /// 
  // /// ### Example
  // /// ```js
  // /// import { Enigo } from '@enigo-js/core'
  // /// 
  // /// const enigo = new Enigo();
  // /// enigo.setDelay(100);
  // /// ```
  // /// 
  // /// @param {number} delay - The delay per keypress in milliseconds.
  // pub fn set_delay(&mut self, delay: u32) {
  //   self.enigo.set_delay(delay);
  // }

  #[napi]
  /// Returns a list of all currently pressed keys.
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// const keys = enigo.held();
  /// console.log(keys);
  /// ```
  /// 
  /// @returns {Array<Key>} keys - The list of currently pressed keys.
  pub fn held(&mut self) -> Uint16Array {
    let (_keys, platform_keys) = self.enigo.held();
    Uint16Array::new(platform_keys)
  }

  #[napi]
  /// Sends an individual mouse button event. 
  /// You can use this for example to simulate a click of the left mouse key.
  /// 
  /// ### Example
  /// ```js
  /// import { Button, Direction, Enigo } from '@enigo-js/core'
  ///
  /// const enigo = new Enigo();
  /// enigo.button(Button.Left, Direction.Click);
  /// ```
  /// 
  /// @param {Button} button - The button to press.
  /// @param {Direction} direction - The direction of the button press.
  pub fn button(&mut self, button: JsButton, direction: JsDirection) {
    self.enigo.button(Button::from(button), Direction::from(direction)).unwrap();
  }

  #[napi]
  /// Move the mouse cursor to the specified x and y coordinates.
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo, Coordinate } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// enigo.moveMouse(100, 100, Coordinate.Rel);
  /// ```
  /// 
  /// @param {number} x - The x coordinate.
  /// @param {number} y - The y coordinate.
  /// @param {Coordinate} coordinate - coordinate is relative or absolute
  pub fn move_mouse(&mut self, x: i32, y: i32, coordinate: JsCoordinate) {
    self.enigo.move_mouse(x, y, Coordinate::from(coordinate)).unwrap();
  }

  #[napi]
  /// Send a mouse scroll event.
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo, Axis } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// enigo.scroll(100, Axis.Horizontal);
  /// ```
  /// 
  /// @param {number} length - The scroll length.
  /// @param {Axis} axis - The axis of the scroll.
  pub fn scroll(&mut self, length: i32, axis: JsAxis) {
    self.enigo.scroll(length, Axis::from(axis)).unwrap();
  }

  #[napi(ts_return_type="readonly [width: number, height: number]")]
  /// Get the [width, height] of the main display in pixels.
  /// This currently only works on the main display.
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// const [width, height] = enigo.mainDisplay();
  /// console.log(width, height);
  /// ```
  /// 
  /// @returns {[width: number, height: number]} size - The width and height of the main display in pixels.
  pub fn main_display(&self) -> Int32Array {
    let (width, height) = self.enigo.main_display().unwrap();
    Int32Array::new(vec![width, height])
  }

  #[napi(ts_return_type="readonly [x: number, y: number]")]
  /// Get the location of the mouse in pixels
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// const [x, y] = enigo.location();
  /// console.log(x, y);
  /// ```
  /// 
  /// @returns {[x: number, y: number]} location - The x and y coordinates of the mouse in pixels.
  pub fn location(&self) -> Int32Array {
    let (x, y) = self.enigo.location().unwrap();
    Int32Array::new(vec![x, y])
  }

  #[napi]
  /// Sends an individual key event.
  /// It will enter the keysym (virtual key).
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo, Key, Direction } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// enigo.key(Key.A, Direction.Press);
  /// ```
  /// 
  /// @param {Key} key - The key to press.
  /// @param {Direction} direction - The direction of the key press.
  pub fn key(&mut self, key: JsKey, direction: JsDirection) {
    self.enigo.key(Key::from(key), Direction::from(direction)).unwrap();
  }

  #[napi]
  /// Sends a raw keycode.
  /// The keycode may or may not be mapped on the current layout.
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo, Direction } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// enigo.raw(13, Direction.Press);
  /// ```
  /// 
  /// @param {number} keycode - The keycode to press.
  /// @param {Direction} direction - The direction of the key press.
  pub fn raw(&mut self, keycode: u16, direction: JsDirection) {
    self.enigo.raw(keycode, Direction::from(direction)).unwrap();
  }

  #[napi]
  /// Sends a key event for a single character.
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// enigo.text("Hello World!");
  /// ```
  /// 
  /// @param {string} text - The text to type.
  pub fn text(&mut self, text: String) {
    self.enigo.text(&text).unwrap();
  }
}



