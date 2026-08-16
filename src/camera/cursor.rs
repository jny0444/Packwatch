use bevy::{prelude::*, window::CursorOptions};

use crate::interactions::components::OpenInspection;

pub fn lock_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
    cursor_options.visible = false;
}

pub fn grab_cursor(
    open: Res<OpenInspection>,
    mut cursor_options: Single<&mut CursorOptions>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if open.0.is_some() {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        cursor_options.visible = false;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
        cursor_options.visible = true;
    }
}
