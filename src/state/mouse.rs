use bevy::prelude::*;
#[derive(Debug, Default)]
pub struct MouseState {
  // Mouse clock
  pub left_button: MouseButtonState,
  pub middle_button: MouseButtonState,
  pub right_button: MouseButtonState,
  // Drag
  pub current_pos: Vec2,
  pub drag: MouseDragState,
}

#[derive(Debug, Default)]
pub struct MouseButtonState {
  pub counter: u32,
}

#[allow(dead_code)]
impl MouseButtonState {
  pub fn is_pressed(&self) -> bool {
    self.counter > 0
  }
  pub fn is_released(&self) -> bool {
    self.counter == 0
  }
  pub fn just_pressed(&self) -> bool {
    self.counter == 1
  }
  pub fn is_dragging(&self) -> bool {
    self.counter > 1
  }
}

#[derive(Debug)]
pub enum MouseDragState {
  Dragging { button: MouseButton, from: Vec2, to: Vec2, delta: Vec2 },
  NotDragging,
}

impl Default for MouseDragState {
  fn default() -> Self {
      Self::NotDragging
  }
}

#[allow(dead_code)]
impl MouseDragState {
  pub fn is_dragging(&self) -> bool {
    match self {
      &MouseDragState::Dragging { button: _, from: _, to: _, delta: _, } => true,
      &MouseDragState::NotDragging => false,
    }
  }
}