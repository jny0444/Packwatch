use bevy::ecs::component::Component;

use crate::{
    items::{
        Deck, ItemKind,
        types::{BeerTypes, CigTypes, GumTypes},
    },
    npc::npc_stats::NpcStats,
};

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NpcKind {
    LightSmoker,
    HeavySmoker,
    ShopKeeper,
    Guide,
}

impl NpcKind {
    pub fn display_name(self) -> &'static str {
        match self {
            NpcKind::LightSmoker => "Light Smoker",
            NpcKind::HeavySmoker => "Heavy Smoker",
            NpcKind::ShopKeeper => "Shopkeeper",
            NpcKind::Guide => "Guide",
        }
    }

    pub fn is_fighter(self) -> bool {
        !matches!(self, NpcKind::ShopKeeper)
    }

    pub fn stake(self) -> u32 {
        match self {
            NpcKind::Guide => 25,
            NpcKind::LightSmoker => 50,
            NpcKind::HeavySmoker => 100,
            NpcKind::ShopKeeper => 0,
        }
    }

    pub fn fighter_model(self) -> (&'static str, f32) {
        match self {
            NpcKind::Guide => ("models/characters/guide/guide.glb", 0.55),
            NpcKind::LightSmoker => ("models/characters/felipe/felipe.glb", 0.55),
            NpcKind::HeavySmoker | NpcKind::ShopKeeper => {
                ("models/characters/nechaev/nechaev.gltf", 1.0)
            }
        }
    }

    pub fn stats(self) -> NpcStats {
        match self {
            NpcKind::Guide => NpcStats {
                name: self.display_name().into(),
                attack: 8.0,
                sp_attack: 12.0,
                dizziness: 0.0,
                dizziness_limit: 80.0,
            },
            NpcKind::LightSmoker => NpcStats {
                name: self.display_name().into(),
                attack: 12.0,
                sp_attack: 16.0,
                dizziness: 0.0,
                dizziness_limit: 100.0,
            },
            NpcKind::HeavySmoker => NpcStats {
                name: self.display_name().into(),
                attack: 18.0,
                sp_attack: 24.0,
                dizziness: 0.0,
                dizziness_limit: 100.0,
            },
            NpcKind::ShopKeeper => NpcStats::named(self.display_name()),
        }
    }

    pub fn deck(self) -> Deck {
        match self {
            NpcKind::Guide => Deck::from_cards(&[
                ItemKind::Cig(CigTypes::MarlboroGold),
                ItemKind::Cig(CigTypes::MarlboroCompact),
                ItemKind::Cig(CigTypes::ClassicIndieMint),
                ItemKind::Cig(CigTypes::CamelBlue),
                ItemKind::Cig(CigTypes::ClassicConnect),
                ItemKind::Gum(GumTypes::LightGum),
            ]),
            NpcKind::LightSmoker => Deck::from_cards(&[
                ItemKind::Cig(CigTypes::MarlboroGold),
                ItemKind::Cig(CigTypes::DoubleHappiness6mg),
                ItemKind::Cig(CigTypes::CamelBlue),
                ItemKind::Cig(CigTypes::MarlboroAdvance),
                ItemKind::Cig(CigTypes::ClassicConnect),
                ItemKind::Gum(GumTypes::LightGum),
            ]),
            NpcKind::HeavySmoker => Deck::from_cards(&[
                ItemKind::Cig(CigTypes::MarlboroRed),
                ItemKind::Cig(CigTypes::CamelYellow),
                ItemKind::Cig(CigTypes::Bidi),
                ItemKind::Gum(GumTypes::MintStrongGum),
                ItemKind::Beer(BeerTypes::KingfisherStrong),
            ]),
            NpcKind::ShopKeeper => Deck::new(),
        }
    }
}
