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
pub struct OpenInspection {
    open: bool,
    target: Option<Entity>,
}

impl OpenInspection {
    pub fn entity(&self) -> Option<Entity> {
        self.target
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn open(&mut self, entity: Entity) {
        self.open = true;
        self.target = Some(entity);
    }

    pub fn open_bag(&mut self) {
        self.open = true;
        self.target = None;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.target = None;
    }
}

#[derive(Component)]
pub struct InspectionPage;

#[derive(Component)]
pub struct InspectionTitle;
