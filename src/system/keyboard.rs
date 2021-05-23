use bevy::prelude::*;

use crate::state::{KeyState, KeyboardState};

pub fn update_state(
  keyboard_input: Res<Input<KeyCode>>,
  mut keyboard_state: ResMut<KeyboardState>,
) {
  let update = |key: KeyCode, state: &mut KeyState| {
    if keyboard_input.pressed(key) {
      state.counter += 1;
    } else {
      state.counter = 0;
    }
  };
  update(KeyCode::Left, &mut keyboard_state.left);
  update(KeyCode::Right, &mut keyboard_state.right);
  update(KeyCode::LShift, &mut keyboard_state.shift);
  update(KeyCode::LControl, &mut keyboard_state.ctrl);
  update(KeyCode::S, &mut keyboard_state.s);
  update(KeyCode::Delete, &mut keyboard_state.delete);
}
