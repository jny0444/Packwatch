pub mod npc_kind;
pub mod npc_stats;
pub mod spawn;

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Npc;
