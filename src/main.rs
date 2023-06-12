//! Shows how to render simple primitive shapes with a single color.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("Chicken.png"),
        transform: Transform::from_scale(Vec3::new(10., 10., 0.)),
        ..default()
    });
}
