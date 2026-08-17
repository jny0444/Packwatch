use bevy::prelude::*;

use crate::{
    npc::npc_kind::NpcKind,
    templates::{spawn_character, CharacterTemplate},
};

pub fn spawn_npc(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    kind: NpcKind,
    position: Vec3,
) {
    spawn_character(
        commands,
        meshes,
        materials,
        CharacterTemplate::new(kind, position),
    );
}
