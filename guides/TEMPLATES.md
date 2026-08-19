# Character and model templates

Use these templates when you want to add something new to the world without rewriting the full Bevy spawn tuple.

## Add a character / NPC

1. Add the character kind in `src/npc/npc_kind.rs`:

```rust
pub enum NpcKind {
    LightSmoker,
    HeavySmoker,
    ShopKeeper,
    Guide,
    NewCharacter,
}
```

2. Add a `display_name()` arm and stats in `NpcKind`. Every kind needs its own arm — do not use `_ =>`.

```rust
NpcKind::NewCharacter => NpcStats {
    name: "New Character".into(),
    speed: 1.0,
    sp_speed: 1.0,
    attack: 1.0,
    sp_attack: 1.0,
    defence: 1.0,
    sp_defence: 1.0,
    capacity: 1.0,
    dizziness: 0.0,
},
```

3. Spawn it in `src/scene/setup.rs`:

```rust
use crate::npc::NpcKind;
use crate::templates::{CharacterTemplate, spawn_character};

spawn_character(
    &mut commands,
    &mut meshes,
    &mut materials,
    CharacterTemplate::new(NpcKind::NewCharacter, Vec3::new(-2.0, 0.85, -2.0))
        .with_color(Color::srgb(0.2, 0.4, 0.9)),
);
```

The character is interactable by default. If you do not want the player to inspect it, add `.not_interactable()`.

## Add a simple model / object

Spawn a cuboid model in `src/scene/setup.rs`:

```rust
use crate::templates::{ModelTemplate, spawn_model};

spawn_model(
    &mut commands,
    &mut meshes,
    &mut materials,
    ModelTemplate::cuboid(
        "Supply Box",
        Vec3::new(1.5, 0.25, -2.5),
        Vec3::new(0.8, 0.5, 0.8),
    )
    .with_color(Color::srgb(0.7, 0.5, 0.2)),
);
```

Models are interactable by default. If it should only be decoration, add `.not_interactable()`.

## Add an imported `.glb` / `.gltf` model

Put the model under `assets/models/` (characters in `assets/models/characters/<name>/`, rooms in `assets/scenes/`). Example: `assets/models/supply_box.glb`.

Add `asset_server` to the `setup` system parameters in `src/scene/setup.rs`:

```rust
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
```

Then spawn the model:

```rust
use crate::templates::{SceneModelTemplate, spawn_scene_model};

spawn_scene_model(
    &mut commands,
    &asset_server,
    SceneModelTemplate::gltf(
        "Supply Box",
        "models/supply_box.glb",
        Vec3::new(1.5, 0.0, -2.5),
    )
    .with_scale(Vec3::splat(1.0)),
);
```

The template uses scene `0` from the GLTF file by default. Change it with `.with_scene_index(1)` if your file has multiple scenes.

## Files involved

- `src/templates/character.rs` — reusable character/NPC spawn template
- `src/templates/model.rs` — reusable cuboid model/object spawn template
- `src/npc/npc_kind.rs` — NPC names and stats
- `src/scene/setup.rs` — where the starting level is spawned
