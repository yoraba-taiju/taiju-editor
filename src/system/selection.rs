use bevy::prelude::*;
use crate::{component::selection::SelectableComponent, state::MouseDragState};
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

pub fn move_selected(
  mut commands: Commands,
  mouse_state: Res<MouseState>,
  selected_query: Query<(Entity, &Parent), With<SelectedComponent>>,
  mut parent_query: Query<(Entity, &mut Transform, &GlobalTransform)>,
) {
  let delta = if let MouseDragState::Dragging {
    button: MouseButton::Left,
    from: _,
    to: _,
    delta,
  } = mouse_state.drag {
    delta
  } else {
    return;
  };
  for (_entity, &Parent(parent_id)) in selected_query.iter() {
    for (_entity, ref mut transform, global_transform) in parent_query.get_mut(parent_id).iter_mut() {
      transform.translation.x += delta.x / global_transform.scale.x;
      transform.translation.y += delta.y / global_transform.scale.y;
    }
  }
}