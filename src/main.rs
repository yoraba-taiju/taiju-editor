use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use taiju::chapter::prelude::*;
use taiju::donut::Clock;

mod editor;

fn main() {
  std::env::set_var("CARGO_MANIFEST_DIR", std::env::current_dir().expect("No current dir??").join("..").join("taiju"));
  println!("Set CARGO_MANIFEST_DIR to {:?}", std::env::var("CARGO_MANIFEST_DIR").expect("Failed to read env variable."));
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(StagePlugin)
    .insert_resource(Clock::new())
    .add_startup_system(setup.system())
    .add_startup_system(editor::Editor::spawn.system())
    .add_system_to_stage(CoreStage::PreUpdate, editor::Editor::update_map.system())
    .add_system_to_stage(CoreStage::Update, editor::Editor::update_ui.system())
    .add_system_to_stage(CoreStage::Update, editor::Editor::update_frame.system())
    .add_system_to_stage(CoreStage::PostUpdate, editor::Editor::reload_map.system())
    .run();
}

fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  // Resources to load
  EnemyServer::spawn(&mut commands, &asset_server, &mut color_materials, &mut texture_atlases);
  BulletServer::spawn(&mut commands, &asset_server, &mut color_materials, &mut texture_atlases);

  // cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
}
