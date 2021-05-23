use bevy::prelude::*;

use crate::state::WindowState;

pub fn update_state(
  mut window_state: ResMut<WindowState>,
  windows: Res<Windows>,
){
  let window = windows.get_primary().unwrap();
  window_state.size = Vec2::new(window.width(), window.height());
}
