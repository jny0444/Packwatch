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

impl NpcStats {
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            speed: 0.0,
            sp_speed: 0.0,
            attack: 0.0,
            sp_attack: 0.0,
            defence: 0.0,
            sp_defence: 0.0,
            capacity: 0.0,
            dizziness: 0.0,
        }
    }
}
