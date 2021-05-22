use bevy::prelude::*;
use crate::{model, state::MapState};

pub mod course;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut AppBuilder) {
    course::current_frame::init(app);
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
 ** Event
 ******************************************************************************/

#[derive(Debug)]
pub struct EventComponent {
  pub at: usize,
  pub event: taiju::chapter::scenario::Event,
}

#[derive(Debug, Bundle)]
pub struct EventBundle {
  pub event_component: EventComponent,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}

impl EventBundle {
  pub fn new(at: usize, event: taiju::chapter::scenario::Event) -> Self {
    Self {
      event_component: EventComponent {
        at,
        event,
      },
      global_transform: Default::default(),
      transform: Default::default(),
    }
  }
}

/******************************************************************************
 ** Inseet/Delete
 ******************************************************************************/

pub fn insert(
  commands: &mut Commands,
  color_materials: &mut ResMut<Assets<ColorMaterial>>,
  map: &model::Map,
) -> crate::state::MapState {
  let mut course_id = Entity::new(0);
  let mut current_frame_id = Entity::new(0);
  let map_id: Entity;

  map_id = commands.spawn_bundle(MapBundle::new())
  .with_children(|builder| {
    course_id = course::insert(builder, &map.course);

    current_frame_id = course::current_frame::insert(builder);

    for event in &map.events {
      builder.spawn_bundle(EventBundle::new(event.at as usize, event.event.clone()));
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