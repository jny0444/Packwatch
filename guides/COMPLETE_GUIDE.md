# Packwatch: The Master Developer & Rust Guide

Welcome to the definitive guide for **Packwatch** — a 3D first-person RPG and turn-based smoke deck-builder built in Rust with the [Bevy Engine](https://bevyengine.org/) (`0.19`) and [Avian 3D Physics](https://github.com/Jondolf/avian) (`0.7`).

This guide is written from the ground up to be **100% beginner-friendly**. Whether you are new to Rust, new to game development, or new to this codebase, this document contains **everything** you need to understand, modify, extend, and debug Packwatch without relying on an AI agent.

---

# Table of Contents
1. [Quick Start & Developer Workflow](#1-quick-start--developer-workflow)
2. [Chapter I: Rust Syntax & Core Concepts for Beginners](#chapter-i-rust-syntax--core-concepts-for-beginners)
   - [Variables, Types & Mutability](#variables-types--mutability)
   - [Functions & Control Flow](#functions--control-flow)
   - [Structs, Tuples & Methods](#structs-tuples--methods)
   - [Enums & Pattern Matching](#enums--pattern-matching)
   - [Option and Result: Safe Error Handling](#option-and-result-safe-error-handling)
   - [Ownership, References & Borrowing](#ownership-references--borrowing)
   - [Traits, Generics & Derives](#traits-generics--derives)
   - [Closures & Iterators](#closures--iterators)
   - [Modules, Crates & Imports](#modules-crates--imports)
3. [Chapter II: The Bevy Game Engine & ECS Architecture](#chapter-ii-the-bevy-game-engine--ecs-architecture)
   - [What is ECS? (Entities, Components, Systems)](#what-is-ecs-entities-components-systems)
   - [Entities and Components](#entities-and-components)
   - [Systems and System Parameters](#systems-and-system-parameters)
   - [Queries & Query Filters](#queries--query-filters)
   - [Commands & Entity Spawning](#commands--entity-spawning)
   - [Resources (Global State)](#resources-global-state)
   - [Schedules, States & Run Criteria](#schedules-states--run-criteria)
   - [3D Math, Transforms & Rotations](#3d-math-transforms--rotations)
   - [Bevy UI System & Offscreen Viewports](#bevy-ui-system--offscreen-viewports)
   - [Physics with Avian 3D](#physics-with-avian-3d)
4. [Chapter III: Complete Codebase Walkthrough](#chapter-iii-complete-codebase-walkthrough)
   - [Folder Map & Architecture Overview](#folder-map--architecture-overview)
   - [Root: `main.rs` & `lib.rs`](#root-mainrs--librs)
   - [Screens & State Machine (`src/screens/`)](#screens--state-machine-srcscreens)
   - [Camera & Viewport (`src/camera/`)](#camera--viewport-srccamera)
   - [Player & Movement (`src/player/`)](#player--movement-srcplayer)
   - [Scene & World Geometry (`src/scene/`)](#scene--world-geometry-srcscene)
   - [Templates & Asset Spawning (`src/templates/`)](#templates--asset-spawning-srctemplates)
   - [Interactions & Overlays (`src/interactions/`)](#interactions--overlays-srcinteractions)
   - [NPCs & AI Profiles (`src/npc/`)](#npcs--ai-profiles-srcnpc)
   - [Items, Inventory, Wallet & Shop (`src/items/`)](#items-inventory-wallet--shop-srcitems)
   - [Combat System & Match Arena (`src/combat/`)](#combat-system--match-arena-srccombat)
5. [Chapter IV: Step-by-Step Practical Recipes](#chapter-iv-step-by-step-practical-recipes)
   - [Recipe 1: Adding a New Item (Cigarette, Beer, or Gum)](#recipe-1-adding-a-new-item-cigarette-beer-or-gum)
   - [Recipe 2: Adding a New NPC with Custom Model & Dialogue](#recipe-2-adding-a-new-npc-with-custom-model--dialogue)
   - [Recipe 3: Creating a New UI Modal / Page](#recipe-3-creating-a-new-ui-modal--page)
   - [Recipe 4: Adding New Scene Props and Colliders](#recipe-4-adding-new-scene-props-and-colliders)
6. [Chapter V: Debugging & Common Compiler Errors](#chapter-v-debugging--common-compiler-errors)

---

# 1. Quick Start & Developer Workflow

### Prerequisites
- Install Rust via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Key Terminal Commands
Always run these commands from the root directory (`/Users/user56/Projects/packwatch`):

| Command | What it does | When to use |
|---|---|---|
| `cargo check` | Checks if your code compiles without building binary (super fast!) | While coding, to verify types & syntax |
| `cargo run` | Builds and launches the game in debug mode | To play and test changes |
| `cargo test` | Runs unit and integration tests | To verify logic correctness |
| `cargo clippy` | Linter that suggests idiomatic Rust improvements | Before committing code |

---

# Chapter I: Rust Syntax & Core Concepts for Beginners

Rust is a modern systems programming language that gives you C++ performance while guaranteeing **memory safety** and **thread safety** without needing a garbage collector.

---

## Variables, Types & Mutability

### Immutable by Default
In Rust, variables cannot be changed once declared unless you explicitly mark them with `mut`:
```rust
let speed = 5.0; // Immutable: speed cannot be modified
// speed = 10.0; // ERROR! Compiler prevents this

let mut health = 100; // Mutable
health -= 25; // OK! health is now 75
```

### Primitive Types
- **Integers**:
  - `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (Unsigned: positive only). `u32` is commonly used for prices, quantities, AP. `usize` is used for array/vector indexing.
  - `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (Signed: positive and negative).
- **Floats**: `f32`, `f64` (32-bit and 64-bit floating point numbers. Bevy uses `f32` everywhere for coordinates, velocities, and timers).
- **Booleans**: `bool` (`true` or `false`).
- **Strings**:
  - `&str`: String slice (borrowed view of text, e.g., `"Shopkeeper"`).
  - `String`: Owned, heap-allocated growable string (e.g., `String::from("Hello")` or `"Hello".to_string()`).

---

## Functions & Control Flow

### Defining Functions
```rust
// A function taking an f32 and returning a bool
fn is_in_range(distance: f32, max_range: f32) -> bool {
    // Note: No semicolon on the last line means it's the return value!
    distance <= max_range
}
```

### `if` Expressions
In Rust, `if` is an expression that can return a value:
```rust
let status_color = if is_alive { Color::srgb(0.0, 1.0, 0.0) } else { Color::srgb(1.0, 0.0, 0.0) };
```

### Loops
- `for item in &collection`: Iterate over items.
- `while condition`: Repeat while true.
- `loop`: Infinite loop (exit with `break`).

---

## Structs, Tuples & Methods

### 1. Named Structs (Data Containers)
```rust
pub struct ItemStats {
    pub attack: u32,
    pub ap_cost: u32,
    pub puffs: u32,
}
```

### 2. Tuple Structs
```rust
pub struct PlayerDeck(pub Deck);
pub struct ShopPreviewModel(pub ItemKind);
```

### 3. Unit Structs (Zero-Sized Markers)
In Bevy, unit structs are frequently used as **marker components** to tag entities:
```rust
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ShopPage;
```

### 4. Implementing Methods (`impl`)
```rust
impl ItemStats {
    // Associated constructor function
    pub fn new(attack: u32, ap_cost: u32, puffs: u32) -> Self {
        Self { attack, ap_cost, puffs }
    }

    // Method taking reference to self (&self)
    pub fn is_free(&self) -> bool {
        self.ap_cost == 0
    }
}
```

---

## Enums & Pattern Matching

Enums in Rust are algebraic data types — each variant can hold different types and amounts of data:

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Lighter,
    Cig(CigTypes),
    Beer(BeerTypes),
    Gum(GumTypes),
}
```

### Pattern Matching with `match`
`match` must be **exhaustive** (cover all possible variants):
```rust
match item_kind {
    ItemKind::Lighter => println!("It's a lighter!"),
    ItemKind::Cig(cig_type) => println!("Smoking cigarette: {:?}", cig_type),
    ItemKind::Beer(_) => println!("It's a beer!"),
    ItemKind::Gum(_) => println!("It's gum!"),
}
```

### `if let` (Match One Variant)
When you only care about one specific variant:
```rust
if let ItemKind::Cig(cig_type) = item_kind {
    println!("Selected cig: {:?}", cig_type);
}
```

---

## Option and Result: Safe Error Handling

Rust does not have `null` or `nil`. Instead, optional values use `Option<T>`:

```rust
pub enum Option<T> {
    Some(T),
    None,
}
```

And fallible operations use `Result<T, E>`:
```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Working with `Option` & `Result` Safely
```rust
// 1. Using if let
if let Some(target_entity) = focused.entity() {
    println!("Looking at entity: {:?}", target_entity);
}

// 2. Using unwrap_or
let current_qty = maybe_qty.unwrap_or(1);

// 3. Question mark operator (?) to early-return on Err or None
fn get_price(ui: &ShopUi) -> Option<u32> {
    let listing = ui.listing();
    Some(listing.price)
}
```

---

## Ownership, References & Borrowing

Rust enforces three rules at compile time:
1. Each value in Rust has an **owner**.
2. There can only be **one owner at a time**.
3. When the owner goes out of scope, the value is **dropped** (freed from memory).

### Borrowing with References
Instead of passing ownership, you pass references:
- **Immutable reference** (`&T`): Read-only access. You can have unlimited `&T` references at the same time.
- **Mutable reference** (`&mut T`): Read-and-write access. You can only have **one** `&mut T` reference to a piece of data at a time, and no active `&T` references.

```rust
fn print_title(info: &InspectInfo) { // Borrows immutably
    println!("{}", info.title);
}

fn add_money(wallet: &mut Wallet, amount: u32) { // Borrows mutably
    wallet.add(amount);
}
```

---

## Traits, Generics & Derives

A **trait** defines shared behavior (like an Interface in other languages):
```rust
pub trait Describable {
    fn description(&self) -> &str;
}
```

### Common Derives (`#[derive(...)]`)
- `#[derive(Component)]`: Marks a struct as a Bevy ECS Component.
- `#[derive(Resource)]`: Marks a struct as a Bevy global Resource.
- `#[derive(Clone, Copy)]`: Allows duplicating the value by simple bitwise copy (like numbers).
- `#[derive(Debug)]`: Enables printing with `println!("{:?}", value)`.
- `#[derive(Default)]`: Gives the type a default value (`Type::default()`).
- `#[derive(PartialEq, Eq)]`: Allows comparing with `==` and `!=`.

---

## Closures & Iterators

A **closure** is an anonymous function (lambda):
```rust
let add_five = |x: i32| x + 5;
```

### Common Iterator Methods
```rust
// Map & Sum:
let total_delta: Vec2 = motion.read().map(|m| m.delta).sum();

// Any (checks if any item matches):
let is_hovered = buttons.iter().any(|(_, interaction)| *interaction == Interaction::Hovered);

// Filter:
let active_cigs = cigs.iter().filter(|c| c.remaining_puffs > 0);
```

---

## Modules, Crates & Imports

- `src/lib.rs` / `src/main.rs`: The root of your crate.
- `mod foo;`: Declares that `foo.rs` or `foo/mod.rs` is part of this module tree.
- `pub mod foo;`: Declares and exports the module publicly.
- `use crate::items::ShopPage;`: Imports `ShopPage` starting from `src/`.
- `use super::*;`: Imports everything from the parent module.

---

# Chapter II: The Bevy Game Engine & ECS Architecture

Bevy is a data-driven game engine built around the **Entity-Component-System (ECS)** pattern.

---

## What is ECS? (Entities, Components, Systems)

Think of your game world as a high-performance spreadsheet:

| Concept | Plain English Analogy | Example in Packwatch |
|---|---|---|
| **Entity** | A row in the table (a unique integer ID) | `Entity(42)` (The Player, an NPC, a UI button) |
| **Component** | A column cell (data attached to a row) | `Transform`, `Player`, `Visibility`, `ItemKind` |
| **System** | A function that processes rows with specific columns | `player_move`, `rotate_preview`, `update_focus` |
| **Resource** | A single global object (outside the table) | `Wallet`, `Inventory`, `Time`, `AssetServer` |

---

## Entities and Components

To create an entity with components, use `commands.spawn(...)`:

```rust
commands.spawn((
    Player, // Marker component
    Transform::from_xyz(0.0, 1.0, 0.0), // Spatial position
    Visibility::Visible, // Visibility
    RigidBody::Dynamic, // Physics body
    Collider::capsule(0.4, 1.0), // Collision shape
));
```

---

## Systems and System Parameters

A **system** is any normal Rust function where every argument is a valid Bevy System Parameter:

```rust
fn my_system(
    time: Res<Time>,                                 // Global resource (read-only)
    mut wallet: ResMut<Wallet>,                      // Global resource (read/write)
    mut commands: Commands,                          // Entity command buffer
    query: Query<(&Transform, &Player)>,             // Entity query
) {
    let dt = time.delta_secs(); // Seconds elapsed since last frame
}
```

---

## Queries & Query Filters

Queries allow your system to find and iterate over all entities that match a set of component criteria.

### Query Syntax Examples
```rust
// 1. Read Transform for all entities that have Player component:
Query<&Transform, With<Player>>

// 2. Mutably edit Transform for all ShopPreviewStages:
Query<&mut Transform, With<ShopPreviewStage>>

// 3. Query multiple components and exclude another:
Query<(&mut Text, &ShopField), Without<ShopConfirm>>

// 4. Expecting exactly ONE entity (errors if 0 or >1):
let transform = player_query.single();
// Or safely:
if let Ok(transform) = player_query.single() { ... }

// 5. Query for entity ID alongside components:
Query<(Entity, &Interaction, &ShopRow)>

// 6. Changed filter (only triggers when component was modified this frame):
Query<&Interaction, Changed<Interaction>>
```

---

## Commands & Entity Spawning

`Commands` queues modifications to the ECS world to be applied safely between systems:

```rust
// Spawn an entity:
let entity_id = commands.spawn((MyComponent, Transform::default())).id();

// Insert a component onto an existing entity:
commands.entity(entity_id).insert(PurchasePulse { remaining: 0.5 });

// Remove a component from an entity:
commands.entity(entity_id).remove::<PurchasePulse>();

// Despawn an entity and all its child entities:
commands.entity(entity_id).despawn_recursive();
```

### Auto Cleanup with `DespawnOnExit`
Whenever an entity is spawned during gameplay, attach `DespawnOnExit(GameState::Playing)` so that returning to the main menu automatically cleans it up!

---

## Resources (Global State)

A **Resource** is state that exists only once across the entire application:

```rust
#[derive(Resource, Default)]
pub struct Wallet {
    pub money: u32,
}

// In plugin setup:
app.init_resource::<Wallet>();

// In systems:
fn earn_money(mut wallet: ResMut<Wallet>) {
    wallet.money += 50;
}
```

---

## Schedules, States & Run Criteria

### Game States (`GameState`)
Packwatch uses `GameState` to control what screen is active:
- `GameState::Loading`: Asset preloading screen.
- `GameState::StartMenu`: Start screen with "Press ENTER to Play".
- `GameState::Playing`: In-game 3D world.

### Play Modes (`PlayMode`)
When inside `GameState::Playing`, `PlayMode` controls the sub-state:
- `PlayMode::Exploring`: Free walking, mouse looking, inspecting items/shops.
- `PlayMode::Match`: Turn-based card/smoke combat arena.

### System Registration Examples
```rust
app
    // Runs ONCE when entering GameState::Playing:
    .add_systems(OnEnter(GameState::Playing), (setup_world, spawn_hud))
    
    // Runs EVERY FRAME while in GameState::Playing:
    .add_systems(
        Update,
        (update_hud, drag_preview, rotate_preview)
            .chain() // Runs in exact order
            .run_if(in_state(GameState::Playing)),
    )
    
    // Runs EVERY FRAME only while in Exploring mode:
    .add_systems(
        Update,
        (player_move, camera_look, use_item)
            .run_if(in_state(PlayMode::Exploring)),
    );
```

---

## 3D Math, Transforms & Rotations

In 3D space:
- **Position (`translation`)**: `Vec3::new(x, y, z)`
  - `+X` = Right, `-X` = Left
  - `+Y` = Up, `-Y` = Down
  - `+Z` = Backward (towards camera), `-Z` = Forward
- **Rotation**: Represented by Quaternions (`Quat`):
  - `Quat::IDENTITY`: No rotation.
  - `Quat::from_rotation_y(radians)`: Yaw (spinning horizontally).
  - `Quat::from_rotation_x(radians)`: Pitch (tilting up/down).
  - `Quat::from_rotation_z(radians)`: Roll (tilting side-to-side).
- **Scale**: `Vec3::splat(1.0)` (Uniform scale).

### Rotating Entities
```rust
// Rotate entity locally around Y axis by angle in radians:
transform.rotate_y(time.delta_secs() * 0.7);

// Rotate entity locally around X axis:
transform.rotate_x(mouse_delta_y * 0.008);
```

---

## Bevy UI System & Offscreen Viewports

Bevy UI uses a Flexbox layout engine powered by `Taffy`:

```rust
commands.spawn((
    Node {
        width: percent(100),
        height: px(50),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        padding: UiRect::all(px(12)),
        ..default()
    },
    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
));
```

### 3D Offscreen Viewports (Used in Shop & Bag Previews)
Packwatch renders 3D models directly inside 2D UI panels:
1. A separate `Camera3d` is spawned with `RenderTarget::Image(image_handle)` and `RenderLayers::layer(1)`.
2. The 3D preview model is placed far away at `Vec3::new(0.0, -200.0, 0.0)` on `RenderLayers::layer(1)`.
3. In the UI node hierarchy, an `ImageNode::new(image_handle)` renders that live texture onto the screen!

---

## Physics with Avian 3D

Packwatch uses **Avian 3D** (`avian3d`) for all collisions and movement:

- `RigidBody::Dynamic`: Affected by gravity and forces (e.g. Player).
- `RigidBody::Static`: Fixed obstacle that cannot move (e.g. Floors, Walls).
- `Collider::cuboid(x, y, z)`: Box collider.
- `Collider::capsule(radius, height)`: Capsule collider.
- `LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z`: Prevents player capsule from tipping over.
- `SpatialQuery::cast_ray`: Fires a 3D ray into the world to check what object the player is looking at.

---

# Chapter III: Complete Codebase Walkthrough

```
/Users/user56/Projects/packwatch/
├── Cargo.toml            # Project manifest & dependencies
├── assets/               # 3D models (.glb, .gltf), textures, audio
├── saves/save.json       # Player save data (Inventory, Wallet, Deck)
├── guides/               # Developer documentation & cheatsheets
└── src/
    ├── main.rs           # Engine bootstrap & entry point
    ├── lib.rs            # Core PackwatchPlugin definition
    ├── camera/           # First-person camera, mouse locking, crosshair
    ├── combat/           # Turn-based match & combat system
    ├── interactions/     # Raycasting, inspect page, 'E' interaction
    ├── items/            # Item database, shop, bag UI, wallet, deck builder
    ├── npc/              # NPC kinds, dialogues, stats, AI profiles
    ├── player/           # Player spawning, physics, WASD movement
    ├── scene/            # World spawn, walls, lighting, NPC placements
    ├── screens/          # Loading, start menu, game state managers
    └── templates/        # 3D model loaders, character animators, alpha fixes
```

---

## Root: `main.rs` & `lib.rs`

- **[`src/main.rs`](file:///Users/user56/Projects/packwatch/src/main.rs)**:
  Initializes `App::new()`, adds `DefaultPlugins` with window settings (Title: `"packwatch"`, resolution: 1280x720), adds `PhysicsPlugins::default()`, and registers `PackwatchPlugin`.
- **[`src/lib.rs`](file:///Users/user56/Projects/packwatch/src/lib.rs)**:
  Bundles all sub-plugins:
  - `ScreensPlugin`
  - `ScenePlugin`
  - `CameraPlugin`
  - `PlayerPlugin`
  - `InteractionPlugin`
  - `NpcPlugin`
  - `ItemsPlugin`
  - `CombatPlugin`

---

## Screens & State Machine (`src/screens/`)

Manages the top-level game state and exploration/combat mode:

- **[`src/screens/mod.rs`](file:///Users/user56/Projects/packwatch/src/screens/mod.rs)**:
  - `GameState`: Enum with `Loading`, `StartMenu`, `Playing`.
  - `PlayMode`: Sub-state with `Exploring`, `Match`.
  - `ActiveMatch`: Resource storing the target NPC entity when entering combat.
- **[`src/screens/loading.rs`](file:///Users/user56/Projects/packwatch/src/screens/loading.rs)**:
  Preloads fonts and assets before transitioning to `StartMenu`.
- **[`src/screens/start.rs`](file:///Users/user56/Projects/packwatch/src/screens/start.rs)**:
  Renders the start screen title and listens for `Enter` to transition to `GameState::Playing`.

---

## Camera & Viewport (`src/camera/`)

Implements first-person look controls and cursor management:

- **[`src/camera/controller.rs`](file:///Users/user56/Projects/packwatch/src/camera/controller.rs)**:
  `CameraController` component with `pitch` and `yaw`.
- **[`src/camera/look.rs`](file:///Users/user56/Projects/packwatch/src/camera/look.rs)**:
  Reads `MouseMotion`, updates `yaw` (spinning left/right) and `pitch` (clamped to [-89°, +89°]), and applies rotation to camera transform.
- **[`src/camera/cursor.rs`](file:///Users/user56/Projects/packwatch/src/camera/cursor.rs)**:
  `set_cursor_locked(options, true/false)` to lock and hide cursor during exploration or release cursor when opening UI.
- **[`src/camera/crosshair.rs`](file:///Users/user56/Projects/packwatch/src/camera/crosshair.rs)**:
  Spawns a subtle central dot in the middle of the screen.

---

## Player & Movement (`src/player/`)

Handles the player physical body and WASD movement:

- **[`src/player/components.rs`](file:///Users/user56/Projects/packwatch/src/player/components.rs)**:
  `Player` marker component, `PlayerStats` (HP, stamina, dizziness limit).
- **[`src/player/spawn.rs`](file:///Users/user56/Projects/packwatch/src/player/spawn.rs)**:
  Spawns the player capsule collider with `RigidBody::Dynamic`, `Friction::ZERO`, and locked rotation axes.
- **[`src/player/movement.rs`](file:///Users/user56/Projects/packwatch/src/player/movement.rs)**:
  Reads WASD keys, projects movement vector onto horizontal plane aligned with camera `yaw`, and updates `LinearVelocity.x` and `LinearVelocity.z`.

---

## Scene & World Geometry (`src/scene/`)

- **[`src/scene/setup.rs`](file:///Users/user56/Projects/packwatch/src/scene/setup.rs)**:
  - Spawns floor plane (10x10) with static cuboid collider.
  - Spawns 4 surrounding walls with colliders.
  - Spawns directional sun light.
  - Spawns world NPCs: `ShopKeeper`, `Guide`, `LightSmoker`, `HeavySmoker`.
  - Spawns the player entity.

---

## Templates & Asset Spawning (`src/templates/`)

Utilities for loading 3D assets smoothly:

- **[`src/templates/model.rs`](file:///Users/user56/Projects/packwatch/src/templates/model.rs)**:
  `SceneModelTemplate` builder for spawning GLTF/GLB models with custom scale, translation, rotation, and capsule colliders.
- **[`src/templates/character.rs`](file:///Users/user56/Projects/packwatch/src/templates/character.rs)**:
  Automatically detects `AnimationPlayer` inside GLTF hierarchies and plays idle animations on loop.

---

## Interactions & Overlays (`src/interactions/`)

- **[`src/interactions/components.rs`](file:///Users/user56/Projects/packwatch/src/interactions/components.rs)**:
  - `Interactable`: Tag placed on any object or NPC the player can interact with.
  - `InspectInfo`: Holds `{ title: String }`.
  - `FocusedInteractable`: Global resource storing the entity currently centered in crosshairs.
  - `OpenInspection`: Tracks which UI page is currently open.
- **[`src/interactions/focus.rs`](file:///Users/user56/Projects/packwatch/src/interactions/focus.rs)**:
  Casts a 3-meter ray from the camera every frame. If it hits an `Interactable` entity (or child of one), sets `FocusedInteractable`.
- **[`src/interactions/prompt.rs`](file:///Users/user56/Projects/packwatch/src/interactions/prompt.rs)**:
  Displays `[E] Talk` or `[E] Inspect` UI when an interactable object is focused.
- **[`src/interactions/use_item.rs`](file:///Users/user56/Projects/packwatch/src/interactions/use_item.rs)**:
  When `E` is pressed on the focused entity:
  - If NPC is `ShopKeeper` -> opens `ShopPage`.
  - If NPC is a fighter (`Guide`, `LightSmoker`, `HeavySmoker`) -> launches `PlayMode::Match` combat.
  - If entity has `InspectInfo` -> opens generic `InspectionPage`.
- **[`src/interactions/page.rs`](file:///Users/user56/Projects/packwatch/src/interactions/page.rs)**:
  Spawns inspect overlay and handles `Escape` key to close all open pages and re-lock cursor.

---

## NPCs & AI Profiles (`src/npc/`)

- **[`src/npc/npc_kind.rs`](file:///Users/user56/Projects/packwatch/src/npc/npc_kind.rs)**:
  Enum of NPC archetypes: `ShopKeeper`, `Guide`, `LightSmoker`, `HeavySmoker`. Defines whether they are fighters, their initial decks, and dialogue text.
- **[`src/npc/npc_stats.rs`](file:///Users/user56/Projects/packwatch/src/npc/npc_stats.rs)**:
  Stores NPC combat statistics (`dizziness_limit`).

---

## Items, Inventory, Wallet & Shop (`src/items/`)

- **[`src/items/item_kind.rs`](file:///Users/user56/Projects/packwatch/src/items/item_kind.rs)**:
  Enum of item variants: `Lighter`, `Cig(CigTypes)`, `Beer(BeerTypes)`, `Gum(GumTypes)`.
- **[`src/items/item_def.rs`](file:///Users/user56/Projects/packwatch/src/items/item_def.rs)**:
  Defines `ItemDef` (name, description, max stack size, 3D model asset path, stats).
- **[`src/items/types/`](file:///Users/user56/Projects/packwatch/src/items/types/)**:
  - `cig.rs`: All cigarette brands (Marlboro Gold, Double Happiness, Stellar Double Shift, Classic Indie Mint, Camel Yellow, Cashtri, Mond).
  - `beer.rs`: Beer brands (Budweiser Magnum, Kingfisher Strong, Corona, Guinness).
  - `gum.rs`: Gum types (Mint Strong Gum, Light Gum).
- **[`src/items/inventory.rs`](file:///Users/user56/Projects/packwatch/src/items/inventory.rs)**:
  Grid-based inventory storage with slots and stack counts.
- **[`src/items/wallet.rs`](file:///Users/user56/Projects/packwatch/src/items/wallet.rs)** & **[`hud.rs`](file:///Users/user56/Projects/packwatch/src/items/hud.rs)**:
  Player cash balance and top-right HUD counter.
- **[`src/items/deck.rs`](file:///Users/user56/Projects/packwatch/src/items/deck.rs)** & **[`deck_builder.rs`](file:///Users/user56/Projects/packwatch/src/items/deck_builder.rs)**:
  Player combat deck with AP cost calculations (max 10 AP, max 10 cards) and full deck customization UI.
- **[`src/items/bag.rs`](file:///Users/user56/Projects/packwatch/src/items/bag.rs)**:
  Inventory UI overlay toggled with `Tab`.
- **[`src/items/shop.rs`](file:///Users/user56/Projects/packwatch/src/items/shop.rs)**:
  Complete Kiosk Shop UI:
  - 3D interactive viewport with dedicated camera.
  - Continuous auto-spin animation (`rotate_preview`).
  - Interactive mouse dragging to inspect items from any angle (`drag_preview`).
  - Item catalog dropdown, price calculations, quantity buttons, and purchases.
- **[`src/items/save.rs`](file:///Users/user56/Projects/packwatch/src/items/save.rs)**:
  Saves and loads player inventory, money, and deck to/from `saves/save.json`.

---

## Combat System & Match Arena (`src/combat/`)

Turn-based smoke duel system:

- **[`src/combat/state.rs`](file:///Users/user56/Projects/packwatch/src/combat/state.rs)**:
  `MatchSession`, `MatchPhase` (`Plan`, `Puffs`, `EnemyTurn`, `Victory`, `Defeat`), active puffs, dizziness gauges.
- **[`src/combat/staging.rs`](file:///Users/user56/Projects/packwatch/src/combat/staging.rs)**:
  Hides world NPCs and spawns combat staged models facing the camera.
- **[`src/combat/round.rs`](file:///Users/user56/Projects/packwatch/src/combat/round.rs)**:
  Calculates damage per puff, status effects, AI turns, and win/loss conditions.
- **[`src/combat/ui.rs`](file:///Users/user56/Projects/packwatch/src/combat/ui.rs)**:
  Combat UI overlay (HP bars, deck cards, Puff button).
- **[`src/combat/ai.rs`](file:///Users/user56/Projects/packwatch/src/combat/ai.rs)**:
  Opponent AI decision logic.

---

# Chapter IV: Step-by-Step Practical Recipes

---

## Recipe 1: Adding a New Item (Cigarette, Beer, or Gum)

Let's add a new cigarette called **"Lucky Strike"**:

1. Open [`src/items/types/cig.rs`](file:///Users/user56/Projects/packwatch/src/items/types/cig.rs) and add the variant:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
   pub enum CigTypes {
       // ... existing items
       LuckyStrike,
   }
   ```
2. In the `impl CigTypes`, define its details in `def(&self)`:
   ```rust
   Self::LuckyStrike => ItemDef {
       name: "Lucky Strike",
       description: "Toasted tobacco. Strong hit, low dizziness.",
       max_stack: 20,
       pocket: Pocket::Cigs,
       model: "customASSets/lucky_strike.glb",
       stats: ItemStats::attack(35).with_ap(3).with_puffs(3),
   },
   ```
3. Add it to the shop stock in [`src/items/shop.rs`](file:///Users/user56/Projects/packwatch/src/items/shop.rs):
   ```rust
   pub const KIOSK_STOCK: &[ShopListing] = &[
       // ...
       listing(ItemKind::Cig(CigTypes::LuckyStrike), 32),
   ];
   ```
4. Run `cargo check` to verify. Done!

---

## Recipe 2: Adding a New NPC with Custom Model & Dialogue

1. Open [`src/npc/npc_kind.rs`](file:///Users/user56/Projects/packwatch/src/npc/npc_kind.rs) and add variant:
   ```rust
   pub enum NpcKind {
       // ...
       VeteranSmoker,
   }
   ```
2. Set display name and combat status:
   ```rust
   impl NpcKind {
       pub fn display_name(&self) -> &'static str {
           match self {
               // ...
               Self::VeteranSmoker => "Veteran Smoker",
           }
       }
       pub fn is_fighter(&self) -> bool {
           matches!(self, Self::Guide | Self::LightSmoker | Self::HeavySmoker | Self::VeteranSmoker)
       }
   }
   ```
3. Spawn the NPC in [`src/scene/setup.rs`](file:///Users/user56/Projects/packwatch/src/scene/setup.rs):
   ```rust
   spawn_scene_model(
       &mut commands,
       &asset_server,
       SceneModelTemplate::gltf(
           "Veteran Smoker",
           "models/characters/nechaev/nechaev.gltf",
           Vec3::new(-3.5, 0.0, 2.0),
       )
       .as_npc(NpcKind::VeteranSmoker)
       .with_capsule(0.5, 1.2),
   );
   ```

---

## Recipe 3: Creating a New UI Modal / Page

1. Define a marker component for your page:
   ```rust
   #[derive(Component)]
   pub struct QuestLogPage;
   ```
2. Spawn the page layout in an `OnEnter(GameState::Playing)` system:
   ```rust
   pub fn spawn_quest_page(mut commands: Commands) {
       commands.spawn((
           QuestLogPage,
           DespawnOnExit(GameState::Playing),
           Node {
               width: percent(100),
               height: percent(100),
               justify_content: JustifyContent::Center,
               align_items: AlignItems::Center,
               ..default()
           },
           Visibility::Hidden, // Start hidden!
       ))
       .with_children(|root| {
           root.spawn((
               Node { width: px(400), height: px(300), padding: UiRect::all(px(16)), ..default() },
               BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
           ))
           .with_children(|panel| {
               panel.spawn((Text::new("Quest Log"), TextColor(Color::WHITE)));
           });
       });
   }
   ```
3. Toggle its `Visibility` in your interaction or input system.

---

## Recipe 4: Adding New Scene Props and Colliders

In [`src/scene/setup.rs`](file:///Users/user56/Projects/packwatch/src/scene/setup.rs):
```rust
// Spawn a solid wooden crate:
commands.spawn((
    DespawnOnExit(GameState::Playing),
    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    MeshMaterial3d(materials.add(Color::srgb(0.5, 0.3, 0.1))),
    Transform::from_xyz(2.0, 0.5, -1.0),
    RigidBody::Static,
    Collider::cuboid(1.0, 1.0, 1.0),
    Interactable, // Makes it interactable!
    InspectInfo { title: "Old Wooden Crate".into() },
));
```

---

# Chapter V: Debugging & Common Compiler Errors

### 1. `QueryDataError::Aliasing` (Query Conflict)
**Error:** `Query<&mut Transform, ...> and Query<&Transform, ...> access the same entity.`
**Fix:** In Bevy, you cannot have two queries in the same system that both access the same component mutably, or one mutably and one immutably, without mutually exclusive filters like `Without<...>`.

### 2. `cannot borrow *self as mutable more than once`
**Error:** Trying to borrow `&mut inventory` while already borrowing from it.
**Fix:** Clone small data or extract IDs/numbers before mutating:
```rust
// BAD:
// let item = inventory.get(idx);
// inventory.remove(item.id);

// GOOD:
let item_id = inventory.get(idx).map(|i| i.id);
if let Some(id) = item_id {
    inventory.remove(id);
}
```

### 3. Warning `[B0004]: Entity with component has parent without component`
**Fix:** In Bevy 0.19, child UI elements must only be attached to parent entities that have a `Node` component.

### 4. Game runs but screen is black
**Check:**
1. Did you spawn a `Camera3d`?
2. Is the camera positioned looking at the scene (e.g. `Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y)`)?
3. Did you spawn a light source (`PointLight` or `DirectionalLight`)?

---

*You are now equipped with everything needed to build, modify, and master the Packwatch codebase! Happy coding!*
