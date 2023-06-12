//! Shows how to render simple primitive shapes with a single color.

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_chickens)
        .add_startup_system(setup_fps_text)
        .add_system(move_chickens)
        .add_system(check_for_boarder)
        .add_system(flip_sprite)
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

fn setup_chickens(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();

    for _ in 0..10000 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("Chicken.png"),
                transform: Transform::from_scale(Vec3::new(10., 10., 0.)),
                ..default()
            },
            Direction {
                direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                    .normalize(),
            },
        ));
    }
}

#[derive(Component)]
struct Direction {
    direction: Vec2,
}

fn move_chickens(mut chickens: Query<(&mut Transform, &Direction)>, time: Res<Time>) {

    for (mut transform, direction) in &mut chickens {
        let translate = &mut transform.translation;

        translate.x += direction.direction.x * 100.0 * time.delta_seconds();
        translate.y += direction.direction.y * 100.0 * time.delta_seconds();
    }
}

fn check_for_boarder(
    mut chickens: Query<(&mut Transform, &mut Direction)>,
    windows: Query<&Window>,
) {
    let window = windows.single();

    let max_width: f32 = window.width() / 2.;
    let max_height: f32 = window.height() / 2.;

    for (mut transform, mut direction) in &mut chickens {
        let translation = &mut transform.translation;

        if translation.x > max_width {
            direction.direction.x = -direction.direction.x;
            translation.x = max_width;
        }

        if translation.x < -max_width {
            direction.direction.x = -direction.direction.x;
            translation.x = -max_width;
        }

        if translation.y > max_height {
            direction.direction.y = -direction.direction.y;
            translation.y = max_height;
        }

        if translation.y < -max_height {
            direction.direction.y = -direction.direction.y;
            translation.y = -max_height;
        }
    }
}

fn flip_sprite(mut chickens: Query<(&mut Transform, &Direction)>) {
    for (mut transform, direction) in &mut chickens {
        if direction.direction.x > 0. && transform.scale.x < 0. {
            transform.scale.x = -transform.scale.x;
        }

        if direction.direction.x < 0. && transform.scale.x > 0. {
            transform.scale.x = -transform.scale.x;
        }
    }
}
