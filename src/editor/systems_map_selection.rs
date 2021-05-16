use bevy::prelude::*;
use taiju::chapter::prelude::*;
use crate::editor::*;

pub fn click_enemy(
  window_state: Res<WindowState>,
  mouse_state: Res<MouseState>,
  mouse_button_input: Res<Input<MouseButton>>,
  map_query: Query<(Entity, &GlobalTransform), With<MapAnchor>>,
  enemy_query: Query<(Entity, &GlobalTransform, &Sprite, &EnemyDescription), With<EnemyAnchor>>,
) {
  if !mouse_button_input.just_pressed(MouseButton::Left) {
    return;
  }
  let (_, map_trans) = map_query.single().unwrap();
  for (_entity, enemy_trans, sprite, desc) in enemy_query.iter() {
    let pos = Vec2::new(enemy_trans.translation.x, enemy_trans.translation.y);
    let size = Vec2::new(sprite.size.x * map_trans.scale.x, sprite.size.y * map_trans.scale.y);
    let mouse_pos = mouse_state.pos - (window_state.size/2.0);
    if contains(&pos, &size, &mouse_pos) {
    }
  }
}

fn contains(pos: &Vec2, size: &Vec2, point: &Vec2) -> bool {
  let beg = *pos - (*size / 2.0);
  let end = beg + *size;
  return beg.x <= point.x && point.x <= end.x && beg.y <= point.y && point.y <= end.y;
}