use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
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
            0..=(map.duration as u32)
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
