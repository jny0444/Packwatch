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

    pub fn model(self) -> &'static str {
        match self {
            ItemKind::Cig(kind) => kind.model(),
            ItemKind::Beer(kind) => kind.model(),
            ItemKind::Gum(kind) => kind.model(),
            ItemKind::Lighter => "models/items/lighter/lighter.glb",
        }
    }

    pub fn kind_model(self) -> &'static str {
        match self {
            ItemKind::Cig(_) => "models/items/cigs/cig.glb",
            ItemKind::Beer(_) => "models/items/beer/beer.glb",
            ItemKind::Gum(_) => "models/items/gum/gum.glb",
            ItemKind::Lighter => "models/items/lighter/lighter.glb",
        }
    }

    pub fn resolved_model(self) -> &'static str {
        let path = self.model();
        if std::path::Path::new("assets").join(path).is_file() {
            path
        } else {
            self.kind_model()
        }
    }
}
