use taiju::chapter::prelude::*;

mod timeline;
use timeline::Timeline;

#[derive(Debug, Default)]
pub struct Map {
  pub scenario: Scenario,
  pub timeline: Timeline,
}

impl Map {
  pub fn load(scenario: Scenario) -> Self {
    let timeline = Timeline::from(&scenario);
    Self {
      scenario,
      timeline,
    }
  }
  pub fn compile() -> Scenario {
    todo!("todo!")
  }
}
