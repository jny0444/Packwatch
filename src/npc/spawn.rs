use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    interactions::components::{InspectInfo, Interactable},
    npc::{Npc, npc_kind::NpcKind},
};

pub fn spawn_npc(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    kind: NpcKind,
    position: Vec3,
) {
    let stats = kind.stats();
    commands.spawn((
        Npc,
        kind,
        stats,
        Transform::from_translation(position),
        Mesh3d(meshes.add(Capsule3d::new(0.4, 0.9))),
        MeshMaterial3d(materials.add(Color::srgb(0.6, 0.6, 0.7))),
        RigidBody::Static,
        Collider::capsule(0.4, 0.9),
        Interactable,
        InspectInfo {
            title: kind.stats().name,
        },
    ));
}
