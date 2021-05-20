use std::collections::HashMap;
use taiju::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Course {
  pub length: usize,
  pub keyframes: HashMap<usize, Vec2>,
}

impl Course {
  pub fn new() -> Self {
    Self::default()
  }
}
