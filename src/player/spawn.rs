use avian3d::prelude::*;
use bevy::prelude::*;

use crate::camera::CameraController;
use crate::player::{Player, PlayerStats};
use crate::screens::GameState;

const CAPSULE_RADIUS: f32 = 0.4;
const CAPSULE_HEIGHT: f32 = 0.9;
const EYE_HEIGHT: f32 = 0.85;

pub fn spawn_player(commands: &mut Commands, position: Vec3) {
    commands.spawn((
        Player,
        PlayerStats {
            dizziness: 0.0,
            dizziness_limit: 100.0,
        },
        DespawnOnExit(GameState::Playing),
        Transform::from_translation(position),
        RigidBody::Dynamic,
        Collider::capsule(CAPSULE_RADIUS, CAPSULE_HEIGHT),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Multiply),
        TranslationInterpolation,
        children![(
            Camera3d::default(),
            CameraController::default(),
            Transform::from_xyz(0.0, EYE_HEIGHT, 0.0),
        )],
    ));
}
