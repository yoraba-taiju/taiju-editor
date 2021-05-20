mod course;
pub use course::Course;

mod event;
pub use event::Event;

mod scape;
pub use scape::Scape;

use taiju::prelude::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Map {
  pub course: Course,
  pub events: Vec<Event>,
  pub scapes: Vec<Scape>,
}

impl Map {
  pub fn from_string(str: &str) -> ron::Result<Self> {
    ron::from_str::<Self>(str)
  }
  pub fn to_string(&self) -> ron::Result<String> {
    ron::to_string(self)
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test() {
    
  }
}