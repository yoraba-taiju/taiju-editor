mod map;
mod runtime;

use runtime::Runtime;

use std::ops::DerefMut;
use bevy::{
  input::mouse::MouseWheel,
  prelude::*,
};
use bevy_egui::{egui, egui::Event, EguiContext};
use taiju::chapter::prelude::*;

pub struct MapAnchor;

pub(crate) struct Editor {
  runtime: Runtime,
  // map_handling
  map_scale: f32,
  mouse_pos: Vec2,
  drag_start_mouse_pos: Vec2,
  drag_start_map_pos: Vec2,
  //
  window_size: Vec2,
  // 
  menu_pos: Option<(f32, f32)>,
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
      map_scale: 1.0,
      mouse_pos: Default::default(),
      drag_start_mouse_pos: Default::default(),
      drag_start_map_pos: Default::default(),
      window_size: Vec2::new(window.width(), window.height()),
      menu_pos: Default::default(),
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
    let window = windows.get_primary().unwrap();
    e.window_size = Vec2::new(window.width(), window.height());
    let (map_id, mut map_trans) = map_query.single_mut().unwrap();
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
  pub(crate) fn update_ui (
    egui_ctx: Res<EguiContext>,
    mut editor_res: ResMut<Editor>,
  ) {
    let e = editor_res.deref_mut();
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
          /*
          if button == egui::PointerButton::Secondary {
            if pressed {
              e.menu_pos = Some((pos.x, pos.y));
            }
          }*/
        }
        &Event::PointerGone => {}
      }
    }
    egui::TopPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
      egui::menu::bar(ui, |ui| {
        egui::menu::menu(ui, "File", |ui| {
          ui.label("Open Scene");
          ui.indent("Open Source", |ui| {
            if ui.button("Chapter 01").clicked() {
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
  
    /*
    let mut open = true;
    if let Some(pos) = e.menu_pos {
      egui::Window::new("Hello")
        .open(&mut open)
        .default_pos(pos)
        .show(egui_ctx.ctx(), |ui| {
          ui.label("world");
        });
    }*/
  }
  pub(crate) fn mouse_in_map(&self) -> Vec2 {
    Vec2::new(self.mouse_pos.x, self.mouse_pos.y)
  }
}
