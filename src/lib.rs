pub mod camera;
pub mod combat;
pub mod interactions;
pub mod items;
pub mod npc;
pub mod player;
pub mod scene;
pub mod screens;
pub mod templates;

use bevy::prelude::*;

use camera::CameraPlugin;
use combat::CombatPlugin;
use interactions::InteractionPlugin;
use player::PlayerPlugin;
use scene::ScenePlugin;
use screens::ScreensPlugin;

use crate::items::ItemsPlugin;

pub struct PackwatchPlugin;

impl Plugin for PackwatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreensPlugin,
            ScenePlugin,
            CameraPlugin,
            PlayerPlugin,
            InteractionPlugin,
            ItemsPlugin,
            CombatPlugin,
        ));
    }
}
