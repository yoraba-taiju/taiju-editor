use bevy::prelude::*;
use crate::editor::editor_states::*;

pub fn update_mouse_state(
  mut mosue_state: ResMut<MouseState>,
  mut map_state: ResMut<MapState>,
  mut cursor_moved_events: EventReader<CursorMoved>,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  for event in cursor_moved_events.iter() {
    mosue_state.pos = event.position;
  }
  if mouse_button_input.just_pressed(MouseButton::Middle) {
    mosue_state.drag_origin = Some(mosue_state.pos);
    map_state.drag_origin = Some(map_state.pos);
  }
  if mouse_button_input.just_released(MouseButton::Middle) {
    mosue_state.drag_origin = None;
    map_state.drag_origin = None;
  }
}
