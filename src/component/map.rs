use bevy::prelude::*;
use crate::{model, state::MapState};

pub mod course;
pub mod event;

/******************************************************************************
 ** Init(Plugin)
 ******************************************************************************/
 pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut AppBuilder) {
    course::current_frame::init(app);
    course::key_frame::init(app);
  }
}

/******************************************************************************
 ** Map
 ******************************************************************************/

#[derive(Debug, Default)]
pub struct MapComponent;
#[derive(Debug, Default, Bundle)]
pub struct MapBundle {
  pub map_component: MapComponent,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}

impl MapBundle {
  pub fn new() -> Self {
    Self {
      transform: Transform::from_scale(Vec3::new(0.2,0.2,0.2)),
      ..Default::default()
    }
  }
}

/******************************************************************************
 ** Selectable
 ******************************************************************************/
#[derive(Debug)]
pub struct SelectableComponent {
  pub size: Vec2,
}

impl SelectableComponent {
  pub fn new(size: Vec2) -> Self {
    Self {
      size,
    }
  }
}

/******************************************************************************
 ** Inseet/Delete
 ******************************************************************************/

pub fn insert(
  commands: &mut Commands,
  mut color_materials: &mut ResMut<Assets<ColorMaterial>>,
  mut meshes: &mut ResMut<Assets<Mesh>>,
  map: &model::Map,
) -> crate::state::MapState {
  let mut course_id = Entity::new(0);
  let mut current_frame_id = Entity::new(0);
  let map_id: Entity;

  map_id = commands.spawn_bundle(MapBundle::new())
  .with_children(|builder| {
    course_id = course::insert(builder, &mut meshes, &map.course);

    current_frame_id = course::current_frame::insert(builder);

    for event in &map.events {
      event::insert(builder, &mut color_materials, event);
    }
  }).id();
  crate::state::MapState {
    map_id,
    course_id,
    current_frame_id,
  }
}

pub fn clear(
  commands: &mut Commands,
  map_state: &MapState,
) {
  commands.entity(map_state.map_id).despawn_recursive();
}