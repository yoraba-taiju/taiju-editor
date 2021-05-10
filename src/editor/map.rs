use taiju::chapter::prelude::*;

mod timeline;
use timeline::Timeline;

#[derive(Debug, Default)]
pub(crate) struct Map {
  pub scenario: Scenario,
  pub timeline: Timeline,
}

impl Map {
  pub(crate) fn load(scenario: Scenario) -> Self {
    let timeline = Timeline::from(&scenario);
    Self {
      scenario,
      timeline,
    }
  }
}
