use bevy::prelude::*;
use bevy::tasks::{TaskPool, TaskPoolBuilder};

pub(crate) struct Runtime {
  pub(crate) task_pool: TaskPool,
}

impl Runtime {
  pub(crate) fn new() -> Self {
    Self {
      task_pool: TaskPoolBuilder::new().build(),
    }
  }
}

impl FromWorld for Runtime {
  fn from_world(_world: &mut World) -> Self {
    Runtime::new()
  }
}