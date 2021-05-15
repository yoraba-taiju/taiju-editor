use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use editor::CameraAnchor;
use taiju::chapter::prelude::*;
use taiju::donut::Clock;

mod editor;
mod map;
mod runtime;
mod bevy_render_primitive;
mod test;

fn main() {
  std::env::set_var("CARGO_MANIFEST_DIR", std::env::current_dir().expect("No current dir??").join("..").join("taiju"));
  println!("Set CARGO_MANIFEST_DIR to {:?}", std::env::var("CARGO_MANIFEST_DIR").expect("Failed to read env variable."));
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(StagePlugin)
//    .add_plugin(bevy_render_primitive::PrimitiveRendererPlugin)
    .insert_resource(Clock::new())
    .add_startup_system(setup.system())
    .add_startup_system(test::setup.system())
    .add_startup_system(editor::spawn_resources.system())
    .add_startup_system(editor::spawn_map_anchor.system())
    .add_system_to_stage(CoreStage::PreUpdate, editor::update_window_state.system())
    .add_system_to_stage(CoreStage::PreUpdate, editor::update_mouse_state.system())
    .add_system_to_stage(CoreStage::PreUpdate, editor::move_map.system())
    .add_system_to_stage(CoreStage::PreUpdate, editor::move_current_frame.system())
    
    .add_system_to_stage(CoreStage::Update, editor::display_ui.system())
    .add_system_to_stage(CoreStage::Update, editor::update_frame.system())
    .add_system_to_stage(CoreStage::Update, editor::click_enemy.system())
    .add_system_to_stage(CoreStage::Update, test::animate_shader.system())

    .add_system_to_stage(CoreStage::PostUpdate, editor::reload_map.system())
    .run();
}

fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  // Resources to load
  EnemyServer::spawn(&mut commands, &asset_server, &mut color_materials, &mut texture_atlases);
  BulletServer::spawn(&mut commands, &asset_server, &mut color_materials, &mut texture_atlases);

  // cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(CameraAnchor);
  commands.spawn_bundle(UiCameraBundle::default());

/*  commands.spawn_bundle(
    bevy_render_primitive::LineStripBuilder::new_loop()
      .append(Vec3::new(-100.0, 100.0, 0.0))
      .append(Vec3::new(-100.0, -100.0, 0.0))
      .append(Vec3::new( 100.0, -100.0, 0.0))
      .append(Vec3::new( 100.0, 100.0, 0.0))
      .build(&mut meshes));
*/
}
