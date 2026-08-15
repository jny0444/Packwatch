use bevy::prelude::*;

#[derive(Component)]
pub struct Interactable;

#[derive(Resource, Default)]
pub struct FocusedInteractable(pub Option<Entity>);
