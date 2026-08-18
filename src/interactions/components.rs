use bevy::prelude::*;

#[derive(Component)]
pub struct Interactable;

#[derive(Resource, Default)]
pub struct FocusedInteractable(Option<Entity>);

impl FocusedInteractable {
    pub fn entity(&self) -> Option<Entity> {
        self.0
    }

    pub fn set(&mut self, entity: Option<Entity>) {
        self.0 = entity;
    }
}

#[derive(Component)]
pub struct InteractPrompt;

#[derive(Component)]
pub struct InspectInfo {
    pub title: String,
}

#[derive(Resource, Default)]
pub struct OpenInspection(Option<Entity>);

impl OpenInspection {
    pub fn entity(&self) -> Option<Entity> {
        self.0
    }

    pub fn is_open(&self) -> bool {
        self.0.is_some()
    }

    pub fn open(&mut self, entity: Entity) {
        self.0 = Some(entity);
    }

    pub fn close(&mut self) {
        self.0 = None;
    }
}

#[derive(Component)]
pub struct InspectionPage;

#[derive(Component)]
pub struct InspectionTitle;
