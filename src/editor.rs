use bevy::prelude::*;
use bevy_egui::egui;

pub struct Map;

pub struct Editor {
  pub map_id: Entity,
  pub menu_pos: Option<(f32, f32)>,
}

impl Editor {
  pub(crate) fn new(map_id: Entity) -> Self {
    Self {
      map_id,
      menu_pos: Default::default(),
    }
  }
}
