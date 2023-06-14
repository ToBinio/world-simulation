//! Shows how to render simple primitive shapes with a single color.

mod chicken;
mod entity;

use crate::chicken::ChickenPlugin;
use crate::entity::EntityPlugin;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(ChickenPlugin)
        .add_plugin(EntityPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_fps_text)
        .add_system(update_fps)
        .run();
}

fn setup_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font: asset_server.load("fonts/Roboto-Black.ttf"),
            font_size: 60.0,
            color: Color::WHITE,
        })]),
        FpsText,
    ));
}

#[derive(Component)]
struct FpsText;

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[0].value = format!("{value:.2}");
            }
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
