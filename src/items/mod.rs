use bevy::prelude::*;

pub mod inventory;
pub mod item_def;
pub mod item_kind;
pub mod pocket;
pub mod save;
pub mod types;

pub use inventory::Inventory;
pub use item_def::ItemDef;
pub use item_kind::ItemKind;
pub use pocket::Pocket;

use crate::screens::GameState;

use save::{load_inventory, save_inventory};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_systems(OnEnter(GameState::Playing), load_into_world)
            .add_systems(OnExit(GameState::Playing), save_from_world);
    }
}

fn load_into_world(mut inventory: ResMut<Inventory>) {
    *inventory = load_inventory();
}

fn save_from_world(inventory: Res<Inventory>) {
    save_inventory(&inventory);
}
