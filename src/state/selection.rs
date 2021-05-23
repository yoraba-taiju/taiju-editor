use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SelectionState {
  pub selected: Option<Entity>,
}
