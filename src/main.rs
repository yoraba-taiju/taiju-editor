use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::build()
      .add_plugins(DefaultPlugins)
      .add_plugin(EguiPlugin)
      .add_startup_system(startup.system())
      .add_system(ui_example.system())
      .run();
}

fn startup(

) {

}

fn ui_example(mut egui_ctx: ResMut<EguiContext>) {
    egui::TopPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                ui.label("Open Scene");
                ui.indent("Open Source", |ui| {
                    if ui.button("Stage01").clicked() {
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