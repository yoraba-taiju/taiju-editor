use bevy::prelude::*;
use taiju::chapter::resources::scenario::Event;

#[derive(Debug, Default)]
pub struct Timeline {
  pub pos: Vec<Vec2>,
  pub events: Vec<(u32, Event)>
}

impl Timeline {
  pub(crate) fn new() -> Self {
    Default::default()
  }
}