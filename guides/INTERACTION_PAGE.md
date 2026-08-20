# Interaction page — ECS working guide

The overlay that opens when you look at something and press **E**. World objects own **data**. The page owns **layout**. Systems copy data into the page on E and hide it on Esc.

Do not rewrite look, walk, or the E/Esc loop. Add widgets and fields on top.

---

## 1. Files

| File | Job | Touch? |
| --- | --- | --- |
| `src/interactions/components.rs` | Data + markers + resources | Yes, when you add a field or widget |
| `src/interactions/page.rs` | Spawn layout; Esc close | Yes, when you add UI |
| `src/interactions/use_item.rs` | **E**: copy object data into the page | Yes, when the page needs new data |
| `src/interactions/mod.rs` | Register systems | Yes, when you add a **new** system |
| `src/interactions/focus.rs` | Raycast “what am I looking at?” | No |
| `src/interactions/prompt.rs` | “E to interact” hint | No |
| `src/templates/model.rs` | Puts `InspectInfo` on spawned models/NPCs | Only if new spawn fields |
| `src/camera/` `src/player/` | Freeze while page is open | No |

**Rule:** page layout stays in `page.rs`. Input that **opens** the page stays in `use_item.rs`. Input that **closes** stays in `close_page`.

---

## 2. The two worlds

```
3D entity (Guide, box, …)
  Interactable          marker — raycast can see it
  InspectInfo { title } data — what the page should show
  maybe Npc, NpcKind    extra data you can also Query

UI entity (spawned once)
  InspectionPage        full-screen overlay, starts Hidden
    └── panel
          └── InspectionTitle   Text filled on E
```

One page. Many objects. E does not spawn a new overlay. It shows the existing one and writes that object’s `InspectInfo` into the title (and later body, portrait, …).

`OpenInspection` is the switch:

- `None` — exploring (look, WASD, E work)
- `Some(entity)` — that object’s page is open (look/move/prompt freeze)

---

## 3. How a Bevy system is a function

A system is a function. Each **parameter** is something you ask the world for. Bevy fills them every frame. You never call other systems; you only declare what you need.

```rust
fn my_system(
    // resources — one global value
    time: Res<Time>,
    mut open: ResMut<OpenInspection>,

    // queries — all entities that match
    mut titles: Query<&mut Text, With<InspectionTitle>>,
    infos: Query<&InspectInfo>,

    // input
    keyboard: Res<ButtonInput<KeyCode>>,
) { ... }
```

Then register it on the plugin (see §7). If it is not in `InteractionPlugin`, it never runs.

---

## 4. System params you will actually use

Copy these. Do not invent new ways to find the same entities.

### Resources (`Res` / `ResMut`)

One of them in the whole app. Not attached to an entity.

| Param | Meaning |
| --- | --- |
| `focused: Res<FocusedInteractable>` | Who the camera is looking at (`Option<Entity>`) |
| `mut open: ResMut<OpenInspection>` | Whether a page is open, and for whom |
| `keyboard: Res<ButtonInput<KeyCode>>` | Keys this frame |
| `time: Res<Time>` | `elapsed_secs()`, `delta_secs()` for pulses |

`Res` = read. `ResMut` = write. Use `ResMut` only when you `open()` / `close()`.

### Queries

`Query<&T, With<Marker>>` = “every entity that has both `T` and `Marker`.”

| Param | Meaning |
| --- | --- |
| `inspect_info: Query<&InspectInfo>` | Data on the **3D object**. Use `.get(entity)` |
| `mut title: Query<&mut Text, With<InspectionTitle>>` | The title widget. Use `.single_mut()` — there is one |
| `mut page: Query<&mut Visibility, With<InspectionPage>>` | Show/hide the overlay |
| `npcs: Query<&NpcKind>` | Extra data on the same 3D entity, if you need it |

**`.get(entity)`** — “this one object I already know.” Used for the focused Guide.

