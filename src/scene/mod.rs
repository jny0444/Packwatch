mod setup;

use bevy::{prelude::*, world_serialization::WorldInstanceReady};

use setup::setup;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalAmbientLight {
            brightness: 250.0,
            ..default()
        })
        .add_observer(fix_imported_materials)
        .add_systems(Startup, setup);
    }
}

fn fix_imported_materials(
    ready: On<WorldInstanceReady>,
    children: Query<&Children>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in children.iter_descendants(ready.entity) {
        let Ok(handle) = mesh_materials.get(entity) else {
            continue;
        };
        let Some(mut material) = materials.get_mut(handle.id()) else {
            continue;
        };

        // Sketchfab PNGs include unused transparent UV padding, so the
        // exporter sets Blend. That disables depth writes and makes the
        // character see-through.
        if matches!(material.alpha_mode, AlphaMode::Blend) {
            material.alpha_mode = AlphaMode::Mask(0.5);
        }
        material.double_sided = false;
    }
}
