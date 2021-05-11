mod map;
mod runtime;

use std::{fs::File, io::Read, ops::DerefMut, sync::Arc, usize};
use bevy::{
  input::mouse::MouseWheel,
  prelude::*,
};
use bevy_egui::{egui, EguiContext};
use taiju::chapter::prelude::*;
use runtime::{Runtime, Handle};
use map::Map;

pub struct MapAnchor;
pub struct FrameAnchor;

pub(crate) struct Editor {
  runtime: Runtime,
  // map loading
  map_handle: Arc<Handle<Map>>,
  map: Option<Map>,
  // map_handling
  map_scale: f32,
  mouse_pos: Vec2,
  drag_start_mouse_pos: Vec2,
  drag_start_map_pos: Vec2,
  //
  window_size: Vec2,
  // 
  current_time: u32,
}

impl Editor {
  pub(crate) fn spawn(
    mut commands: Commands,
    windows: Res<Windows>,
  ) {
    let window = windows.get_primary().unwrap();
    commands
      .spawn()
      .insert(MapAnchor)
      .insert(Transform::identity())
      .insert(GlobalTransform::identity());
    commands.insert_resource(Self {
      runtime: Runtime::new(),
      map_handle: Default::default(),
      map: Default::default(),
      map_scale: 1.0,
      mouse_pos: Default::default(),
      drag_start_mouse_pos: Default::default(),
      drag_start_map_pos: Default::default(),
      window_size: Vec2::new(window.width(), window.height()),
      current_time: 0,
    });
  }
  pub(crate) fn update_map(
    mut editor_res: ResMut<Editor>,
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    enemy_server: Res<EnemyServer>,
    mut map_query: Query<(Entity, &mut Transform), With<MapAnchor>>,
  ) {
    let mut e = editor_res.deref_mut();
    let (map_id, mut map_trans) = map_query.single_mut().unwrap();
    let window = windows.get_primary().unwrap();
    e.window_size = Vec2::new(window.width(), window.height());
    // zoom map
    for event in mouse_wheel_events.iter() {
      e.map_scale += event.y / 5.0;
      if e.map_scale < 0.2 {
        e.map_scale = 0.2;
      }
      map_trans.scale.x = e.map_scale;
      map_trans.scale.y = e.map_scale;
      map_trans.scale.z = e.map_scale;
    }
    // move map
    for event in cursor_moved_events.iter() {
      e.mouse_pos = event.position;
    }
    if mouse_button_input.just_pressed(MouseButton::Middle) {
      e.drag_start_map_pos.x = map_trans.translation.x;
      e.drag_start_map_pos.y = map_trans.translation.y;
      e.drag_start_mouse_pos = e.mouse_pos;
    } else if mouse_button_input.pressed(MouseButton::Middle) {
      map_trans.translation.x = e.drag_start_map_pos.x + (e.mouse_pos.x - e.drag_start_mouse_pos.x);
      map_trans.translation.y = e.drag_start_map_pos.y + (e.mouse_pos.y - e.drag_start_mouse_pos.y);
    }
    // modify map
    if mouse_button_input.just_pressed(MouseButton::Right) {
      let mut spr = enemy_server.sprites[&EnemyBody::Enemy01].clone();
      let pt = (e.mouse_in_map() - e.window_size/2.0 - Vec2::new(map_trans.translation.x, map_trans.translation.y)) / e.map_scale;
      spr.transform.translation.x = pt.x;
      spr.transform.translation.y = pt.y;
      // Parentを指定する方法だと駄目だった。バグ？
      let id = commands.spawn().insert_bundle(spr).id();
      commands.entity(map_id).push_children(&[id]);
    }
  }
  pub(crate) fn update_frame (
    mut editor_res: ResMut<Editor>,
    mut frame_query: Query<&mut Transform, With<FrameAnchor>>,
  ) {
    let e = editor_res.deref_mut();
    if e.map.is_none() {
      return;
    }
    let mut frame_transform = frame_query.single_mut().unwrap();
    let pos = e.map.as_ref().unwrap().timeline.pos[e.current_time as usize];
    frame_transform.translation.x = pos.x;
    frame_transform.translation.y = pos.y;
  }
  pub(crate) fn update_ui (
    egui_ctx: Res<EguiContext>,
    mut editor_res: ResMut<Editor>,
  ) {
    let e = editor_res.deref_mut();
    egui::TopPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
      egui::menu::bar(ui, |ui| {
        egui::menu::menu(ui, "File", |ui| {
          ui.label("Open Scene");
          ui.indent("Open Source", |ui| {
            if ui.button("Chapter 01").clicked() {
              e.load_scenario("../taiju/assets/scenario/stage01.ron");
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
    egui::SidePanel::left("timeline_side_panel", 200.0).show(egui_ctx.ctx(), |ui| {
      ui.heading("Timeline");
      ui.separator();
    });
    egui::SidePanel::left("edit_side_panel", 200.0).show(egui_ctx.ctx(), |ui| {
      ui.heading("Edit Event/Object");
      ui.separator();
    });
    egui::TopPanel::top("scroll_bar").show(egui_ctx.ctx(), |ui| {
      let range = {
        if let Some(map) = e.map.as_ref() {
          0..=(map.timeline.len()-1) as u32
        } else {
          0..=100
        }
      };
      ui.horizontal(|ui| {
        ui.label("Time: ");
        ui.add(egui::DragValue::new(&mut e.current_time).speed(1).clamp_range(range.clone()));
      });
      ui.spacing_mut().slider_width = ui.available_width();
      ui.add(egui::Slider::new(&mut e.current_time, range.clone()).show_value(false).smart_aim(true));
   });
  }
  pub(crate) fn reload_map(
    mut editor_res: ResMut<Editor>,
    mut map_query: Query<(Entity, &mut Transform), With<MapAnchor>>,
    mut commands: Commands,
    mut enemy_server: ResMut<EnemyServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
  ) {
    let mut e = editor_res.deref_mut();
    if let Some(map) = e.map_handle.poll() {
      e.map = Some(map);
    } else {
      return;
    }
    { // remove old maps
      let (map_id, mut map_trans) = map_query.single_mut().unwrap();
      commands.entity(map_id).despawn_recursive();
    }
    let map = e.map.as_ref().unwrap();
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
        for e in events {
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
  pub(crate) fn mouse_in_map(&self) -> Vec2 {
    Vec2::new(self.mouse_pos.x, self.mouse_pos.y)
  }
  pub(crate) fn load_scenario(&mut self, path: &str) {
    let path = path.to_owned();
    self.map_handle = self.runtime.spawn(async move {
      let path = path;
      let mut bytes = Vec::new();
      File::open(path).unwrap().read_to_end(&mut bytes).unwrap();
      let scenario = ron::from_str::<Scenario>(std::str::from_utf8(&bytes).unwrap()).unwrap();
      let map = Map::load(scenario);
      map
    });
  }

}
