use bevy::prelude::*;

#[derive(Debug)]
pub struct MapState{
  pub map_id: Entity,
  pub course_id: Entity,
  pub current_frame_id: Entity,
}

#[derive(Debug)]
pub struct MapTransformState {
  pub scale: f32,
  pub pos: Vec2,
}

impl Default for MapTransformState {
  fn default() -> Self {
    Self {
      scale: 0.2,
      pos: Vec2::ZERO,
    }
  }
}