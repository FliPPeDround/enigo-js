use enigo::{Axis, Button, Coordinate, Direction};

pub fn get_button(button: i32) -> Button {
  match button {
      0 => Button::Left,
      1 => Button::Middle,
      2 => Button::Right,
      3 => Button::ScrollUp,
      4 => Button::ScrollDown,
      5 => Button::ScrollLeft,
      6 => Button::ScrollRight,
      _ => unreachable!(),
  }
}

pub fn get_direction(direction: i32) -> Direction {
  match direction {
      0 => Direction::Press,
      1 => Direction::Release,
      2 => Direction::Click,
      _ => unreachable!(),
  }
}

pub fn get_coordinate(coordinate: i32) -> Coordinate {
  match coordinate {
     0 => Coordinate::Abs,
     1 => Coordinate::Rel,
     _ => unreachable!(),
  }
}

pub fn get_axis(axis: i32) -> Axis {
  match axis {
      0 => Axis::Horizontal,
      1 => Axis::Vertical,
      _ => unreachable!(),
  }
}
