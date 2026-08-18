use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::interactions::components::OpenInspection;

pub fn set_cursor_locked(cursor: &mut CursorOptions, locked: bool) {
    if locked {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    } else {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}

pub fn lock_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    set_cursor_locked(&mut cursor_options, true);
}

pub fn grab_cursor(
    open: Res<OpenInspection>,
    mut cursor_options: Single<&mut CursorOptions>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if open.is_open() {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        set_cursor_locked(&mut cursor_options, true);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        set_cursor_locked(&mut cursor_options, false);
    }
}
