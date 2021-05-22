
use std::sync::Arc;

use crate::runtime::Handle;
pub struct MapToLoad(pub Option<Arc<Handle<crate::model::Map>>>);

impl Default for MapToLoad {
  fn default() -> Self {
    Self(None)
  }
}