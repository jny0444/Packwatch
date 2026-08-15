pub mod components;
pub mod focus;
pub mod use_item;

use bevy::prelude::*;

use crate::interactions::{
    components::FocusedInteractable, focus::update_focus, use_item::use_item,
};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FocusedInteractable>()
            .add_systems(Update, (update_focus, use_item));
    }
}
