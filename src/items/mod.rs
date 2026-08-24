use bevy::{prelude::*, window::WindowCloseRequested};

pub mod bag;
pub mod hud;
pub mod inventory;
pub mod item_def;
pub mod item_kind;
pub mod pocket;
pub mod save;
pub mod shop;
pub mod types;
pub mod wallet;

pub use bag::InventoryPage;
pub use inventory::{Inventory, Stack};
pub use item_def::{ItemDef, ItemStats};
pub use item_kind::ItemKind;
pub use pocket::Pocket;
pub use shop::ShopPage;

use crate::{
    items::{
        bag::{BagUi, bag_interact, spawn_bag_page, update_bag_visuals},
        hud::{spawn_wallet_hud, update_wallet_hud},
        save::{load_save, save_inventory},
        shop::{
            ShopUi, SpendFlash, animate_purchase, drag_preview, rotate_preview, shop_interact,
            spawn_shop_page, sync_preview_layers, sync_shop_preview, update_shop_visuals,
        },
        wallet::Wallet,
    },
    screens::{GameState, PlayMode},
};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .init_resource::<Wallet>()
            .init_resource::<SpendFlash>()
            .init_resource::<ShopUi>()
            .init_resource::<BagUi>()
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    load_into_world,
                    spawn_wallet_hud,
                    spawn_shop_page,
                    spawn_bag_page,
                ),
            )
            .add_systems(
                Update,
                (
                    update_wallet_hud,
                    update_bag_visuals,
                    update_shop_visuals,
                    sync_shop_preview,
                    drag_preview,
                    rotate_preview,
                    sync_preview_layers,
                    animate_purchase,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (shop_interact, bag_interact)
                    .chain()
                    .run_if(in_state(PlayMode::Exploring)),
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
    close: MessageReader<WindowCloseRequested>,
    exit: MessageReader<AppExit>,
    state: Res<State<GameState>>,
    inventory: Res<Inventory>,
    wallet: Res<Wallet>,
) {
    if close.is_empty() && exit.is_empty() {
        return;
    }
    if *state.get() != GameState::Playing {
        return;
    }
    save_inventory(&inventory, &wallet);
}
