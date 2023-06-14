use bevy::input::Input;
use bevy::prelude::{Camera, Camera2d, KeyCode, Query, Res, Transform, With};
use bevy::time::Time;

pub fn move_with_keyboard(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut transform: Query<&mut Transform, With<Camera2d>>,
) {
    if input.any_pressed([KeyCode::Left, KeyCode::A]) {
        let mut transform = transform.single_mut();

        transform.translation.x -= 100. * time.delta_seconds();
    }

    if input.any_pressed([KeyCode::Right, KeyCode::D]) {
        let mut transform = transform.single_mut();

        transform.translation.x += 100. * time.delta_seconds();
    }

    if input.any_pressed([KeyCode::Up, KeyCode::W]) {
        let mut transform = transform.single_mut();

        transform.translation.y += 100. * time.delta_seconds();
    }

    if input.any_pressed([KeyCode::Down, KeyCode::S]) {
        let mut transform = transform.single_mut();

        transform.translation.y -= 100. * time.delta_seconds();
    }

    if input.pressed(KeyCode::N) {
        let mut transform = transform.single_mut();

        transform.scale.x *= 1.1;
        transform.scale.y *= 1.1;
    }

    if input.pressed(KeyCode::M) {
        let mut transform = transform.single_mut();

        transform.scale.x *= 0.9;
        transform.scale.y *= 0.9;
    }
}
