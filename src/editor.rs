mod states;

use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::{egui, EguiContext};
use taiju::chapter::prelude::*;
use crate::runtime::Runtime;

pub use states::*;

pub struct MapAnchor;
pub struct FrameAnchor;

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

pub fn update_window_state(
  mut window_state: ResMut<WindowState>,
  windows: Res<Windows>,
){
  let window = windows.get_primary().unwrap();
  window_state.size = Vec2::new(window.width(), window.height());
}

pub fn update_mouse_state(
  mut mosue_state: ResMut<MouseState>,
  mut map_state: ResMut<MapState>,
  mut cursor_moved_events: EventReader<CursorMoved>,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  for event in cursor_moved_events.iter() {
    mosue_state.pos = event.position;
  }
  if mouse_button_input.just_pressed(MouseButton::Middle) {
    mosue_state.drag_origin = Some(mosue_state.pos);
    map_state.drag_origin = Some(map_state.pos);
  }
  if mouse_button_input.just_released(MouseButton::Middle) {
    map_state.drag_origin = None;
  }
}

pub fn update_map_trans(
  mut map_state: ResMut<MapState>,
  mouse_state: Res<MouseState>,
  mut mouse_wheel_events: EventReader<MouseWheel>,
  mut map_query: Query<(Entity, &mut Transform), With<MapAnchor>>,
) {
  let (map_id, mut map_trans) = map_query.single_mut().unwrap();
  for event in mouse_wheel_events.iter() {
    map_state.scale += event.y / 5.0;
    if map_state.scale < 0.1 {
      map_state.scale = 0.1;
    }
    map_trans.scale.x = map_state.scale;
    map_trans.scale.y = map_state.scale;
    map_trans.scale.z = map_state.scale;
  }
  if let Some(start) = mouse_state.drag_origin {
    let delta = mouse_state.pos - start;
    if let Some(start) = mouse_state.drag_origin {
      map_state.pos = start + delta;
    }
  }
  map_trans.translation.x = map_state.pos.x;
  map_trans.translation.y = map_state.pos.y;
}

pub fn update_current_frame(
  keyboard_input: Res<Input<KeyCode>>,
  mut map_state: ResMut<MapState>,
  mut frame_state: ResMut<CurrentFrameState>,
  mut map_query: Query<(Entity, &mut Transform), With<MapAnchor>>,
) {
  let (map_id, mut map_trans) = map_query.single_mut().unwrap();
  let map = if let Some(map) = map_state.map.as_ref() {
    map
  } else {
    return;
  };
  
  let mut changed = false;
  if keyboard_input.pressed(KeyCode::Left) {
    if frame_state.current_time > 0 {
      frame_state.current_time -= 1;
      changed = true;
    }
  }
  if keyboard_input.pressed(KeyCode::Right) {
    frame_state.current_time = std::cmp::min((map.timeline.pos.len() as u32) - 1, frame_state.current_time + 1);
    changed = true;
  }
  if changed {
    let pos = map.timeline.pos[frame_state.current_time as usize];
    map_state.pos.x = (-pos.x) * map_state.scale;
    map_state.pos.y = (-pos.y) * map_state.scale;
  }
}

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

pub fn update_frame (
  mut map_state: ResMut<MapState>,
  frame_state: Res<CurrentFrameState>,
  mut frame_query: Query<&mut Transform, With<FrameAnchor>>,
) {
  let map = if let Some(map) = map_state.map.as_ref() {
    map
  } else {
    return;
  };
  let mut frame_transform = frame_query.single_mut().unwrap();
  let pos = map.timeline.pos[frame_state.current_time as usize];
  frame_transform.translation.x = pos.x;
  frame_transform.translation.y = pos.y;
}

pub fn reload_map(
  mut map_state: ResMut<MapState>,
  mut map_query: Query<(Entity, &mut Transform), With<MapAnchor>>,
  mut commands: Commands,
  mut enemy_server: ResMut<EnemyServer>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
  if let Some(map) = map_state.handle.take() {
    map_state.map = Some(map);
  } else {
    return;
  }
  { // remove old maps
    let (map_id, mut map_trans) = map_query.single_mut().unwrap();
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

