mod loading;
mod start;

pub use loading::GameAssets;

use bevy::prelude::*;

use loading::LoadingPlugin;
use start::StartPlugin;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Start,
    Playing,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.03)))
            .add_plugins((LoadingPlugin, StartPlugin));
    }
}
