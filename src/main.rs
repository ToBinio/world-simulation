//! Shows how to render simple primitive shapes with a single color.

use std::time::Duration;
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
        .add_system(select_new_target)
        .add_system(spawn_eggs)
        .add_system(hatch_eggs)
        .add_system(flip_sprite)
        .add_system(update_fps)
        .add_system(grow_chickens)
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

    for _ in 0..5 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("Chicken.png"),
                transform: Transform::from_scale(Vec3::new(10., 10., 0.)),
                ..default()
            },
            Target {
                target: Vec2::new(rng.gen_range(-500.0..500.0), rng.gen_range(-500.0..500.0)),
            },
            EggSpawnTimer {
                timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating)
            },
            Chicken {
                state: ChickenState::Adult
            }
        ));
    }
}

#[derive(Component)]
struct Target {
    target: Vec2,
}

#[derive(Component)]
struct EggSpawnTimer {
    timer: Timer,
}

#[derive(Component)]
struct EggHatchTimer {
    timer: Timer,
}

#[derive(Component)]
struct Chicken {
    state: ChickenState,
}

#[derive(Component)]
enum ChickenState {
    Child(Timer),
    Adult,
}

fn move_chickens(mut chickens: Query<(&mut Transform, &Target)>, time: Res<Time>) {
    for (mut transform, direction) in &mut chickens {
        let translate = &mut transform.translation;

        let angle = (direction.target.y - translate.y).atan2(direction.target.x - translate.x);

        translate.x += angle.cos() * 100.0 * time.delta_seconds();
        translate.y += angle.sin() * 100.0 * time.delta_seconds();
    }
}


fn select_new_target(mut entities: Query<(&Transform, &mut Target)>) {
    let mut rng = thread_rng();

    for (transform, mut target) in &mut entities {
        let distance = target.target.distance(Vec2::new(transform.translation.x, transform.translation.y));

        if distance < 5. {
            target.target = Vec2::new(rng.gen_range(-500.0..500.0), rng.gen_range(-500.0..500.0));
        }
    }
}

fn flip_sprite(mut chickens: Query<(&mut Transform, &Target)>) {
    for (mut transform, target) in &mut chickens {
        if target.target.x > transform.translation.x && transform.scale.x < 0. {
            transform.scale.x = -transform.scale.x;
        }

        if target.target.x < transform.translation.x && transform.scale.x > 0. {
            transform.scale.x = -transform.scale.x;
        }
    }
}

fn spawn_eggs(mut chickens: Query<(&Transform, &mut EggSpawnTimer)>, time: Res<Time>, mut commands: Commands, asset_server: Res<AssetServer>) {
    for (transform, mut eggTimer) in &mut chickens {
        eggTimer.timer.tick(time.delta());

        if eggTimer.timer.finished() {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("Egg.png"),
                    transform: Transform {
                        translation: transform.translation.clone(),
                        scale: transform.scale.clone(),
                        ..default()
                    },
                    ..default()
                },
                EggHatchTimer {
                    timer: Timer::new(Duration::from_secs(5), TimerMode::Once)
                })
            );
        }
    }
}

fn hatch_eggs(mut eggs: Query<(Entity, &Transform, &mut EggHatchTimer)>, time: Res<Time>, mut commands: Commands, asset_server: Res<AssetServer>) {
    for (entity, transform, mut egg_timer) in &mut eggs {
        egg_timer.timer.tick(time.delta());

        if egg_timer.timer.finished() {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("Small_Chicken.png"),
                    transform: transform.clone(),
                    ..default()
                },
                Target {
                    target: Vec2::new(transform.translation.x, transform.translation.y),
                },
                EggSpawnTimer {
                    timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating)
                },
                Chicken {
                    state: ChickenState::Child(Timer::new(Duration::from_secs(5), TimerMode::Once))
                }
            ));

            commands.entity(entity).despawn();
        }
    }
}

fn grow_chickens(mut chickens: Query<(&mut Handle<Image>, &mut Chicken)>, time: Res<Time>, asset_server: Res<AssetServer>) {
    for (mut texture, mut chicken) in &mut chickens {
        if let ChickenState::Child(timer) = &mut chicken.state {
            timer.tick(time.delta());

            if timer.finished() {
                chicken.state = ChickenState::Adult;

                *texture = asset_server.load("Chicken.png");
            }
        }
    }
}