use avian3d::prelude::*;
use bevy::prelude::*;

use crate::interactions::components::{InspectInfo, Interactable};
use crate::npc::{Npc, NpcKind};
use crate::screens::GameState;

/// Marker for imported scenes whose materials should drop unused PNG alpha.
#[derive(Component)]
pub struct FixGltfAlpha;

#[derive(Clone)]
pub struct ModelTemplate {
    pub title: String,
    pub position: Vec3,
    pub size: Vec3,
    pub color: Color,
    pub interactable: bool,
}

impl ModelTemplate {
    pub fn cuboid(title: impl Into<String>, position: Vec3, size: Vec3) -> Self {
        Self {
            title: title.into(),
            position,
            size,
            color: Color::srgb(0.8, 0.3, 0.3),
            interactable: true,
        }
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

#[derive(Clone)]
pub enum ScenePhysics {
    Mesh(ColliderConstructor),
    Capsule { radius: f32, height: f32 },
    None,
}

#[derive(Clone)]
pub struct SceneModelTemplate {
    pub title: String,
    pub asset_path: String,
    pub scene_index: usize,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub interactable: bool,
    pub physics: ScenePhysics,
    pub npc: Option<NpcKind>,
}

impl SceneModelTemplate {
    pub fn gltf(title: impl Into<String>, asset_path: impl Into<String>, position: Vec3) -> Self {
        Self {
            title: title.into(),
            asset_path: asset_path.into(),
            scene_index: 0,
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            interactable: true,
            physics: ScenePhysics::Mesh(ColliderConstructor::ConvexHullFromMesh),
            npc: None,
        }
    }

    pub fn with_scene_index(mut self, scene_index: usize) -> Self {
        self.scene_index = scene_index;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn not_interactable(mut self) -> Self {
        self.interactable = false;
        self
    }

    pub fn with_trimesh_collider(mut self) -> Self {
        self.physics = ScenePhysics::Mesh(ColliderConstructor::TrimeshFromMesh);
        self
    }

    pub fn with_capsule(mut self, radius: f32, height: f32) -> Self {
        self.physics = ScenePhysics::Capsule { radius, height };
        self
    }

    pub fn without_physics(mut self) -> Self {
        self.physics = ScenePhysics::None;
        self
    }

    pub fn as_npc(mut self, kind: NpcKind) -> Self {
        self.title = kind.display_name().into();
        self.npc = Some(kind);
        self
    }
}

pub fn spawn_model(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    template: ModelTemplate,
) {
    let mut model = commands.spawn((
        DespawnOnExit(GameState::Playing),
        Mesh3d(meshes.add(Cuboid::new(
            template.size.x,
            template.size.y,
            template.size.z,
        ))),
        MeshMaterial3d(materials.add(template.color)),
        Transform::from_translation(template.position),
        RigidBody::Static,
        Collider::cuboid(template.size.x, template.size.y, template.size.z),
    ));

    if template.interactable {
        model.insert((
            Interactable,
            InspectInfo {
                title: template.title,
            },
        ));
    }
}

pub fn spawn_scene_model(
    commands: &mut Commands,
    asset_server: &AssetServer,
    template: SceneModelTemplate,
) -> Entity {
    let mut model = commands.spawn((
        DespawnOnExit(GameState::Playing),
        FixGltfAlpha,
        WorldAssetRoot(
            asset_server
                .load(GltfAssetLabel::Scene(template.scene_index).from_asset(template.asset_path)),
        ),
        Transform {
            translation: template.position,
            rotation: template.rotation,
            scale: template.scale,
        },
    ));

    match template.physics {
        ScenePhysics::Mesh(constructor) => {
            model.insert((
                RigidBody::Static,
                ColliderConstructorHierarchy::new(constructor),
            ));
        }
        ScenePhysics::Capsule { radius, height } => {
            let scale = template.scale.y.abs().max(1e-4);
            model.with_children(|parent| {
                parent.spawn((
                    RigidBody::Static,
                    Collider::capsule(radius / scale, height / scale),
                    Transform::from_xyz(0.0, (height * 0.5 + radius) / scale, 0.0),
                ));
            });
        }
        ScenePhysics::None => {}
    }

    if let Some(kind) = template.npc {
        model.insert((Npc, kind, kind.stats()));
    }

    if template.interactable {
        model.insert((
            Interactable,
            InspectInfo {
                title: template.title,
            },
        ));
    }

    model.id()
}
