use bevy::ecs::component::Component;

#[derive(Component, Clone)]
pub struct NpcStats {
    pub name: String,
    pub speed: f32,
    pub sp_speed: f32,
    pub attack: f32,
    pub sp_attack: f32,
    pub defence: f32,
    pub sp_defence: f32,
    pub capacity: f32,
    pub dizziness: f32,
}
