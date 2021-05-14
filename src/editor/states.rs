use std::sync::Arc;
use bevy::prelude::*;
use crate::runtime::Handle;
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
