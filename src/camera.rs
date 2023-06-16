use bevy::app::{App, Plugin};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::input::Input;
use bevy::prelude::{Camera2d, EventReader, MouseButton, Query, Res, Transform, With};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(zoom).add_system(move_with_mouse);
    }
}

pub fn zoom(
    mut inputs: EventReader<MouseWheel>,
    mut camera_transform: Query<&mut Transform, With<Camera2d>>,
) {
    let mut camera_transform = camera_transform.single_mut();

    //todo offset on scroll

    for mouse_wheel in inputs.iter() {
        if mouse_wheel.y < 0. {
            camera_transform.scale.x *= 1.1;
            camera_transform.scale.y *= 1.1;
        } else {
            camera_transform.scale.x *= 0.9;
            camera_transform.scale.y *= 0.9;
        }
    }
}

pub fn move_with_mouse(
    mut inputs: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    mut camera_transform: Query<&mut Transform, With<Camera2d>>,
) {
    let mut camera_transform = camera_transform.single_mut();

    for mouse_motion in inputs.iter() {
        if buttons.any_pressed([MouseButton::Middle, MouseButton::Left]) {
            camera_transform.translation.x -= mouse_motion.delta.x * camera_transform.scale.x;
            camera_transform.translation.y += mouse_motion.delta.y * camera_transform.scale.y;
        }
    }
}
