use std::path::PathBuf;

use bevy::prelude::ResMut;
use bevy_egui::{EguiContext, egui};

#[derive(Clone)]
pub struct UIState {
    pub open: bool,
    pub start: bool,
    pub image_path: Option<PathBuf>,
}
pub fn ui(egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UIState>) {
    egui::Window::new("Fourier Analysis")
        .collapsible(false)
        .show(egui_context.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui_state.open = ui.button("Open").clicked();
                ui_state.start = ui.button("Start").clicked();
            });
            if let Some(path) = &ui_state.image_path {
                ui.label(path.to_str().unwrap());
            }
    });

}