**`.single_mut()`** — “there is exactly one of these in the world.” Used for page widgets you spawned once.

**`.iter_mut()`** — many entities. You almost never need this on the page (one overlay).

### `Single<T>`

Same as `Query<T>` that must have exactly one match:

```rust
mut cursor_options: Single<&mut CursorOptions>,
```

That is already how mouse lock is done. Fine for “the window cursor.” Prefer `Query` + `With<Marker>` for page widgets so a missing widget is a quiet `Err`, not a panic.

### `Commands`

Only when you **create or destroy** entities (`spawn_page`). Opening/closing the page does **not** use `Commands` — it toggles `Visibility` and `OpenInspection`.

### What you do **not** take

- `AssetServer` in `use_item` unless you are loading an image **at press time** (prefer a `Handle<Image>` already on `InspectInfo`)
- `Camera` / `Player` — focus already did that
- `NextState<PlayMode>` — inspect is not a match. Stay in `Exploring`

---

## 5. The systems that already exist (read these as templates)

Registered in `src/interactions/mod.rs`, **chained**, only while `PlayMode::Exploring`:

```text
update_focus → update_prompt → use_item → close_page
```

Order matters: focus must run before E, or you open last frame’s target.

### `use_item` — open (this is the one you extend)

```rust
pub fn use_item(
    keyboard: Res<ButtonInput<KeyCode>>,
    focused: Res<FocusedInteractable>,
    inspect_info: Query<&InspectInfo>,
    mut open: ResMut<OpenInspection>,
    mut page: Query<&mut Visibility, With<InspectionPage>>,
    mut title: Query<&mut Text, With<InspectionTitle>>,
    mut cursor_options: Single<&mut CursorOptions>,
)
```

Pattern:

1. Early-out if the page is already open, or E was not just pressed, or nothing is focused.
2. `inspect_info.get(entity)` — if that entity has no `InspectInfo`, do nothing (a door can be `Interactable` without a page).
3. `open.open(entity)`.
4. Show the page, copy fields into widgets, unlock the cursor.

Every new line of inspect data is: **new query param** + **copy in this function**.

### `close_page` — Esc

```rust
pub fn close_page(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut open: ResMut<OpenInspection>,
    mut page: Query<&mut Visibility, With<InspectionPage>>,
    mut cursor_options: Single<&mut CursorOptions>,
)
```

No `InspectInfo`. Closing does not care which object it was. Hide + `open.close()` + lock mouse.

### `spawn_page` — once

```rust
pub fn spawn_page(mut commands: Commands)
```

`OnEnter(GameState::Playing)`. Builds the widget tree. Starts `Visibility::Hidden`. Uses `DespawnOnExit(GameState::Playing)` so it dies with the level.

---

## 6. Recipe: add body text (the pattern for everything else)

This is the full ECS loop for “the page shows one more thing.”

### 6a. Data on the object — `components.rs`

```rust
pub struct InspectInfo {
    pub title: String,
    pub body: String,   // new
}
```

Every spawn that inserts `InspectInfo` must set `body` too (`templates/model.rs`, and `templates/character.rs` if you use capsules again).

### 6b. Marker on the widget — `components.rs`

```rust
#[derive(Component)]
pub struct InspectionBody;
```

Empty struct. It exists so the query can say `With<InspectionBody>` instead of guessing among several `Text`s.

### 6c. Spawn the widget — `page.rs`

Sibling of `InspectionTitle`, inside the panel. Panel `Node` should be a column:

```rust
Node {
    flex_direction: FlexDirection::Column,
    row_gap: px(12),
    width: px(360),
    padding: UiRect::all(px(24)),
    ..default()
}
```

Child:

```rust
(
    InspectionBody,
    Text::new(""),
    TextFont { font_size: FontSize::Px(16.0), ..default() },
    TextColor(Color::srgb(0.7, 0.7, 0.72)),
)
```

