pub mod camera;
pub mod interactions;
pub mod player;
pub mod scene;

use bevy::prelude::*;

use camera::CameraPlugin;
use interactions::InteractionPlugin;
use player::PlayerPlugin;
use scene::ScenePlugin;

pub struct PackwatchPlugin;

impl Plugin for PackwatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScenePlugin, CameraPlugin, PlayerPlugin, InteractionPlugin));
    }
}
