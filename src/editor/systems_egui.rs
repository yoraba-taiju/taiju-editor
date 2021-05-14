use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use taiju::chapter::prelude::*;
use crate::runtime::Runtime;

use crate::editor::*;

pub fn display_ui(
  egui_ctx: Res<EguiContext>,
  runtime: Res<Runtime>,
  mut map_state: ResMut<MapState>,
  mut frame_state: ResMut<CurrentFrameState>,
  mut subwindow_state: ResMut<SubWindowState>,
) {
  let ctx = egui_ctx.ctx();
  egui::TopPanel::top("top_panel").show(ctx, |ui| {
    egui::menu::bar(ui, |ui| {
      egui::menu::menu(ui, "File", |ui| {
        ui.label("Open Scene");
        if ui.button("Chapter 01").clicked() {
          map_state.load_scenario(&runtime, "../taiju/assets/scenario/stage01.ron");
        }
        ui.separator();
        if ui.button("Save Scene").clicked() {
        }
        ui.separator();
        if ui.button("Quit").clicked() {
          std::process::exit(0);
        }
      });
      egui::menu::menu(ui, "Window", |ui|{
        ui.checkbox(&mut subwindow_state.open_timeline_window, "Timeline Window");
        ui.checkbox(&mut subwindow_state.open_editor_window, "Editor Window");
      });  
    });
  });
  egui::Window::new("Timeline")
    .title_bar(true)
    .open(&mut subwindow_state.open_timeline_window)
    .min_width(200.0)
    .show(ctx, |ui| {
      { // show bar
        let range = {
          if let Some(map) = map_state.map.as_ref() {
            0..=(map.timeline.len()-1) as u32
          } else {
            0..=100
          }
        };
        ui.horizontal(|ui| {
          ui.label("Time: ");
          ui.add(egui::DragValue::new(&mut frame_state.current_time).speed(1).clamp_range(range.clone()));
        });
        ui.spacing_mut().slider_width = ui.available_width();
        ui.add(egui::Slider::new(&mut frame_state.current_time, range.clone()).show_value(false).smart_aim(true));
      }
    });
  egui::Window::new("Edit Event/Object")
    .title_bar(true)
    .open(&mut subwindow_state.open_editor_window)
    .min_width(200.0)
    .show(ctx, |ui|{
    });
}

pub fn reload_map(
  mut map_state: ResMut<MapState>,
  map_query: Query<(Entity), With<MapAnchor>>,
  mut commands: Commands,
  mut enemy_server: ResMut<EnemyServer>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
  if let Some(map) = map_state.handle.take() {
    map_state.map = Some(map);
  } else {
    return;
  }
  // remove old maps
  for map_id in map_query.iter() {
    commands.entity(map_id).despawn_recursive();
  }
  let map = map_state.map.as_ref().unwrap();
  let map_id =  commands
    .spawn()
    .insert(MapAnchor)
    .insert(Transform::identity())
    .insert(GlobalTransform::identity())
    .id();
  let mut map_entities = Vec::<Entity>::new();
  {
    for (at, events) in map.scenario.events.iter() {
      let pos = map.timeline.pos[*at as usize];
      {
        let id = commands.spawn().insert_bundle(SpriteBundle{
          sprite: Sprite::new(Vec2::new(1920.0, 1080.0)),
          material: color_materials.add(Color::rgba(0.5, 0.5, 1.0, 0.5).into()),
          transform: Transform::from_xyz(pos.x, pos.y, 0.0),
          ..Default::default()
        }).id();
        map_entities.push(id);
      }
      for e in events.iter() {
        match e.clone() {
            Event::ChangeWitchSpeed(_) => {}
            Event::SpawnEnemy(desc) => {
              let mut spr = enemy_server.sprites[&desc.body].clone();
              spr.transform.translation.x = desc.position.x + pos.x;
              spr.transform.translation.y = desc.position.y + pos.y;
              let id = commands.spawn().insert_bundle(spr).id();
              map_entities.push(id);
            }
        }
      }
    }
  }
  {
    let id = commands.spawn().insert_bundle(SpriteBundle{
      sprite: Sprite::new(Vec2::new(1920.0, 1080.0)),
      material: color_materials.add(Color::rgba(1.0, 0.5, 0.5, 0.5).into()),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..Default::default()
    }).insert(FrameAnchor).id();
    map_entities.push(id);
  }

  commands.entity(map_id).push_children(&map_entities);
}

