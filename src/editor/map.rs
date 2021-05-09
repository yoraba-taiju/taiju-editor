use taiju::chapter::prelude::*;

pub(crate) struct Map {
  scenario: Scenario,
  
}

impl Map {
  pub(crate) fn new(scenario: Scenario) -> Self {
    Self {
      scenario,
    }
  }
  pub(crate) fn process_events(&self) {
    
  }
}
