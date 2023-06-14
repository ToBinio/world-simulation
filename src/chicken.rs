use std::time::Duration;
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::entity::Target;

pub struct ChickenPlugin;

impl Plugin for ChickenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_chickens)
            .add_system(spawn_eggs)
            .add_system(hatch_eggs)
            .add_system(grow_chickens);
    }
}

#[derive(Component)]
pub struct EggSpawnTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct EggHatchTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Chicken {
    state: ChickenState,
}

#[derive(Component)]
pub enum ChickenState {
    Child(Timer),
    Adult,
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

fn spawn_eggs(mut chickens: Query<(&Transform, &mut EggSpawnTimer)>, time: Res<Time>, mut commands: Commands, asset_server: Res<AssetServer>) {
    for (transform, mut egg_timer) in &mut chickens {
        egg_timer.timer.tick(time.delta());

        if egg_timer.timer.finished() {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("Egg.png"),
                    transform: Transform {
                        translation: transform.translation,
                        scale: transform.scale,
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
                    transform: *transform,
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