use crate::items::Pocket;
use crate::items::item_def::ItemDef;
use crate::items::types::{BeerTypes, CigTypes, GumTypes};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
            ItemKind::Lighter => ItemDef {
                name: "Lighter",
                description: "No flame, no smoke.",
                pocket: Pocket::KeyItems,
                max_stack: 1,
                tossable: false,
                needs_lighter: false,
                dizziness_delta: 0.0,
            },
        }
    }
}
