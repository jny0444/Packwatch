use avian3d::prelude::*;
use bevy::prelude::*;

use crate::camera::CameraController;
use crate::interactions::components::OpenInspection;
use crate::player::Player;

const MOVE_SPEED: f32 = 4.0;

#[derive(Resource, Default)]
pub(crate) struct MoveWish(Vec3);

pub fn read_move_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&CameraController>,
    mut wish: ResMut<MoveWish>,
) {
    wish.0 = Vec3::ZERO;

    let Ok(controller) = camera_query.single() else {
        return;
    };

    let rotation = Quat::from_rotation_y(controller.yaw);
    let forward = (rotation * Vec3::NEG_Z).normalize_or_zero();
    let right = (rotation * Vec3::X).normalize_or_zero();

    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += right;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= right;
    }

    wish.0 = direction.normalize_or_zero();
}

pub fn player_move(
    open: Res<OpenInspection>,
    wish: Res<MoveWish>,
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
) {
    let Ok(mut velocity) = player_query.single_mut() else {
        return;
    };

    if open.is_open() {
        velocity.x = 0.0;
        velocity.z = 0.0;
        return;
    }

    let direction = wish.0 * MOVE_SPEED;
    velocity.x = direction.x;
    velocity.z = direction.z;
}
