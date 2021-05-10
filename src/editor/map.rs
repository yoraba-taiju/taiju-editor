use taiju::chapter::prelude::*;

mod timeline;
use timeline::Timeline;

pub(crate) struct Map {
  scenario: Scenario,
  timeline: Timeline,
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
