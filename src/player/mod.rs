use bevy::prelude::*;

use crate::components::Player;

const MOVE_SPEED: f32 = 4.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_move);
    }
}

fn player_move(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
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

    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * MOVE_SPEED * time.delta_secs();
    }
}
