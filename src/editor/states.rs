use std::{fs::File, io::Read};
use std::sync::Arc;
use bevy::prelude::*;
use taiju::chapter::prelude::*;
use crate::runtime::{Runtime, Handle};
use crate::map::Map;

#[derive(Debug, Default)]
pub struct WindowState{
  pub size: Vec2,
}

#[derive(Debug, Default)]
pub struct MapState {
  pub handle: Arc<Handle<Map>>,
  pub map: Option<Map>,
  pub scale: f32,
  pub pos: Vec2,
  pub drag_origin: Option<Vec2>,
}

impl MapState {
  pub fn load_scenario(&mut self, rt: &Runtime, path: &str) {
    let path = path.to_owned();
    self.handle = rt.spawn(async move {
      let path = path;
      let mut bytes = Vec::new();
      File::open(path).unwrap().read_to_end(&mut bytes).unwrap();
      let scenario = ron::from_str::<Scenario>(std::str::from_utf8(&bytes).unwrap()).unwrap();
      let map = Map::load(scenario);
      map
    });
  }
}

#[derive(Debug, Default)]
pub struct CurrentFrameState {
  pub current_time: u32,
}

#[derive(Debug, Default)]
pub struct MouseState {
  pub pos: Vec2,
  pub drag_origin: Option<Vec2>,
}

#[derive(Debug, Default)]
pub struct SubWindowState {
  pub open_timeline_window: bool,
  pub open_editor_window: bool,
}
