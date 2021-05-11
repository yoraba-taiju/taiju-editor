use std::usize;

use bevy::prelude::*;
use taiju::chapter::{
  prelude::*,
  resources::scenario::Event,
};

#[derive(Debug, Default)]
pub struct Timeline {
  pub pos: Vec<Vec2>
}

impl Timeline {
  pub(crate) fn from(scenario: &Scenario) -> Self {
    let mut tl: Timeline = Default::default();
    tl.load(scenario);
    tl
  }
  fn load(&mut self, scenario: &Scenario) {
    let mut pos = Vec2::new(0.0, 0.0);
    let mut motion: Motion = Motion::Constant(0.0, 0.0);
    for (at, events) in scenario.events.iter() {
      while self.pos.len() <= (*at as usize) {
        pos = move_pos_by_motion(&pos, &motion);
        self.pos.push(pos);
      }
      for ev in events {
        match ev {
          Event::ChangeWitchSpeed(next) => { motion = *next; }
          Event::SpawnEnemy(_desc) => {}
        }
      }
    }
  }
  pub(crate) fn len(&self) -> usize {
    self.pos.len()
  }
}

fn move_pos_by_motion(pos: &Vec2, motion: &Motion) -> Vec2 {
  let mut pos = pos.clone();
  match motion {
    Motion::Constant(x, y) => {
      pos.x += *x;
      pos.y += *y;
    }
  }
  pos
}