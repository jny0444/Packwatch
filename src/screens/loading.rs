use bevy::prelude::*;

use crate::screens::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_loading)
            .add_systems(Update, finish_loading.run_if(in_state(GameState::Loading)));
    }
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pending: Vec<UntypedHandle>,
}

impl GameAssets {
    pub fn track<A: Asset>(&mut self, handle: Handle<A>) -> Handle<A> {
        self.pending.push(handle.clone().untyped());
        handle
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        self.pending.iter().all(|handle| {
            match asset_server.get_recursive_dependency_load_state(handle) {
                Some(state) => state.is_loaded() || state.is_failed(),
                None => false,
            }
        })
    }
}

/// Queue every asset that must finish before the start screen.
/// Add new `assets.track(asset_server.load(...))` lines here.
fn queue_assets(asset_server: &AssetServer, assets: &mut GameAssets) {
    assets.track(asset_server.load::<WorldAsset>(
        GltfAssetLabel::Scene(0).from_asset("models/characters/nechaev/nechaev.gltf"),
    ));
    assets.track(asset_server.load::<WorldAsset>(
        GltfAssetLabel::Scene(0).from_asset("models/characters/guide/guide.glb"),
    ));
    assets.track(asset_server.load::<WorldAsset>(
        GltfAssetLabel::Scene(0).from_asset("models/items/cigs/cig.glb"),
    ));
    assets.track(asset_server.load::<WorldAsset>(
        GltfAssetLabel::Scene(0).from_asset("models/items/beer/beer.glb"),
    ));
    assets.track(asset_server.load::<WorldAsset>(
        GltfAssetLabel::Scene(0).from_asset("models/items/gum/gum.glb"),
    ));
    assets.track(asset_server.load::<WorldAsset>(
        GltfAssetLabel::Scene(0).from_asset("models/items/lighter/lighter.glb"),
    ));
}

fn spawn_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut assets = GameAssets::default();
    queue_assets(&asset_server, &mut assets);
    commands.insert_resource(assets);

    commands.spawn((
        DespawnOnExit(GameState::Loading),
        GlobalZIndex(1000),
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(16),
            ..default()
        },
        BackgroundColor(Color::srgb(0.04, 0.04, 0.05)),
        children![
            (
                Text::new("PACKWATCH"),
                TextFont {
                    font_size: FontSize::Px(56.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Text::new("Loading..."),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.65, 0.65, 0.68)),
            ),
        ],
    ));
}

fn finish_loading(
    assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if assets.is_ready(&asset_server) {
        next_state.set(GameState::Start);
    }
}
