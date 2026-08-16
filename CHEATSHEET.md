# Packwatch cheatsheet

You **can** start building the game on what you have. You already have the loop most 3D games need first:

- a window and a 3D world
- a player who looks (mouse) and walks (WASD)
- gravity and solid objects (you do not fall through the floor)
- looking at an object and pressing **E** to open a page
- **Esc** to close that page and go back to walking around

That is a foundation, not a finished game. The floor is small, there is one test cube, and there is no real story, inventory, or map yet. Add those **on top of this**. Do not rewrite look / move / E unless something is broken.

This file explains the ideas behind the code you already wrote. You do not need prior game experience.

---

## 1. How a game thinks

A game is not a comic you draw once. It is a **loop** that runs many times per second:

1. Read input (keys, mouse).
2. Update the world (walk, physics, “am I looking at the cube?”).
3. Draw what the camera sees.

Bevy runs that loop for you. You only write **systems**: small functions that run every frame (or once at start).

### Entities, components, systems (ECS)

Think of the world as a spreadsheet.

| Idea | Plain English | In your game |
| --- | --- | --- |
| **Entity** | One row. An ID for “this thing.” | The player, the cube, the floor, a UI button |
| **Component** | A column. Data glued onto that row. | `Player`, `Transform`, `Interactable` |
| **System** | A function that finds matching rows and changes them | `player_move`, `update_focus` |
| **Resource** | Not a row. One global value for the whole game | `FocusedInteractable`, `OpenInspection` |

You never write `player.x += 1` on a giant `Player` object with everything inside. You ask: “find the entity that has `Player` and give me its `Transform`.”

A **marker component** is a tag with no extra data. `Player` and `Interactable` are tags. `InspectInfo { title }` is a component that also holds a name.

---

## 2. Your folder map

```
src/
  main.rs              starts the engine
  lib.rs               PackwatchPlugin (your game plugins)
  camera/              look, mouse lock, crosshair
  player/              WASD
  scene/               spawn floor, light, cube, player
  interactions/        look-at, prompt, inspect page
```

**Rule:** new feature = new folder (or a new file in an existing folder). Camera code does not go in `player/`. UI for inspect does not go in `scene/`.

`main.rs` should stay small: only “start Bevy + physics + PackwatchPlugin.”

---

## 3. Two libraries

| Crate | Job |
| --- | --- |
| **Bevy** (`0.19`) | Window, input, 3D drawing, UI, the ECS |
| **Avian** (`avian3d 0.7`) | Physics: gravity, collisions, velocity, raycasts |

Bevy draws boxes. Avian decides whether those boxes **block** you or **move** when you bump them.

---

## 4. App, plugins, schedules

### `App`

The whole program. You add plugins and systems, then `.run()`.

### Plugin

A bundle of systems (and sometimes resources) for one feature. You have:

- `ScenePlugin` — spawn the world once
- `CameraPlugin` — look + cursor + crosshair
- `PlayerPlugin` — move
- `InteractionPlugin` — E / inspect page
- `PackwatchPlugin` — those four together
- `DefaultPlugins` — Bevy’s engine
- `PhysicsPlugins` — Avian

When you add a feature, add it as a plugin on `PackwatchPlugin`.

### `Startup` vs `Update`

| Schedule | When | Examples |
| --- | --- | --- |
| `Startup` | Once, when the game starts | Spawn floor, player, crosshair, hidden inspect page |
| `Update` | Every frame | Look, move, raycast, E, Esc |

### `.chain()`

Systems in a tuple run in **that order**, in the same frame.

```text
update_focus → update_prompt → use_item → close_page
```

Focus must run **before** E, or you might open last frame’s target. Prompt must run after focus, or the hint lags.

---

## 5. Spawning (creating things)

`commands.spawn(( ... ))` creates one entity and attaches every component in the tuple.

Example (the cube): mesh + color + position + physics + “can be used” + display name.

`meshes.add(...)` / `materials.add(...)` stores a mesh or color in Bevy’s **asset** storage and returns a handle. `Mesh3d(handle)` means “draw this mesh.”

### `children![]`

Makes **child** entities. A child’s `Transform` is relative to the parent.

- Camera is a child of the player, at local `y = 0.85`. If the player walks or turns, the camera goes with them.
- Crosshair bars are children of a full-screen UI node.
- Inspect title text is a child of the dark panel.

