use bevy::prelude::*;

#[derive(Component)]
pub struct Interactable;

#[derive(Resource, Default)]
pub struct FocusedInteractable(pub Option<Entity>);

#[derive(Component)]
pub struct InteractPrompt;

#[derive(Component)]
pub struct InspectInfo {
    pub title: String,
}

#[derive(Resource, Default)]
pub struct OpenInspection(pub Option<Entity>);

#[derive(Component)]
pub struct InspectionPage;

#[derive(Component)]
pub struct InspectionTitle;
