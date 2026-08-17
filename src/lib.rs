pub mod camera;
pub mod interactions;
pub mod npc;
pub mod player;
pub mod scene;
pub mod screens;

use bevy::prelude::*;

use camera::CameraPlugin;
use interactions::InteractionPlugin;
use player::PlayerPlugin;
use scene::ScenePlugin;
use screens::ScreensPlugin;

pub struct PackwatchPlugin;

impl Plugin for PackwatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreensPlugin,
            ScenePlugin,
            CameraPlugin,
            PlayerPlugin,
            InteractionPlugin,
        ));
    }
}
