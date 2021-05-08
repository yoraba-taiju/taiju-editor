use bevy::prelude::*;
use bevy_egui::{egui, egui::{InputState, Event}, EguiContext, EguiPlugin, };
use taiju::chapter::prelude::*;
use taiju::donut::Clock;
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

fn setup(
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

  // Editor
  let map_id = commands
    .spawn()
    .insert(editor::Map)
    .insert(Transform::identity())
    .id();
  commands.insert_resource(editor::Editor::new(map_id));
}

fn ui_menu(
  mut commands: Commands,
  egui_ctx: Res<EguiContext>,
  mut editor: ResMut<editor::Editor>,
  clock: Res<ClockRef>,
  asset_server: Res<AssetServer>,
) {
  let input: &egui::InputState = egui_ctx.ctx().input();
  for ev in input.events.iter() {
    match ev {
      &Event::Copy => {}
      &Event::Cut => {}
      Event::Text(str) => {}
      &Event::Key {
        key: Key,
        pressed: bool,
        modifiers: Modifiers,
      } => {}
      &Event::PointerMoved(pos) => {}
      &Event::PointerButton {
        pos,
        button,
        pressed,
        modifiers,
      } => {
        if pressed {
          if button == egui::PointerButton::Secondary {
            editor.menu_pos = Some((pos.x, pos.y));
          }
        }
      }
      &Event::PointerGone => {}
    }
  }
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

  if let Some(pos) = editor.menu_pos {
    egui::Window::new("Hello")
      .default_pos(pos)
      .show(egui_ctx.ctx(), |ui| {
        ui.label("world");
      });
  }
}