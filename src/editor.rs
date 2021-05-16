use bevy::prelude::*;
use crate::runtime::Runtime;

mod editor_anchors;
mod editor_states;
mod systems_egui;
mod systems_window;
mod systems_map;
mod systems_map_reloading;
mod systems_map_selection;
mod game_events;
pub use editor_anchors::*;
pub use editor_states::*;
pub use systems_egui::*;
pub use systems_window::*;
pub use systems_map::*;
pub use systems_map_reloading::*;
pub use systems_map_selection::*;
pub use game_events::*;

pub fn spawn_resources(
  mut commands: Commands,
  windows: Res<Windows>,
) {
  let window = windows.get_primary().unwrap();
  commands.insert_resource(Runtime::new());
  commands.insert_resource(WindowState{
    size: Vec2::new(window.width(), window.height()),
  });
  commands.insert_resource(MapState::default());
  commands.insert_resource(CurrentFrameState::default());
  commands.insert_resource(MouseState::default());
  commands.insert_resource(SubWindowState::default());
}

pub fn spawn_map_anchor(
  mut commands: Commands,
) {
  commands
    .spawn()
    .insert(MapAnchor)
    .insert(Transform::identity())
    .insert(GlobalTransform::identity());
}

