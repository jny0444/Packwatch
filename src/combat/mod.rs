pub mod ai;
pub mod round;
pub mod staging;
pub mod state;
pub mod ui;

use bevy::prelude::*;

use crate::screens::PlayMode;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayMode::Match),
            (
                staging::hide_world_npcs,
                staging::hide_explore_camera,
                staging::hide_hud_bits,
                staging::spawn_arena,
                ui::spawn_match_ui,
                round::init_session,
            )
                .chain(),
        )
        .add_systems(
            OnExit(PlayMode::Match),
            (
                staging::show_world_npcs,
                staging::show_explore_camera,
                staging::show_hud_bits,
                round::clear_session,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                ui::update_match_ui,
                round::match_input,
                round::match_resolve,
                staging::idle_fighters,
            )
                .chain()
                .run_if(in_state(PlayMode::Match)),
        );
    }
}
