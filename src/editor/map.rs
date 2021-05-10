use taiju::chapter::prelude::*;

mod timeline;
use timeline::Timeline;

pub(crate) struct Map {
  scenario: Scenario,
  timeline: Timeline,
}

impl Map {
  pub(crate) fn load(scenario: Scenario) -> Self {
    Self {
      scenario,
      timeline: Timeline::new(),
    }
  }
  pub(crate) fn process_events(&self) {
  }
}
