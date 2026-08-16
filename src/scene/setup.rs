use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::camera::CameraController;
use crate::interactions::components::{InspectInfo, Interactable};
use crate::player::Player;

pub fn walls(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 2.5, -5.0).with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
        RigidBody::Static,
        Collider::cuboid(10.0, 0.1, 5.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 2.5, 5.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        RigidBody::Static,
        Collider::cuboid(10.0, 0.1, 5.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(5.0, 2.5, 0.0).with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        RigidBody::Static,
        Collider::cuboid(5.0, 0.1, 10.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(-5.0, 2.5, 0.0).with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
        RigidBody::Static,
        Collider::cuboid(5.0, 0.1, 10.0),
    ));
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Generates a plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(10.0, 0.1, 10.0),
    ));

    walls(&mut commands, &mut meshes, &mut materials);

    // Generates the light source
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawns the cube for testing
    // REMOVE LATER
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.3, 0.3))),
        Transform::from_xyz(0.0, 0.5, -3.0),
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        Interactable,
        InspectInfo {
            title: "Test Cube".into(),
        },
    ));

    // Spawns the player
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 0.85, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.4, 0.9),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Multiply),
        TranslationInterpolation,
        children![(
            Camera3d::default(),
            CameraController::default(),
            Transform::from_xyz(0.0, 0.85, 0.0),
        )],
    ));
}
