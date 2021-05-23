use bevy::prelude::*;
use crate::component::selection::SelectableComponent;
use crate::component::selection::SelectedComponent;
use crate::state::MouseState;

pub fn update_selection(
  mut commands: Commands,
  mouse_state: Res<MouseState>,
  selected_query: Query<Entity, With<SelectedComponent>>,
  selectable_query: Query<(Entity, &GlobalTransform, &SelectableComponent)>,
) {
  if !mouse_state.left_button.just_pressed() {
    return;
  }
  for entity in selected_query.iter() {
    commands.entity(entity).despawn_recursive();
  }
  let current_mouse_pos = mouse_state.current_pos;
  for (
    entity,
    global_transform,
    selectable_component,
  ) in selectable_query.iter() {
    let pos = Vec2::new(global_transform.translation.x, global_transform.translation.y);
    let size = Vec2::new(selectable_component.size.x * global_transform.scale.x, selectable_component.size.y * global_transform.scale.y);
    if contains(&pos, &size, &current_mouse_pos) {
      commands.entity(entity).with_children(|commands| {
        commands
          .spawn_bundle(SpriteBundle {
            sprite: Sprite {
              size: selectable_component.size,
              resize_mode: SpriteResizeMode::Manual,
              ..Default::default()
            },
            material: crate::component::selection::SELECTED_COLOR_MATERIAL_HANDLE.typed(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)),
            ..Default::default()
          })
          .insert(SelectedComponent);
      });
    }
  }
}

fn contains(pos: &Vec2, size: &Vec2, point: &Vec2) -> bool {
  let beg = *pos - (*size / 2.0);
  let end = beg + *size;
  return beg.x <= point.x && point.x <= end.x && beg.y <= point.y && point.y <= end.y;
}
