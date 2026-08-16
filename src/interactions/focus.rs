use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    camera::CameraController,
    interactions::components::{FocusedInteractable, Interactable, OpenInspection},
    player::Player,
};

const INTERACT_RANGE: f32 = 3.0;

pub fn update_focus(
    open: Res<OpenInspection>,
    spatial_query: SpatialQuery,
    camera_query: Query<&GlobalTransform, With<CameraController>>,
    player_query: Query<Entity, With<Player>>,
    interactables: Query<(), With<Interactable>>,
    mut focused: ResMut<FocusedInteractable>,
) {
    if open.0.is_some() {
        return;
    }

    focused.0 = None;

    let Ok(camera) = camera_query.single() else {
        return;
    };
    let Ok(player) = player_query.single() else {
        return;
    };

    let transform = camera.compute_transform();
    let filter = SpatialQueryFilter::from_excluded_entities([player]);

    let Some(hit) = spatial_query.cast_ray(
        transform.translation,
        transform.forward(),
        INTERACT_RANGE,
        true,
        &filter,
    ) else {
        return;
    };

    if interactables.contains(hit.entity) {
        focused.0 = Some(hit.entity);
    }
}
