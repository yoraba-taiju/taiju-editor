use bevy::prelude::*;

use crate::component;
use crate::state::KeyboardState;

pub fn update(
  keyboard_state: Res<KeyboardState>,
  course_query: Query<&component::map::CourseComponent>,
  mut frame_query: Query<&mut component::map::course::current_frame::CurrentFrameComponent>,
) {
  let course = course_query.single();
  let frame = frame_query.single_mut();
  if course.is_err() || frame.is_err() {
    return;
  }
  let course = course.unwrap();
  let mut frame = frame.unwrap();
  if keyboard_state.left.should_take_action() && frame.at > 0 {
    frame.at -= 1;
  }
  if keyboard_state.right.should_take_action() && frame.at + 1 < course.length {
    frame.at += 1;
  }
}
