mod components;
mod movement;
mod spawn;

pub use components::{Player, PlayerStats};
pub use spawn::spawn_player;

use bevy::prelude::*;

use crate::screens::PlayMode;

use movement::{MoveWish, player_move, read_move_input};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MoveWish>()
            .add_systems(
                Update,
                read_move_input.run_if(in_state(PlayMode::Exploring)),
            )
            .add_systems(
                FixedUpdate,
                player_move.run_if(in_state(PlayMode::Exploring)),
            );
    }
}
