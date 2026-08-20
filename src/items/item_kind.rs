use serde::{Deserialize, Serialize};

use crate::items::item_def::ItemDef;
use crate::items::types::{BeerTypes, CigTypes, GumTypes};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ItemKind {
    Cig(CigTypes),
    Beer(BeerTypes),
    Gum(GumTypes),
    Lighter,
}

impl ItemKind {
    pub fn def(self) -> ItemDef {
        match self {
            ItemKind::Cig(kind) => kind.def(),
            ItemKind::Beer(kind) => kind.def(),
            ItemKind::Gum(kind) => kind.def(),
            ItemKind::Lighter => ItemDef::key_item("Lighter", "No flame, no smoke."),
        }
    }
}
