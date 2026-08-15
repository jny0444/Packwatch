use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::camera::CameraController;
use crate::player::Player;

pub fn player_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut CameraController), Without<Player>>,
) {
    let Ok(mut player_transform) = player_query.single_mut() else {
        return;
    };
    let Ok((mut camera_transform, mut controller)) = camera_query.single_mut() else {
        return;
    };

    for event in mouse_motion.read() {
        controller.yaw -= event.delta.x * controller.sensitivity;
        controller.pitch -= event.delta.y * controller.sensitivity;
        controller.pitch = controller.pitch.clamp(-1.54, 1.54);
    }

    player_transform.rotation = Quat::from_rotation_y(controller.yaw);
    camera_transform.rotation = Quat::from_rotation_x(controller.pitch);
}
