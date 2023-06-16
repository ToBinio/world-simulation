use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::HALF_MAP_SIZE;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_entity)
            .add_system(select_new_random_target)
            .add_system(flip_sprite);
    }
}

#[derive(Component, Default)]
pub struct Target {
    pub target: Vec2,
}

fn move_entity(mut entities: Query<(&mut Transform, &Target)>, time: Res<Time>) {
    for (mut transform, direction) in &mut entities {
        let translate = &mut transform.translation;

        let angle = (direction.target.y - translate.y).atan2(direction.target.x - translate.x);

        translate.x += angle.cos() * 100.0 * time.delta_seconds();
        translate.y += angle.sin() * 100.0 * time.delta_seconds();
    }
}

fn select_new_random_target(mut entities: Query<(&Transform, &mut Target)>) {
    let mut rng = thread_rng();

    for (transform, mut target) in &mut entities {
        let distance = target
            .target
            .distance(Vec2::new(transform.translation.x, transform.translation.y));

        if distance < 5. {
            target.target = Vec2::new(rng.gen_range(-HALF_MAP_SIZE..HALF_MAP_SIZE), rng.gen_range(-HALF_MAP_SIZE..HALF_MAP_SIZE));
        }
    }
}

fn flip_sprite(mut entities: Query<(&mut Transform, &Target)>) {
    for (mut transform, target) in &mut entities {
        if target.target.x > transform.translation.x && transform.scale.x < 0. {
            transform.scale.x = -transform.scale.x;
        }

        if target.target.x < transform.translation.x && transform.scale.x > 0. {
            transform.scale.x = -transform.scale.x;
        }
    }
}
