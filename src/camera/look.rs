use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::camera::CameraController;
use crate::interactions::components::OpenInspection;

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.03;

pub fn player_look(
    open: Res<OpenInspection>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
) {
    if open.is_open() {
        return;
    }

    let Ok((mut camera_transform, mut controller)) = camera_query.single_mut() else {
        return;
    };

    for event in mouse_motion.read() {
        controller.yaw -= event.delta.x * controller.sensitivity;
        controller.pitch -= event.delta.y * controller.sensitivity;
        controller.pitch = controller.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    camera_transform.rotation =
        Quat::from_rotation_y(controller.yaw) * Quat::from_rotation_x(controller.pitch);
}
