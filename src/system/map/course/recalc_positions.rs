use std::collections::HashMap;

use bevy::prelude::*;
use crate::state::Frames;
use crate::component::map::course::route::Route;
use crate::component::map::course::CourseComponent;
use crate::component::map::course::key_frame::KeyFrameComponent;

pub fn on_changed(
  mut frames: ResMut<Frames>,
  mut route: Query<&mut Route>,
  course_query: Query<&CourseComponent>,
  keyframe_query: Query<(Entity, &KeyFrameComponent, &Transform)>,
  //
  course_changed_query: Query<&CourseComponent, Changed<CourseComponent>>,
  changed_query: Query<(Entity, &KeyFrameComponent, &Transform), Changed<Transform>>,
  added_query: Query<(Entity, &KeyFrameComponent, &Transform), Added<KeyFrameComponent>>,
  removed_query: RemovedComponents<KeyFrameComponent>,
) {
  let course_component = if let Ok(course_component) = course_query.single() {
    course_component
  } else {
    return;
  };
  let mut empty = true;
  if empty {
      for _ in changed_query.iter() {
      empty = false;
      break;
    }
  }
  if empty {
    for _ in course_changed_query.iter() {
      empty = false;
      break;
    }  
  }
  if empty {
    for _ in added_query.iter() {
      empty = false;
      break;
    }  
  }
  if empty {
      for _ in removed_query.iter() {
      empty = false;
      break;
    }
  }
  if empty {
    return;
  }
  let mut keyframs = HashMap::<usize, Vec2>::new();
  for (
    _entity,
    KeyFrameComponent { at },
    transform
  ) in keyframe_query.iter()
  {
    keyframs.insert(*at, Vec2::new(transform.translation.x, transform.translation.y));
  }
  let mut keyframes = keyframs.into_iter().collect::<Vec<(usize, Vec2)>>();
  keyframes.sort_by_key(|(k, _v)| *k);
  //
  if let Ok(mut route) = route.single_mut() {
    route.route_to_update = Some(keyframes.iter().map(|(_, v)| *v).collect::<Vec<Vec2>>());
  }

  // recalc all position
  let mut positions = Vec::<Vec2>::new();
  if keyframes.is_empty() {
    frames.positions = positions;
    return;
  }
  {
    let (beg_keyframe_idx, beg_keyframe_vec) = keyframes[0].clone();
    for _i in 0..=beg_keyframe_idx {
      positions.push(beg_keyframe_vec);
    }
  }
  let mut last_idx: usize = 0;
  let mut last_vec = Vec2::ZERO;
  for j in 1..keyframes.len() {
    let (beg_keyframe_idx, beg_keyframe_vec) = keyframes[j-1].clone();
    let (end_keyframe_idx, end_keyframe_vec) =keyframes[j].clone();
    let length =(end_keyframe_idx - beg_keyframe_idx) as f32;
    for i in (beg_keyframe_idx+1)..=end_keyframe_idx {
      let d = (i - beg_keyframe_idx) as f32;
      last_vec = (beg_keyframe_vec * (length - d) / length) + (end_keyframe_vec * d / length);
      last_idx = i;
      positions.push(last_vec);
    }
  }
  for _ in last_idx..course_component.length {
    positions.push(last_vec);
  }
  frames.positions = positions;
}