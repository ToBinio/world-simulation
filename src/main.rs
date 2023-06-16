use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::camera::CameraPlugin;
use crate::chicken::ChickenPlugin;
use crate::entity::EntityPlugin;

mod camera;
mod chicken;
mod entity;

const MAP_SIZE: f32 = 5000.;
const HALF_MAP_SIZE: f32 = MAP_SIZE / 2.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(ChickenPlugin)
        .add_plugin(EntityPlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(setup_fps_text)
        .add_startup_system(setup_camera)
        .add_startup_system(background)
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

fn background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(MAP_SIZE, MAP_SIZE)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::SEA_GREEN)),
        ..default()
    });
}
