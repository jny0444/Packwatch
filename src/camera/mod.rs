use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorOptions};

use crate::components::{CameraController, Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_look, grab_cursor));
    }
}

fn player_look(
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

fn grab_cursor(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        cursor_options.visible = false;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
        cursor_options.visible = true;
    }
}
