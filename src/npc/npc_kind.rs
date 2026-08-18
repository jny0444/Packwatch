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
                speed: 0.0,
                sp_speed: 100.0,
                attack: 100.0,
                sp_attack: 100.0,
                defence: 100.0,
                sp_defence: 100.0,
                capacity: 100.0,
                dizziness: 0.0,
            },
            NpcKind::LightSmoker | NpcKind::HeavySmoker | NpcKind::ShopKeeper => {
                NpcStats::named(self.display_name())
            }
        }
    }
}
