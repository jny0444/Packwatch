use bevy::{prelude::*, window::CursorOptions};

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
    if open.0.is_some() {
        return;
    }

    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Some(entity) = focused.0 else {
        return;
    };

    let Ok(info) = inspect_info.get(entity) else {
        return;
    };

    open.0 = Some(entity);

    if let Ok(mut visibility) = page.single_mut() {
        *visibility = Visibility::Visible;
    }
    if let Ok(mut text) = title.single_mut() {
        **text = info.title.clone();
    }

    cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
    cursor_options.visible = true;
}
