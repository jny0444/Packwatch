mod controller;
mod crosshair;
mod cursor;
mod look;

pub use controller::CameraController;
pub use crosshair::Crosshair;
pub use cursor::set_cursor_locked;

use bevy::prelude::*;

use crate::screens::{GameState, PlayMode};

use crosshair::{spawn_crosshair, update_crosshair};
use cursor::{grab_cursor, lock_cursor};
use look::player_look;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_crosshair, lock_cursor))
            .add_systems(
                Update,
                (player_look, grab_cursor, update_crosshair).run_if(in_state(PlayMode::Exploring)),
            );
    }
}
