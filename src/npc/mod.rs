pub mod npc_kind;
pub mod npc_stats;

pub use npc_kind::NpcKind;
pub use npc_stats::NpcStats;

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Npc;
