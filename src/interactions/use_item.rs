use bevy::{prelude::*, window::CursorOptions};

use crate::camera::set_cursor_locked;
use crate::interactions::components::{
    FocusedInteractable, InspectInfo, InspectionPage, InspectionTitle, OpenInspection,
};

pub fn use_item(
    keyboard: Res<ButtonInput<KeyCode>>,
    focused: Res<FocusedInteractable>,
    inspect_info: Query<&InspectInfo>,
    mut open: ResMut<OpenInspection>,
    mut page: Query<&mut Visibility, With<InspectionPage>>,
    mut title: Query<&mut Text, With<InspectionTitle>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if open.is_open() {
        return;
    }

    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Some(entity) = focused.entity() else {
        return;
    };

    let Ok(info) = inspect_info.get(entity) else {
        return;
    };

    open.open(entity);

    if let Ok(mut visibility) = page.single_mut() {
        *visibility = Visibility::Visible;
    }
    if let Ok(mut text) = title.single_mut() {
        **text = info.title.clone();
    }

    set_cursor_locked(&mut cursor_options, false);
}
