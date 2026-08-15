mod controller;
mod cursor;
mod look;

pub use controller::CameraController;

use bevy::prelude::*;

use cursor::grab_cursor;
use look::player_look;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_look, grab_cursor));
    }
}
