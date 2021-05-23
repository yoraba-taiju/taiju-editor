use bevy::prelude::*;
use crate::{component::map::SelectableComponent, state::{MouseState, SelectionState}};

pub fn update_selection(
  mut selection_state: ResMut<SelectionState>,
  mouse_state: Res<MouseState>,
  selectable_query: Query<(Entity, &GlobalTransform, &SelectableComponent)>,
) {
  if !mouse_state.left_button.just_pressed() {
    return;
  }
  selection_state.selected = None;
  let current_mouse_pos = mouse_state.current_pos;
  for (
    entity,
    global_transform,
    selectable_component,
  ) in selectable_query.iter() {
    let pos = Vec2::new(global_transform.translation.x, global_transform.translation.y);
    let size = Vec2::new(selectable_component.size.x * global_transform.scale.x, selectable_component.size.y * global_transform.scale.y);
    if contains(&pos, &size, &current_mouse_pos) {
      println!("Selected: {:?}", entity);
      selection_state.selected = Some(entity);
      break;
    }
  }
}

fn contains(pos: &Vec2, size: &Vec2, point: &Vec2) -> bool {
  let beg = *pos - (*size / 2.0);
  let end = beg + *size;
  return beg.x <= point.x && point.x <= end.x && beg.y <= point.y && point.y <= end.y;
}
