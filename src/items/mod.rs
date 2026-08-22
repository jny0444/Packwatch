use bevy::prelude::*;

pub mod hud;
pub mod inventory;
pub mod item_def;
pub mod item_kind;
pub mod pocket;
pub mod save;
pub mod shop;
pub mod types;
pub mod wallet;

pub use inventory::{Inventory, Stack};
pub use item_def::ItemDef;
pub use item_kind::ItemKind;
pub use pocket::Pocket;

use crate::{
    items::{
        hud::{spawn_wallet_hud, update_wallet_hud},
        save::{load_save, save_inventory},
        wallet::Wallet,
    },
    screens::GameState,
};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .init_resource::<Wallet>()
            .add_systems(
                Update,
                update_wallet_hud.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnEnter(GameState::Playing),
                (load_into_world, spawn_wallet_hud),
            )
            .add_systems(OnExit(GameState::Playing), save_from_world)
            .add_systems(Last, save_on_quit);
    }
}

fn load_into_world(mut inventory: ResMut<Inventory>, mut wallet: ResMut<Wallet>) {
    let loaded = load_save();
    *inventory = loaded.inventory;
    *wallet = loaded.wallet;
}

fn save_from_world(inventory: Res<Inventory>, wallet: Res<Wallet>) {
    save_inventory(&inventory, &wallet);
}

fn save_on_quit(
    exit: MessageReader<AppExit>,
    state: Res<State<GameState>>,
    inventory: Res<Inventory>,
    wallet: Res<Wallet>,
) {
    if exit.is_empty() || *state.get() != GameState::Playing {
        return;
    }
    save_inventory(&inventory, &wallet);
}
