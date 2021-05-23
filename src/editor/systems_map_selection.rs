use bevy::prelude::*;
//use taiju::chapter::prelude::*;
use crate::editor::*;

pub fn update_game_object_visibility(
  map_state: Res<MapState>,
  mut enemy_query: Query<(Entity, &mut ColorMaterial, &mut GameEvent)>,
) {
  for (entity, mut color, _event) in enemy_query.iter_mut() {
    if map_state.selected.contains(&entity) {
      color.color = Color::rgba(1.0, 0.7, 0.7, 1.0);
    } else {
      color.color = Color::rgba(1.0, 1.0, 1.0, 0.5);
    }
  }
}
