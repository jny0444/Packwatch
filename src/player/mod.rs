mod components;
mod movement;

pub use components::Player;

use bevy::prelude::*;

use crate::screens::GameState;

use movement::player_move;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            player_move.run_if(in_state(GameState::Playing)),
        );
    }
}
