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

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Playing)]
pub enum PlayMode {
    #[default]
    Exploring,
    Dialogue,
    Match,
}

#[derive(Resource, Default)]
pub struct ActiveMatch(Option<Entity>);

impl ActiveMatch {
    pub fn opponent(&self) -> Option<Entity> {
        self.0
    }

    pub fn start(&mut self, opponent: Entity) {
        self.0 = Some(opponent);
    }

    pub fn clear(&mut self) {
        self.0 = None;
    }
}

#[derive(Component)]
struct MenuCamera;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<PlayMode>()
            .init_resource::<ActiveMatch>()
            .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.03)))
            .add_systems(OnEnter(GameState::Loading), spawn_menu_camera)
            .add_systems(OnEnter(GameState::Start), spawn_menu_camera)
            .add_systems(OnExit(GameState::Start), despawn_menu_camera)
            .add_plugins((LoadingPlugin, StartPlugin));
    }
}

fn spawn_menu_camera(mut commands: Commands, existing: Query<(), With<MenuCamera>>) {
    if !existing.is_empty() {
        return;
    }

    commands.spawn((Camera2d, MenuCamera));
}

fn despawn_menu_camera(mut commands: Commands, cameras: Query<Entity, With<MenuCamera>>) {
    for entity in &cameras {
        commands.entity(entity).despawn();
    }
}
