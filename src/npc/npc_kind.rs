use bevy::ecs::component::Component;

use crate::npc::npc_stats::NpcStats;

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

    pub fn stats(self) -> NpcStats {
        match self {
            NpcKind::Guide => NpcStats {
                name: self.display_name().into(),
                attack: 15.0,
                sp_attack: 20.0,
                dizziness: 0.0,
                dizziness_limit: 80.0,
            },
            NpcKind::LightSmoker | NpcKind::HeavySmoker | NpcKind::ShopKeeper => {
                NpcStats::named(self.display_name())
            }
        }
    }
}
