use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod model;
mod component;
mod state;
mod io;

mod system;

mod runtime;
mod bevy_render_primitive;

fn main() {

  let egui_systems =
    SystemSet::new()
      .with_system(system::egui::display_menu.system())
      .with_system(system::egui::display_timeline.system())
      .with_system(system::egui::display_edit.system());

  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(bevy_render_primitive::PrimitivePlugin)
    .add_startup_system(setup.system())
    .add_system_to_stage(CoreStage::PreUpdate, system::window::update.system())
    .add_system_set(egui_systems)
    .add_system_to_stage(CoreStage::PostUpdate, system::recalc_frames::on_changed.system())
    .run();
}

fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  ///////// Add Res /////////
  // runtime
  commands.insert_resource(runtime::Runtime::new());
  // io
  commands.insert_resource(io::map::MapToLoad(None));
  // Map
  let map = model::Map::default();
  let map_state = component::map::insert(&mut commands, &mut color_materials, &map);
  commands.insert_resource(map_state);
  commands.insert_resource(state::MapTransformState::default());
  commands.insert_resource(state::Frames::default());
  // Other gui
  commands.insert_resource(state::EguiState::default());
  commands.insert_resource(state::MouseState::default());
  commands.insert_resource(state::WindowState::default());

  ///////// Add Camera /////////
  // Spawn cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());

}
