#![deny(clippy::all)]

use enigo::*;
use napi::bindgen_prelude::Int32Array;
mod mouse_button;
pub use crate::mouse_button::*; 

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

  #[napi]
  /// Get the delay per keypress.
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// const delay = enigo.getDelay();
  /// console.log(delay);
  /// ```
  /// 
  /// @return {number} delay - The delay per keypress in milliseconds.
  pub fn get_delay(&self) -> u32 {
    self.enigo.delay()
  }

  #[napi]
  /// Set the delay per keypress.
  /// 
  /// ### Example
  /// ```js
  /// import { Enigo } from '@enigo-js/core'
  /// 
  /// const enigo = new Enigo();
  /// enigo.setDelay(100);
  /// ```
  /// 
  /// @param {number} delay - The delay per keypress in milliseconds.
  pub fn set_delay(&mut self, delay: u32) {
    self.enigo.set_delay(delay);
  }

  // #[napi]
  // /// Returns a list of all currently pressed keys.
  // /// 
  // /// ### Example
  // /// ```js
  // /// import { Enigo } from '@enigo-js/core'
  // /// 
  // /// const enigo = new Enigo();
  // /// const keys = enigo.held();
  // /// console.log(keys);
  // /// ```
  // /// 
  // /// @return {Array<Key>} keys - The list of currently pressed keys.
  // pub fn held(&self) -> Array {
  //   let list = self.enigo.held();
  //   Array::new(vec![list])
  // }

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
  pub fn button(&mut self, button: i32, direction: i32) {
    self.enigo.button(get_button(button), get_direction(direction)).unwrap();
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
  pub fn move_mouse(&mut self, x: i32, y: i32, coordinate: i32) {
    self.enigo.move_mouse(x, y, get_coordinate(coordinate)).unwrap();
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
  pub fn scroll(&mut self, length: i32, axis: i32) {
    self.enigo.scroll(length, get_axis(axis)).unwrap();
  }

  #[napi]
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
  /// @return {[number, number]} size - The width and height of the main display in pixels.
  pub fn main_display(&self) -> Int32Array {
    let (width, height) = self.enigo.main_display().unwrap();
    Int32Array::new(vec![width, height])
  }

  #[napi]
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
  /// @return {[number, number]} location - The x and y coordinates of the mouse in pixels.
  pub fn location(&self) -> Int32Array {
    let (x, y) = self.enigo.location().unwrap();
    Int32Array::new(vec![x, y])
  }
}



