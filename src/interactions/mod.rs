pub mod components;
pub mod focus;
pub mod page;
pub mod prompt;
pub mod use_item;

use bevy::prelude::*;

use crate::{
    interactions::{
        components::{FocusedInteractable, OpenInspection},
        focus::update_focus,
        page::{close_page, spawn_page},
        prompt::{spawn_prompt, update_prompt},
        use_item::use_item,
    },
    screens::{GameState, PlayMode},
};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FocusedInteractable>()
            .init_resource::<OpenInspection>()
            .add_systems(OnEnter(GameState::Playing), (spawn_prompt, spawn_page))
            .add_systems(
                Update,
                (update_focus, update_prompt, use_item, close_page)
                    .chain()
                    .run_if(in_state(PlayMode::Exploring)),
            );
    }
}
