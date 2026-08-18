use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::npc::NpcKind;
use crate::player::spawn_player;
use crate::screens::GameState;
use crate::templates::{CharacterTemplate, SceneModelTemplate, spawn_character, spawn_scene_model};

const ROOM_COLOR: Color = Color::srgb(0.3, 0.5, 0.3);
const FLOOR_COLLIDER_THICKNESS: f32 = 0.1;

fn walls(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    wall_material: &MeshMaterial3d<StandardMaterial>,
) {
    let plane_10x5 = Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 5.0)));
    let plane_5x10 = Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 10.0)));

    let walls = [
        (
            Vec3::new(0.0, 2.5, -5.0),
            Quat::from_rotation_x(FRAC_PI_2),
            plane_10x5.clone(),
            Vec3::new(10.0, 0.1, 5.0),
        ),
        (
            Vec3::new(0.0, 2.5, 5.0),
            Quat::from_rotation_x(-FRAC_PI_2),
            plane_10x5,
            Vec3::new(10.0, 0.1, 5.0),
        ),
        (
            Vec3::new(5.0, 2.5, 0.0),
            Quat::from_rotation_z(FRAC_PI_2),
            plane_5x10.clone(),
            Vec3::new(5.0, 0.1, 10.0),
        ),
        (
            Vec3::new(-5.0, 2.5, 0.0),
            Quat::from_rotation_z(-FRAC_PI_2),
            plane_5x10,
            Vec3::new(5.0, 0.1, 10.0),
        ),
    ];

    for (translation, rotation, mesh, collider) in walls {
        commands.spawn((
            DespawnOnExit(GameState::Playing),
            mesh,
            wall_material.clone(),
            Transform::from_translation(translation).with_rotation(rotation),
            RigidBody::Static,
            Collider::cuboid(collider.x, collider.y, collider.z),
        ));
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let room_material = MeshMaterial3d(materials.add(ROOM_COLOR));

    commands.spawn((
        DespawnOnExit(GameState::Playing),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        room_material.clone(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(10.0, FLOOR_COLLIDER_THICKNESS, 10.0),
    ));

    walls(&mut commands, &mut meshes, &room_material);

    commands.spawn((
        DespawnOnExit(GameState::Playing),
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    spawn_scene_model(
        &mut commands,
        &asset_server,
        SceneModelTemplate::gltf("Test Character", "char.gltf", Vec3::new(0.0, 0.0, -3.0)),
    );

    spawn_character(
        &mut commands,
        &mut meshes,
        &mut materials,
        CharacterTemplate::new(NpcKind::Guide, Vec3::new(2.0, 0.85, -2.0)),
    );

    spawn_player(&mut commands, Vec3::new(0.0, 0.85, 0.0));
}
