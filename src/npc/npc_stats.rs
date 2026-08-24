use bevy::ecs::component::Component;

#[derive(Component, Clone)]
pub struct NpcStats {
    pub name: String,
    pub attack: f32,
    pub sp_attack: f32,
    pub dizziness: f32,
    pub dizziness_limit: f32,
}

impl NpcStats {
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attack: 0.0,
            sp_attack: 0.0,
            dizziness: 0.0,
            dizziness_limit: 100.0,
        }
    }
}
