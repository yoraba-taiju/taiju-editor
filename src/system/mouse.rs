use bevy::prelude::*;

use crate::state::{MouseButtonState, MouseDragState, MouseState, WindowState};

pub fn update_state(
  mut mosue_state: ResMut<MouseState>,
  window_state: Res<WindowState>,
  mut cursor_moved_events: EventReader<CursorMoved>,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  let update = |key: MouseButton, state: &mut MouseButtonState| {
    if mouse_button_input.pressed(key) {
      state.counter += 1;
    } else {
      state.counter = 0;
    }  
  };
  update(MouseButton::Left, &mut mosue_state.left_button);
  update(MouseButton::Middle, &mut mosue_state.middle_button);
  update(MouseButton::Right, &mut mosue_state.right_button);

  let mut current_pos = mosue_state.current_pos;
  for event in cursor_moved_events.iter() {
    current_pos = event.position - (window_state.size/2.0);
  }
  mosue_state.current_pos = current_pos;
  if let &mut MouseDragState::Dragging { button: _, from: _, ref mut to, ref mut delta} = &mut mosue_state.drag {
    *delta = current_pos - *to;
    *to = current_pos;
  }
  for button in mouse_button_input.get_just_pressed() {
    mosue_state.drag = MouseDragState::Dragging {
      button: *button, 
      from: mosue_state.current_pos,
      to: mosue_state.current_pos,
      delta: Vec2::ZERO,
    };
    break;
  }
  for _ in mouse_button_input.get_just_released() {
    mosue_state.drag = MouseDragState::NotDragging;
    break;
  }
}
