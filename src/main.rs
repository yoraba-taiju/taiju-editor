use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use taiju::*;
use bevy::ecs::system::Command;

mod runtime;
mod editor;

fn main() {
  std::env::set_var("CARGO_MANIFEST_DIR", std::env::current_dir().expect("No current dir??").join("..").join("taiju"));
  println!("Set CARGO_MANIFEST_DIR to {:?}", std::env::var("CARGO_MANIFEST_DIR").expect("Failed to read env variable."));
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(StagePlugin)
    .init_resource::<runtime::Runtime>()
    .insert_resource(Clock::new())
    .add_startup_system(setup.system())
    .add_system(ui_menu.system())
    .run();
}

pub fn setup(
  mut commands: Commands,
  mut _rt: ResMut<runtime::Runtime>,
  clock: Res<ClockRef>,
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

fn ui_menu(
  mut commands: Commands,
  egui_ctx: Res<EguiContext>,
  clock: Res<ClockRef>,
  asset_server: Res<AssetServer>,
) {
  egui::TopPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
    egui::menu::bar(ui, |ui| {
      egui::menu::menu(ui, "File", |ui| {
        ui.label("Open Scene");
        ui.indent("Open Source", |ui| {
          if ui.button("Stage01").clicked() {
            commands.remove_resource::<Handle<Scenario>>();
            let handle = asset_server.load::<Scenario, _>("scenario/stage01.ron");
            commands.insert_resource(handle);
          }
        });
        ui.separator();
        if ui.button("Save Scene").clicked() {
        }
        ui.separator();
        if ui.button("Quit").clicked() {
          std::process::exit(0);
        }
      });
    });
  });
  egui::Window::new("Hello")
    .default_pos((100.0, 100.0))
    .show(egui_ctx.ctx(), |ui| {
      ui.label("world");
    });
}