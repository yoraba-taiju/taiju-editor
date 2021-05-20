use std::{fs::File, io::Read, ops::RangeInclusive};
use bevy::prelude::*;
use bevy_egui::*;
use crate::{component::map::{CourseComponent, CourseCurrentFrameComponent, MapComponent}, state::*};
use crate::runtime::*;
use crate::io::map::MapToLoad;
use crate::model::Map;

pub fn display_menu(
  egui_ctx: Res<EguiContext>,
  runtime: Res<Runtime>,
  mut egui_state: ResMut<EguiState>,
  mut map_to_load: ResMut<MapToLoad>,
) {
  let ctx = egui_ctx.ctx();
  egui::TopPanel::top("top_panel").show(ctx, |ui| {
    egui::menu::bar(ui, |ui| {
      egui::menu::menu(ui, "File", |ui| {
        if ui.button("Load Scene").clicked() {
          let handle = runtime.spawn(async {
            let mut bytes = Vec::new();
            File::open("./current_map.ron").unwrap().read_to_end(&mut bytes).unwrap();
            let map =Map::from_string(std::str::from_utf8(&bytes).unwrap()).unwrap();
            map
          });
          map_to_load.0 = Some(handle);
        }
        if ui.button("Save Scene").clicked() {
        }
        ui.separator();
        if ui.button("Quit").clicked() {
          std::process::exit(0);
        }
      });
      egui::menu::menu(ui, "Window", |ui|{
        ui.checkbox(&mut egui_state.open_timeline_window, "Timeline Window");
        ui.checkbox(&mut egui_state.open_editor_window, "Editor Window");
      });  
    });
  });
}

pub fn display_timeline(
  egui_ctx: Res<EguiContext>,
  mut egui_state: ResMut<EguiState>,
  course_query: Query<&CourseComponent>,
  mut frame_query: Query<&mut CourseCurrentFrameComponent>,
) {
  let course = course_query.single();
  let frame = frame_query.single_mut();
  let ctx = egui_ctx.ctx();
  egui::Window::new("Timeline")
    .title_bar(true)
    .open(&mut egui_state.open_timeline_window)
    .min_width(200.0)
    .show(ctx, |ui| {
      { // show bar
        let range: RangeInclusive<usize> = {
          if let Ok(course) = course {
            0..=course.length
          } else {
            0..=0
          }
        };
        let mut zero: usize = 0;
        let mut current_time: &mut usize = &mut zero;
        // if frame.is_ok() {
        //   current_time = &mut (frame.unwrap().at);
        // }
        ui.horizontal(|ui| {
          ui.label("Time: ");
          ui.add(egui::DragValue::new(current_time).speed(1).clamp_range(range.clone()));
        });
        ui.spacing_mut().slider_width = ui.available_width();
        ui.add(egui::Slider::new(current_time, range.clone()).show_value(false).smart_aim(true));
      }
    });
}

pub fn display_edit (
  egui_ctx: Res<EguiContext>,
  runtime: Res<Runtime>,
  mut egui_state: ResMut<EguiState>,
) {
  let ctx = egui_ctx.ctx();
  egui::Window::new("Edit Event/Object")
    .title_bar(true)
    .open(&mut egui_state.open_editor_window)
    .min_width(200.0)
    .show(ctx, |ui|{
    });
}