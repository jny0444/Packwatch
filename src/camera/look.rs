use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::camera::CameraController;
use crate::interactions::components::OpenInspection;

pub fn player_look(
    open: Res<OpenInspection>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
) {
    if open.0.is_some() {
        return;
    }

    let Ok((mut camera_transform, mut controller)) = camera_query.single_mut() else {
        return;
    };

    for event in mouse_motion.read() {
        controller.yaw -= event.delta.x * controller.sensitivity;
        controller.pitch -= event.delta.y * controller.sensitivity;
        controller.pitch = controller.pitch.clamp(-1.54, 1.54);
    }

    camera_transform.rotation =
        Quat::from_rotation_y(controller.yaw) * Quat::from_rotation_x(controller.pitch);
}
