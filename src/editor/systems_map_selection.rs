use bevy::prelude::*;
//use taiju::chapter::prelude::*;
use crate::editor::*;

pub fn select_game_event(
  window_state: Res<WindowState>,
  mut map_state: ResMut<MapState>,
  mouse_state: Res<MouseState>,
  mouse_button_input: Res<Input<MouseButton>>,
  map_query: Query<(Entity, &GlobalTransform), With<MapAnchor>>,
  mut enemy_query: Query<(Entity, &GlobalTransform, &Sprite, &mut GameEvent)>,
) {
  if !mouse_button_input.just_pressed(MouseButton::Left) {
    return;
  }
  let (_, map_trans) = map_query.single().unwrap();
  for (entity, enemy_trans, sprite, mut event) in enemy_query.iter_mut() {
    let pos = Vec2::new(enemy_trans.translation.x, enemy_trans.translation.y);
    let size = Vec2::new(sprite.size.x * map_trans.scale.x, sprite.size.y * map_trans.scale.y);
    let mouse_pos = mouse_state.pos - (window_state.size/2.0);
    if contains(&pos, &size, &mouse_pos) {
      println!("selected: {:?}", entity);
      if map_state.selected.contains(&entity) {
        map_state.selected.remove(&entity);
      } else {
        map_state.selected.insert(entity);
      }
    }
  }
}

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

