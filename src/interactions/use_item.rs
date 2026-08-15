use bevy::prelude::*;

use crate::interactions::components::FocusedInteractable;

pub fn use_item(
    keyboard: Res<ButtonInput<KeyCode>>,
    focused: Res<FocusedInteractable>,
    mut commands: Commands,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Some(entity) = focused.0 else {
        return;
    };

    info!("used {entity}");
    commands.entity(entity).despawn();
}
