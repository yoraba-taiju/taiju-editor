use taiju::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
  pub at: u32,
  pub event: taiju::chapter::scenario::Event,
}
