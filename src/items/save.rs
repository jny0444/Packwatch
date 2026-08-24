use std::{fs, path::Path};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::items::{Deck, Inventory, wallet::Wallet};

const SAVE_VERSION: u32 = 1;
const SAVE_PATH: &str = "saves/slot1.json";

#[derive(Serialize, Deserialize)]
struct SaveFile {
    version: u32,
    inventory: Inventory,
    #[serde(default)]
    amount: u32,
    #[serde(default)]
    deck: Deck,
}

pub struct LoadedSave {
    pub inventory: Inventory,
    pub wallet: Wallet,
    pub deck: Deck,
}

pub fn save_inventory(inventory: &Inventory, wallet: &Wallet, deck: &Deck) {
    let save = SaveFile {
        version: SAVE_VERSION,
        inventory: inventory.clone(),
        amount: wallet.balance,
        deck: deck.clone(),
    };
    let Ok(json) = serde_json::to_string_pretty(&save) else {
        error!("failed to serialize save");
        return;
    };
    if let Err(err) = fs::create_dir_all("saves") {
        error!("failed to create saves directory: {err}");
        return;
    }
    if let Err(err) = fs::write(SAVE_PATH, json) {
        error!("failed to write {SAVE_PATH}: {err}");
    }
}

pub fn load_save() -> LoadedSave {
    let empty = || LoadedSave {
        inventory: Inventory::new(),
        wallet: Wallet::new(),
        deck: Deck::new(),
    };

    let Ok(bytes) = fs::read(Path::new(SAVE_PATH)) else {
        return empty();
    };

    match serde_json::from_slice::<SaveFile>(&bytes) {
        Ok(save) if save.version == SAVE_VERSION => LoadedSave {
            inventory: save.inventory,
            wallet: Wallet {
                balance: save.amount,
            },
            deck: save.deck,
        },
        _ => empty(),
    }
}
