use avian3d::prelude::*;
use bevy::prelude::*;

use crate::interactions::components::OpenInspection;
use crate::player::Player;

const MOVE_SPEED: f32 = 4.0;

pub fn player_move(
    open: Res<OpenInspection>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut LinearVelocity), With<Player>>,
) {
    if open.0.is_some() {
        if let Ok((_, mut velocity)) = player_query.single_mut() {
            velocity.x = 0.0;
            velocity.z = 0.0;
        }
        return;
    }

    let Ok((transform, mut velocity)) = player_query.single_mut() else {
        return;
    };

    let forward = transform.forward().as_vec3();
    let right = transform.right().as_vec3();
    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

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

    let direction = direction.normalize_or_zero() * MOVE_SPEED;
    velocity.x = direction.x;
    velocity.z = direction.z;
}
