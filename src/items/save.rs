use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::items::Inventory;

const SAVE_VERSION: u32 = 1;
const SAVE_PATH: &str = "saves/slot1.json";

#[derive(Serialize, Deserialize)]
struct SaveFile {
    version: u32,
    inventory: Inventory,
}

pub fn load_inventory() -> Inventory {
    let Ok(bytes) = fs::read(Path::new(SAVE_PATH)) else {
        return Inventory::new();
    };

    match serde_json::from_slice::<SaveFile>(&bytes) {
        Ok(save) if save.version == SAVE_VERSION => save.inventory,
        _ => Inventory::new(),
    }
}

pub fn save_inventory(inventory: &Inventory) {
    let save = SaveFile {
        version: SAVE_VERSION,
        inventory: inventory.clone(),
    };
    let Ok(json) = serde_json::to_string_pretty(&save) else {
        return;
    };
    let _ = fs::create_dir_all("saves");
    let _ = fs::write(SAVE_PATH, json);
}
