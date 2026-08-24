use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerStats {
    pub dizziness: f32,
    pub dizziness_limit: f32,
}