### 6d. Copy on E — `use_item.rs`

Add a param:

```rust
mut body: Query<&mut Text, With<InspectionBody>>,
```

After you copy the title:

```rust
if let Ok(mut text) = body.single_mut() {
    **text = info.body.clone();
}
```

`**text` because `Text` derefs to `String`.

### 6e. Do you need `mod.rs`?

No. `use_item` is already registered. You only edit `mod.rs` when you add a **new function** (e.g. a Close button click system).

---

## 7. Registering a new system

`src/interactions/mod.rs`:

```rust
.add_systems(
    Update,
    (update_focus, update_prompt, use_item, close_page, my_new_system)
        .chain()
        .run_if(in_state(PlayMode::Exploring)),
)
```

Put click/Esc handlers **after** `use_item` if they should see the page that just opened this frame.

If the system should run **while the page is open** (pulse a widget, hover), it still goes in this chain — look/move already stop because they check `OpenInspection`. Do not switch `PlayMode` for inspect.

Spawn-only systems:

```rust
.add_systems(OnEnter(GameState::Playing), (spawn_prompt, spawn_page))
```

---

## 8. Querying extra data on the same object

The focused entity may also have `NpcKind`, `NpcStats`, etc. Add another query and `.get` the **same** entity:

```rust
fn use_item(
    // ...
    inspect_info: Query<&InspectInfo>,
    kinds: Query<&NpcKind>,
) {
    let entity = focused.entity()?;
    let Ok(info) = inspect_info.get(entity) else { return; };
    let kind = kinds.get(entity).ok(); // None if it is a box, not an NPC
}
```

Do not put NPC stats into `InspectInfo` unless every inspectable has them. Optional data = optional `Query` + `.get().ok()`.

---

## 9. Static chrome vs per-object data

| Kind | Where it lives | Example |
| --- | --- | --- |
| Same for every inspect | Spawn in `page.rs` only | `"Esc to close"`, dim backdrop |
| Different per object | Field on `InspectInfo` (or another component) + copy in `use_item` | title, body, portrait |
| Changes while open | System that `Query`s the widget + `Res<Time>` | fade, blink |

A Close **button**: spawn `Button` in `page.rs`, new system with `Query<&Interaction, With<CloseInspectButton>>`. On click, do the same three lines as Esc (`open.close()`, hide page, lock cursor). Extract those three lines into a small function both can call if you want.

---

## 10. Images later

Put the handle on the object when you spawn it, not on E:

```rust
// InspectInfo
pub portrait: Option<Handle<Image>>,

// widget
#[derive(Component)]
struct InspectionPortrait;

// spawn_page: ImageNode::default() + InspectionPortrait, maybe Hidden

// use_item
mut portraits: Query<&mut ImageNode, With<InspectionPortrait>>,
```

Track the image in `queue_assets` (`src/screens/loading.rs`) so Loading waits for it.

---

## 11. Do not

- `commands.spawn` a new `InspectionPage` on every E (stacked overlays).
- Put inspect UI in `scene/setup.rs` or `player/`.
- Animate `TextFont.font_size` (reflows). Use `TextColor` or `UiTransform.scale`.
- Use `PlayMode::Match` for this overlay. Match is combat.
- Mark the floor `Interactable`.
- Forget `Interactable` **and** `InspectInfo` on the 3D entity. Ray hits colliders; E reads `InspectInfo`.

---

## 12. Checklist when E “does nothing”

1. Entity has `Interactable` and a **collider** (ray hits physics, not the picture).
2. Entity has `InspectInfo`.
3. You are in `PlayMode::Exploring` (inspect systems do not run in `Match`).
4. `OpenInspection` is not stuck `Some` — press Esc.
5. New widget: marker on the spawned node **and** a `Query` in `use_item`.
6. New system: listed in `InteractionPlugin`.

Start with **§6 body text**. That is the same pattern as title, buttons, and portraits.
