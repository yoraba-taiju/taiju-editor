use bevy::prelude::*;

use crate::model;

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

pub fn insert(
  commands: &mut ChildBuilder,
  color_materials: &mut ResMut<Assets<ColorMaterial>>,
  event: &model::map::Event,
) -> Entity {
  commands
    .spawn_bundle(EventBundle::new(event.at as usize, event.event.clone()))
    .id()
}
