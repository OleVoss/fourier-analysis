use bevy::{prelude::{AssetServer, Assets, Commands, OrthographicCameraBundle, Res, ResMut}, sprite::ColorMaterial};

use crate::ui::UIState;

pub fn setup(
    mut commands: Commands,
) {
    commands.insert_resource(UIState { open: false, start: false, image_path: None });
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}