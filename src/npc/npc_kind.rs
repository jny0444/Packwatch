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
    pub fn stats(self) -> NpcStats {
        match self {
            NpcKind::Guide => NpcStats {
                name: "Guide".into(),
                speed: 0.0,
                sp_speed: 100.0,
                attack: 100.0,
                sp_attack: 100.0,
                defence: 100.0,
                sp_defence: 100.0,
                capacity: 100.0,
                dizziness: 0.0,
            },
            _ => NpcStats {
                name: "something".into(),
                speed: 0.0,
                sp_speed: 0.0,
                attack: 0.0,
                sp_attack: 0.0,
                defence: 0.0,
                sp_defence: 0.0,
                capacity: 0.0,
                dizziness: 0.0,
            },
        }
    }
}
