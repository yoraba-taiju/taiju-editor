use std::collections::HashMap;
use taiju::chapter::prelude::*;

#[derive(Debug, Default)]
pub struct Map {
  pub duration: usize,
  pub enemies: HashMap<u32, Vec<EnemyDescription>>,
  pub motion_changes: HashMap<u32, Motion>,
  pub pos: Vec<Vec2>,
}

impl Map {
  pub fn load(scenario: Scenario) -> Self {
    let mut map = Self {
      duration: 0,
      enemies: Default::default(),
      motion_changes: Default::default(),
      pos: Default::default(),
    };
    map.load_events(&scenario);
    map.update_pos();
    map
  }
  pub fn load_events(&mut self, scenario: &Scenario) {
    for (at, events) in scenario.events.iter() {
      self.duration = std::cmp::max(self.duration, *at as usize);
      let mut enemies = Vec::<EnemyDescription>::new();
      for ev in events.iter() {
        match ev {
          Event::ChangeWitchSpeed(next) => {
            self.motion_changes.insert(*at, next.clone());
          }
          Event::SpawnEnemy(desc) => {
            enemies.push(desc.clone());
          }
        }
      }
      self.enemies.insert(*at, enemies);
    }
  }
  fn update_pos(&mut self) {
    let mut pos = Vec2::new(0.0, 0.0);
    let mut motion: Motion = Motion::Constant(0.0, 0.0);
    for at in 0..=(self.duration as u32) {
      if self.motion_changes.contains_key(&at) {
        motion = self.motion_changes[&at].clone();
      }
      pos = move_pos_by_motion(&pos, &motion);
      self.pos.push(pos);
    }
  }
  pub fn compile() -> Scenario {
    todo!("todo!")
  }
}

// ----------------------------------------------------------------------------

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
