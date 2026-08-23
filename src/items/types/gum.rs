use serde::{Deserialize, Serialize};

use crate::items::item_def::ItemDef;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum GumTypes {
    MintStrongGum,
    LightGum,
}

impl GumTypes {
    pub fn def(self) -> ItemDef {
        let (name, description, dizziness_delta) = match self {
            GumTypes::MintStrongGum => (
                "Mint Strong Gum",
                "Cuts dizziness more than light gum.",
                -22.0,
            ),
            GumTypes::LightGum => ("Light Gum", "Takes the edge off.", -10.0),
        };

        ItemDef::item(name, description, dizziness_delta, false)
    }

    pub fn model(self) -> &'static str {
        match self {
            GumTypes::MintStrongGum => "models/items/gum/mint_strong_gum.glb",
            GumTypes::LightGum => "models/items/gum/light_gum.glb",
        }
    }
}