Same idea as “GoPro bolted to a person.”

### `Transform` vs `GlobalTransform`

| Type | Meaning |
| --- | --- |
| `Transform` | Local: where I am **relative to my parent** |
| `GlobalTransform` | World: where I am **in the level** |

The camera’s `Transform` is only pitch + eye height. For a ray “from the eyes into the world,” you must use `GlobalTransform` and `compute_transform()`.

---

## 6. 3D vs UI

Two different kinds of things, both spawned as entities.

**3D world** (the room):

- `Transform`, `Mesh3d`, `MeshMaterial3d`
- `Camera3d`, `DirectionalLight`
- Physics: `RigidBody`, `Collider`, …

**UI** (crosshair, “E to interact”, inspect page):

- `Node { ... }` — a layout box (like a CSS div): size, padding, center/bottom
- `BackgroundColor`
- `Text`, `TextFont`, `TextColor`
- `Visibility` — `Visible` or `Hidden`

`px(14)` is 14 pixels. `percent(100)` is 100% of the parent. `..default()` means “fill in the rest with Bevy’s defaults.”

The OS mouse is **not** the crosshair. You hide the system cursor and draw a plus in the **center of the screen**, because look-direction **is** screen center.

---

## 7. Query (finding things every frame)

A `Query` is: “give me every entity that matches this.”

```text
Query<WHAT_TO_BORROW, FILTER>
```

| You write | Meaning |
| --- | --- |
| `&Transform` | Read position/rotation |
| `&mut LinearVelocity` | Write velocity |
| `(&A, &mut B)` | Both, on the same entity |
| `With<Player>` | Must have `Player` |
| `Without<Player>` | Must **not** have `Player` |
| `Query<(), With<Interactable>>` | I only care that the tag exists |
| `Query<Entity, With<Player>>` | I only need the ID |

`single()` / `single_mut()`: “there should be exactly one.” You have one player and one inspect page.

`.get(entity)`: “does **this** entity have this component?” Used to read `InspectInfo` from whoever you are looking at.

`Without<Player>` on the camera query lets you mutably access the player `Transform` and the camera `Transform` in one system. They are different entities; Bevy needs the filter so it knows they don’t overlap.

`Single<&mut CursorOptions>` is a query that already means “exactly one.”

---

## 8. Resources (global game state)

Not attached to one entity.

| Resource | Meaning |
| --- | --- |
| `FocusedInteractable(Option<Entity>)` | Crosshair is on this thing, or nothing |
| `OpenInspection(Option<Entity>)` | Inspect page is open on this thing, or closed |

`Res<T>` = read. `ResMut<T>` = write. `init_resource::<T>()` creates it at startup (`Default`).

`open.0` is the value inside the tuple struct. `None` = closed / not looking. `Some(entity)` = that entity.

While the page is open, look / move / focus **return early**. Two modes:

1. **Exploring** — FPS, locked mouse
2. **Inspecting** — page visible, free mouse, no WASD/look

---

## 9. Input

| API | Meaning |
| --- | --- |
| `Res<ButtonInput<KeyCode>>` | Keys this frame |
| `.pressed(KeyW)` | Held down |
| `.just_pressed(KeyE)` | Went down **this** frame (once) |
| `MessageReader<MouseMotion>` | Mouse movement since last frame |
| `event.delta` | How far the mouse moved |

Bevy 0.19 calls buffered events **messages**. That is why it is `MessageReader`, not `EventReader`.

---

## 10. Physics (Avian)

| Piece | Meaning |
| --- | --- |
| `RigidBody::Static` | Never moves (floor) |
| `RigidBody::Dynamic` | Gravity + can be pushed (player, cube) |
| `Collider::cuboid(w, h, d)` | Box-shaped hit volume (full size, not half) |
| `Collider::capsule(radius, length)` | Player shape. `length` is the cylinder, not counting the round caps |
| `LinearVelocity` | Speed. `x`/`z` = walk. `y` = gravity/jump (you don’t jump) |
| `LockedAxes::ROTATION_LOCKED` | Physics will not tip the capsule over. You still set yaw yourself |
| `TransformInterpolation` | Smooth drawing between physics steps |

**Do not** move a physics body by writing `transform.translation` every frame. Set `LinearVelocity` and let Avian move it. (You **do** write `transform.rotation` for look; rotation is locked so physics won’t fight you.)

