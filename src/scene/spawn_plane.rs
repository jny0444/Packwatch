use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::{CameraController, Player};

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
        Transform::from_xyz(3.0, 0.5, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));

    // Spawns the player
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 0.85, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.4, 0.9),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
        TransformInterpolation,
        children![(
            Camera3d::default(),
            CameraController::default(),
            Transform::from_xyz(0.0, 0.85, 0.0),
        )],
    ));
}
