use bevy::prelude::*;
#[derive(Debug, Default)]
pub struct MouseState {
  pub cursor_pos: Vec2,
  pub drag_origin: Option<Vec2>,
}
