use bevy::prelude::*;

use crate::state::MouseState;

pub fn update(
  mut mosue_state: ResMut<MouseState>,
  mut cursor_moved_events: EventReader<CursorMoved>,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  for event in cursor_moved_events.iter() {
    mosue_state.cursor_pos = event.position;
  }
  if mouse_button_input.just_pressed(MouseButton::Middle) {
    mosue_state.drag_origin = Some(mosue_state.cursor_pos);
  }
  if mouse_button_input.just_released(MouseButton::Middle) {
    mosue_state.drag_origin = None;
  }
}
