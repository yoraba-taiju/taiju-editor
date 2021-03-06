use bevy::{input::mouse::MouseWheel, prelude::*};
use crate::editor::*;

pub fn move_map(
  mut map_state: ResMut<MapState>,
  mouse_state: Res<MouseState>,
  mut mouse_wheel_events: EventReader<MouseWheel>,
  mut map_query: Query<(Entity, &mut Transform), With<MapAnchor>>,
) {
  let (_, mut map_trans) = map_query.single_mut().unwrap();
  for event in mouse_wheel_events.iter() {
    map_state.scale += event.y / 20.0;
    if map_state.scale < 0.1 {
      map_state.scale = 0.1;
    }
  }
  if let Some(start) = mouse_state.drag_origin {
    let delta = mouse_state.pos - start;
    if let Some(start) = map_state.drag_origin {
      map_state.pos = start + delta;
    }
  }
  // Write down all states to map_trans
  map_trans.scale.x = map_state.scale;
  map_trans.scale.y = map_state.scale;
  map_trans.scale.z = map_state.scale;
  map_trans.translation.x = map_state.pos.x;
  map_trans.translation.y = map_state.pos.y;
}

pub fn update_frame (
  map_state: Res<MapState>,
  frame_state: Res<CurrentFrameState>,
  mut frame_query: Query<&mut Transform, With<FrameAnchor>>,
) {
  let map = if let Some(map) = map_state.map.as_ref() {
    map
  } else {
    return;
  };
  let mut frame_trans = frame_query.single_mut().unwrap();
  let pos = map.pos[frame_state.current_time as usize];
  frame_trans.translation.x = pos.x;
  frame_trans.translation.y = pos.y;
}