Player spawn `y = 0.85` is the **center** of the capsule, not the feet. Camera local `y = 0.85` puts the eyes at about `1.7` in the world.

Floor `Collider::cuboid(10, 0.1, 10)` is a thin box so you can stand on it. The green plane is just the picture; the collider is what you stand on.

### Raycast (`SpatialQuery`)

A ray is an invisible line from a point in a direction.

Every frame, from the **camera’s world position**, go **forward** up to `INTERACT_RANGE` (3 meters). First thing you hit: if it has `Interactable`, store it in `FocusedInteractable`.

`SpatialQueryFilter::from_excluded_entities([player])` skips your own body, or the ray starts inside the capsule and never leaves.

---

## 11. Look math (short)

- **Yaw** = turn left/right. Applied on the **player**. Walking uses the player’s facing, so W follows the look.
- **Pitch** = look up/down. Applied only on the **camera**, so the body does not tilt.
- Pitch is clamped (`±1.54` radians, almost 90°) so you cannot flip upside down.
- `Quat::from_rotation_y` / `from_rotation_x` = “rotate around that axis.”
- Flatten `forward`/`right` onto XZ (`y = 0`) so looking up does not make you fly.

`1.54` radians ≈ 88°. Games often store angles in radians.

---

## 12. UI pieces you spawned

| Thing | Marker | Behavior |
| --- | --- | --- |
| Crosshair | `Crosshair` | Screen center. Hidden while the inspect page is open |
| “E to interact” | `InteractPrompt` | Visible only if focused **and** page closed |
| Inspect panel | `InspectionPage` | Hidden until E |
| Title text | `InspectionTitle` | Filled from `InspectInfo.title` |

Show/hide with `Visibility`. You spawn once; you do **not** spawn a new page every time you press E.

---

## 13. Controls

| Input | Action |
| --- | --- |
| Left click | Lock mouse, hide system cursor (enter FPS look) |
| Mouse | Look |
| WASD | Walk (relative to look, on the ground) |
| Look at cube + **E** | Open inspect page (“Test Cube”) |
| **Esc** (page open) | Close page, lock mouse again |
| **Esc** (page closed) | Unlock mouse (your `grab_cursor` still does this) |

---

## 14. How to add a new inspectable object

Copy the cube spawn. Change mesh, position, color, and title. Keep:

- `RigidBody` + matching `Collider` (or it won’t block / the ray won’t hit)
- `Interactable`
- `InspectInfo { title: "...".into() }`

Do not put `Interactable` on the floor unless you want E to inspect the ground.

---

## 15. Glossary (your symbols)

| In the code | Meaning |
| --- | --- |
| `f32` | A decimal number (positions, angles, speed) |
| `Vec3` | `x, y, z`. In Bevy, **Y is up** |
| `Quat` | Rotation |
| `Dir3` | A direction of length 1 (e.g. camera forward) |
| `Entity` | ID of one thing |
| `Color::srgb(r,g,b)` | Color, each channel `0.0`–`1.0` |
| `Option<T>` | `Some(value)` or `None` |
| `Res` / `ResMut` | Read / write a resource |
| `Commands` | Queue spawns/despawns this frame |
| `let Ok(x) = ... else { return }` | If it failed, skip this frame |

---

## 16. What this stack is **not** yet

Fine to ignore until you need them:

- A real level (walls, rooms, falling off the 10×10 plane)
- Inventory / picking items up (E opens a page; it does not keep the cube)
- Save/load, menus, audio, animations
- Bevy `States` (a cleaner Exploring / Inspecting split; your `OpenInspection` resource is enough for now)
- Jump (you chose not to)

Build the **world and objects** next. The character controller and inspect loop can stay.

---

## 17. Tiny habits

- One plugin per feature.
- Marker components (`Player`, `Interactable`) to find things.
- Resources for “what am I looking at / is a menu open.”
- `GlobalTransform` for world-space rays; local `Transform` for attaching the camera.
- Physics velocity for walking; don’t fight Avian with raw position.
- Early `return` when `OpenInspection` is open so FPS systems don’t run during UI.

If something “does nothing,” check order: is the ray using world transform? Is a freeze check **inverted** (`is_none` vs `is_some`)? Is the plugin actually added in `lib.rs`?
