use crate::entity::Target;
use bevy::prelude::*;
use std::time::Duration;

pub struct ChickenPlugin;

impl Plugin for ChickenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_chickens)
            .add_system(spawn_eggs)
            .add_system(hatch_eggs)
            .add_system(grow_chickens);
    }
}

#[derive(Bundle)]
pub struct ChickenBundle {
    egg_spawn_timer: EggSpawnTimer,
    target: Target,
    chicken_data: ChickenData,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl ChickenBundle {
    pub fn adult(asset_server: &Res<AssetServer>) -> Self {
        ChickenBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("Chicken.png"),
                transform: Transform::from_scale(Vec3::new(5., 5., 0.)),
                ..default()
            },
            chicken_data: ChickenData::default(),
            target: Target::default(),
            egg_spawn_timer: EggSpawnTimer::default(),
        }
    }

    pub fn child(asset_server: &Res<AssetServer>, transform: &Transform) -> Self {
        ChickenBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("Small_Chicken.png"),
                transform: *transform,
                ..default()
            },
            chicken_data: ChickenData {
                state: ChickenState::Child(Timer::new(Duration::from_secs(5), TimerMode::Once)),
            },
            target: Target::default(),
            egg_spawn_timer: EggSpawnTimer::default(),
        }
    }
}

#[derive(Bundle)]
pub struct EggBundle {
    egg_hatch_timer: EggHatchTimer,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl EggBundle {
    pub fn from_chicken(asset_server: &Res<AssetServer>, transform: &Transform) -> Self {
        EggBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("Egg.png"),
                transform: Transform {
                    translation: transform.translation,
                    scale: transform.scale,
                    ..default()
                },
                ..default()
            },
            egg_hatch_timer: EggHatchTimer::default(),
        }
    }
}

#[derive(Component)]
pub struct EggSpawnTimer {
    timer: Timer,
}

impl Default for EggSpawnTimer {
    fn default() -> Self {
        EggSpawnTimer {
            timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct EggHatchTimer {
    timer: Timer,
}

impl Default for EggHatchTimer {
    fn default() -> Self {
        EggHatchTimer {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
        }
    }
}

#[derive(Component, Default)]
pub struct ChickenData {
    state: ChickenState,
}

#[derive(Component, Default)]
pub enum ChickenState {
    Child(Timer),
    #[default]
    Adult,
}

fn setup_chickens(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..5 {
        commands.spawn(ChickenBundle::adult(&asset_server));
    }
}

fn spawn_eggs(
    mut chickens: Query<(&Transform, &mut EggSpawnTimer)>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (transform, mut egg_timer) in &mut chickens {
        egg_timer.timer.tick(time.delta());

        if egg_timer.timer.finished() {
            commands.spawn(EggBundle::from_chicken(&asset_server, transform));
        }
    }
}

fn hatch_eggs(
    mut eggs: Query<(Entity, &Transform, &mut EggHatchTimer)>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, mut egg_timer) in &mut eggs {
        egg_timer.timer.tick(time.delta());

        if egg_timer.timer.finished() {
            commands.spawn(ChickenBundle::child(&asset_server, transform));
            commands.entity(entity).despawn();
        }
    }
}

fn grow_chickens(
    mut chickens: Query<(&mut Handle<Image>, &mut ChickenData)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
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
