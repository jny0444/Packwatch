use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    interactions::components::{InspectInfo, Interactable},
    npc::{Npc, NpcKind},
    screens::GameState,
};

#[derive(Clone)]
pub struct CharacterTemplate {
    pub kind: NpcKind,
    pub position: Vec3,
    pub radius: f32,
    pub height: f32,
    pub color: Color,
    pub interactable: bool,
}

impl CharacterTemplate {
    pub fn new(kind: NpcKind, position: Vec3) -> Self {
        Self {
            kind,
            position,
            radius: 0.4,
            height: 0.9,
            color: Color::srgb(0.6, 0.6, 0.7),
            interactable: true,
        }
    }

    pub fn with_size(mut self, radius: f32, height: f32) -> Self {
        self.radius = radius;
        self.height = height;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn not_interactable(mut self) -> Self {
        self.interactable = false;
        self
    }
}

pub fn spawn_character(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    template: CharacterTemplate,
) {
    let stats = template.kind.stats();
    let mut character = commands.spawn((
        Npc,
        DespawnOnExit(GameState::Playing),
        template.kind,
        stats,
        Transform::from_translation(template.position),
        Mesh3d(meshes.add(Capsule3d::new(template.radius, template.height))),
        MeshMaterial3d(materials.add(template.color)),
        RigidBody::Static,
        Collider::capsule(template.radius, template.height),
    ));

    if template.interactable {
        character.insert((
            Interactable,
            InspectInfo {
                title: template.kind.display_name().into(),
            },
        ));
    }
}
