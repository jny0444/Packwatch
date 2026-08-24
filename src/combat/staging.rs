use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    camera::{CameraController, Crosshair},
    combat::state::Side,
    interactions::components::InteractPrompt,
    npc::{Npc, NpcKind},
    screens::{ActiveMatch, PlayMode},
    templates::{SceneModelTemplate, spawn_scene_model},
};

pub const ARENA: Vec3 = Vec3::new(0.0, -80.0, 0.0);
const PLAYER_MODEL: &str = "models/characters/felipe/felipe.glb";
const PLAYER_SCALE: f32 = 0.55;

#[derive(Component)]
pub struct MatchProp;

#[derive(Component)]
pub struct BattleCamera;

#[derive(Component)]
pub struct MatchFighter {
    pub side: Side,
}

pub fn hide_world_npcs(mut npcs: Query<&mut Visibility, With<Npc>>) {
    for mut visibility in &mut npcs {
        *visibility = Visibility::Hidden;
    }
}

pub fn show_world_npcs(mut npcs: Query<&mut Visibility, With<Npc>>) {
    for mut visibility in &mut npcs {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_explore_camera(mut cameras: Query<&mut Camera, With<CameraController>>) {
    for mut camera in &mut cameras {
        camera.is_active = false;
    }
}

pub fn show_explore_camera(mut cameras: Query<&mut Camera, With<CameraController>>) {
    for mut camera in &mut cameras {
        camera.is_active = true;
    }
}

pub fn hide_hud_bits(
    mut crosshair: Query<&mut Visibility, With<Crosshair>>,
    mut prompt: Query<&mut Visibility, (With<InteractPrompt>, Without<Crosshair>)>,
) {
    if let Ok(mut visibility) = crosshair.single_mut() {
        *visibility = Visibility::Hidden;
    }
    if let Ok(mut visibility) = prompt.single_mut() {
        *visibility = Visibility::Hidden;
    }
}

pub fn show_hud_bits(mut crosshair: Query<&mut Visibility, With<Crosshair>>) {
    if let Ok(mut visibility) = crosshair.single_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn spawn_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    active: Res<ActiveMatch>,
    kinds: Query<&NpcKind>,
) {
    let enemy_kind = active
        .opponent()
        .and_then(|entity| kinds.get(entity).ok().copied())
        .unwrap_or(NpcKind::Guide);
    let (enemy_path, enemy_scale) = enemy_kind.fighter_model();

    let player_pos = ARENA + Vec3::new(-1.55, 0.0, 1.55);
    let enemy_pos = ARENA + Vec3::new(1.85, 0.0, -2.15);
    let look_at = ARENA + Vec3::new(0.15, 0.95, -0.35);

    commands.spawn((
        MatchProp,
        BattleCamera,
        DespawnOnExit(PlayMode::Match),
        Camera3d::default(),
        Camera {
            order: 2,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.52, 0.78, 0.92)),
            ..default()
        },
        Transform::from_translation(ARENA + Vec3::new(0.35, 1.85, 5.4)).looking_at(look_at, Vec3::Y),
    ));

    commands.spawn((
        MatchProp,
        DespawnOnExit(PlayMode::Match),
        DirectionalLight {
            illuminance: 14_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_translation(ARENA + Vec3::new(4.0, 8.0, 5.0)).looking_at(ARENA, Vec3::Y),
    ));

    let grass = MeshMaterial3d(materials.add(Color::srgb(0.38, 0.62, 0.32)));
    let dirt = MeshMaterial3d(materials.add(Color::srgb(0.55, 0.48, 0.28)));
    commands.spawn((
        MatchProp,
        DespawnOnExit(PlayMode::Match),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(18.0, 18.0))),
        grass,
        Transform::from_translation(ARENA),
    ));
    commands.spawn((
        MatchProp,
        DespawnOnExit(PlayMode::Match),
        Mesh3d(meshes.add(Cuboid::new(7.5, 0.08, 7.5))),
        dirt,
        Transform::from_translation(ARENA + Vec3::new(0.0, 0.04, 0.0)),
    ));

    let player = spawn_scene_model(
        &mut commands,
        &asset_server,
        SceneModelTemplate::gltf("You", PLAYER_MODEL, player_pos)
            .with_scale(Vec3::splat(PLAYER_SCALE))
            .with_rotation(Quat::from_rotation_y(-0.55))
            .not_interactable()
            .without_physics(),
    );
    commands.entity(player).insert((
        MatchProp,
        MatchFighter { side: Side::Player },
        DespawnOnExit(PlayMode::Match),
    ));

    let enemy = spawn_scene_model(
        &mut commands,
        &asset_server,
        SceneModelTemplate::gltf(enemy_kind.display_name(), enemy_path, enemy_pos)
            .with_scale(Vec3::splat(enemy_scale))
            .with_rotation(Quat::from_rotation_y(PI - 0.35))
            .not_interactable()
            .without_physics(),
    );
    commands.entity(enemy).insert((
        MatchProp,
        MatchFighter { side: Side::Enemy },
        DespawnOnExit(PlayMode::Match),
    ));
}

pub fn idle_fighters(time: Res<Time>, mut fighters: Query<(&MatchFighter, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (fighter, mut transform) in &mut fighters {
        let bob = (t * 1.6
            + match fighter.side {
                Side::Player => 0.0,
                Side::Enemy => 1.1,
            })
        .sin()
            * 0.03;
        transform.translation.y = ARENA.y + bob;
    }
}
