mod components;
mod movement;

pub use components::Player;

use bevy::prelude::*;

use movement::player_move;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_move);
    }
}
