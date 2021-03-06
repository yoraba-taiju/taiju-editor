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
  let input_system_set = 
    SystemSet::new()
      .with_system(system::window::update_state.system())
      .with_system(system::mouse::update_state.system())
      .with_system(system::keyboard::update_state.system());

  let egui_system_set =
    SystemSet::new()
      .with_system(system::egui::display_menu.system())
      .with_system(system::egui::display_timeline.system())
      .with_system(system::egui::display_edit.system());

  let selection_system_set =
    SystemSet::new()
      .with_system(system::selection::update_selection.system())
      .with_system(system::selection::move_selected.system());

  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(bevy_render_primitive::PrimitivePlugin)
    .add_plugin(component::map::MapPlugin)
    .add_plugin(component::selection::SelectionPlugin)
    .add_startup_system(setup.system())
    .add_system_set_to_stage(CoreStage::PreUpdate, input_system_set)
    .add_system_set_to_stage(CoreStage::Update, egui_system_set)
    .add_system_set_to_stage(CoreStage::Update, selection_system_set)
    .add_system_to_stage(CoreStage::Update, system::map::course::current_frame::update.system())
    .add_system_to_stage(CoreStage::Update, system::map::course::route::update_on_changed.system())
    .add_system_to_stage(CoreStage::PostUpdate, system::map::course::recalc_positions::on_changed.system())
    .run();
}

fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
  asset_server: Res<AssetServer>,
) {
  ///////// Add Res /////////
  // runtime
  commands.insert_resource(runtime::Runtime::new());
  // io
  commands.insert_resource(io::map::MapToLoad(None));
  // Map
  let map = model::Map::default();
  let map_state = component::map::insert(&mut commands, &mut color_materials, &mut meshes, &map);
  commands.insert_resource(map_state);
  commands.insert_resource(state::MapTransformState::default());
  commands.insert_resource(state::Frames::default());
  // Other gui
  commands.insert_resource(state::EguiState::default());
  commands.insert_resource(state::MouseState::default());
  commands.insert_resource(state::KeyboardState::default());
  commands.insert_resource(state::WindowState::default());

  ///////// Add Camera /////////
  // Spawn cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());

}
