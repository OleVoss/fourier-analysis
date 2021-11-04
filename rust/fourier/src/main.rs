#![allow(clippy::needless_return)]

use std::path::PathBuf;

use bevy::prelude::*;
use bevy::{
    input::mouse::MouseWheel,
    prelude::{App, EventWriter, IntoSystem, Res, ResMut},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;

mod setup;
use rfd::FileDialog;
use setup::setup;

mod ui;
use ui::{ui, UIState};

pub struct LoadImageEvent;
pub struct Image;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup.system())
        .add_event::<LoadImageEvent>()
        .add_system(ui.system())
        .add_system(controller.system())
        .add_system(load_image_listener.system())
        .add_system(mouse_zoom_resize.system())
        .run();
}

fn controller(mut ui_state: ResMut<UIState>, mut events: EventWriter<LoadImageEvent>) {
    if ui_state.open {
        ui_state.image_path = open_file();
        if ui_state.image_path.is_some() {
            events.send(LoadImageEvent {});
        }
    }
}

fn load_image_listener(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<LoadImageEvent>,
    ui_state: Res<UIState>,
    mut query: Query<Entity, With<Image>>,
) {
    if events.iter().last().is_some() {
        if let Some(p) = &ui_state.image_path {
            for entity in query.iter_mut() {
                commands.entity(entity).despawn();
            }

            let material_handle = materials.add(assets.load(p.as_path()).into());
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    material: material_handle,
                    ..Default::default()
                })
                .insert(Image);
        }
    }
}

fn mouse_zoom_resize(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Sprite, With<Image>>,
) {
    if let Some(event) = mouse_wheel_events.iter().last() {
        for mut sprite in query.iter_mut() {
            sprite.resize_mode = SpriteResizeMode::Manual;
            let new_height = sprite.size.y as f32 + sprite.size.y as f32 * 0.25 * event.y;
            let new_width = sprite.size.x as f32 + sprite.size.x as f32 * 0.25 * event.y;
            sprite.size = Vec2::new(new_width, new_height);
        }
    }
}

fn open_file() -> Option<PathBuf> {
    let path_to_open = std::env::current_dir().unwrap();

    let path = FileDialog::new()
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Images", &["jpg", "jpeg"])
        .set_directory(path_to_open)
        .pick_file();

    println!("{:?}", path);
    return path;
}
