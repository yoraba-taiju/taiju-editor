use std::ops::DerefMut;
use bevy::{
  input::mouse::{MouseMotion, MouseWheel},
  prelude::*,
};
use bevy_egui::{egui, egui::Event, EguiContext};
use taiju::chapter::prelude::*;

pub struct Map;

pub struct Editor {
  pub map_id: Entity,
  // map_handling
  pub map_scale: f32,
  pub mouse_pos: Vec2,
  pub drag_start_mouse_pos: Vec2,
  pub drag_start_map_pos: Vec2,
  // 
  pub menu_pos: Option<(f32, f32)>,
}

impl Editor {
  pub(crate) fn spawn(mut commands: Commands) {
    let map_id = commands
      .spawn()
      .insert(Map)
      .insert(Transform::identity())
      .insert(GlobalTransform::identity())
      .id();
    commands.insert_resource(Self {
      map_id,
      map_scale: 1.0,
      mouse_pos: Default::default(),
      drag_start_mouse_pos: Default::default(),
      drag_start_map_pos: Default::default(),
      menu_pos: Default::default(),
    });
  }
  pub(crate) fn update_map(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut editor_res: ResMut<Editor>,
    enemy_server: Res<EnemyServer>,
    mut map_query: Query<(Entity, &mut Transform), With<Map>>,
  ) {
    let mut e = editor_res.deref_mut();
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
      let spr = enemy_server.sprites[&EnemyBody::Enemy01].clone();
      //spr.transform.translation.x = 
      commands.spawn().insert_bundle(spr).insert(Parent(map_id));
    }
  }
  pub(crate) fn update_ui (
    mut commands: Commands,
    egui_ctx: Res<EguiContext>,
    mut editor: ResMut<Editor>,
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
          if button == egui::PointerButton::Secondary {
            if pressed {
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
  
    let mut open = true;
    if let Some(pos) = editor.menu_pos {
      egui::Window::new("Hello")
        .open(&mut open)
        .default_pos(pos)
        .show(egui_ctx.ctx(), |ui| {
          ui.label("world");
        });
    }
  }
}
