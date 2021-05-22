use std::collections::HashMap;
use taiju::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
  pub length: usize,
  pub keyframes: HashMap<usize, Vec2>,
}

impl Course {
  pub fn new() -> Self {
    Self::default()
  }
}

const DEFAULT_COURSE_LENGTH: usize = 100;

impl Default for Course {
  fn default() -> Self {
    let mut keyframes: HashMap<usize, Vec2> = HashMap::new();
    keyframes.insert(0, Vec2::new(0.0, 0.0));
    keyframes.insert(DEFAULT_COURSE_LENGTH-1, Vec2::new(100.0, 0.0));
    Self {
      length: DEFAULT_COURSE_LENGTH,
      keyframes,
    }
  }
}