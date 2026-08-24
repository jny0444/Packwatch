# Match — combat working guide

Turn-based smoke fight. Exploring stays exploring. Match is its own `PlayMode`. Shop and bag stay overlays; they are not combat.

Do not despawn world NPCs and recreate them. Hide them, spawn **match copies**, despawn those copies when the match ends. World entity ids stay valid (`ActiveMatch` already stores the world opponent).

`src/combat/` already exists (`mod.rs` + empty `state.rs`). Fill those files. Do not name the module `match` (Rust keyword).

---

## 1. What a match is

```
Exploring  --E on a fighter-->  PlayMode::Match
                                  hide world NPCs
                                  spawn staged copies in front of the camera
                                  round loop until someone is down
Match      --win/lose/flee---->  PlayMode::Exploring
                                  despawn staged copies
                                  show world NPCs
                                  ActiveMatch::clear()
```

Look, walk, inspect already freeze in `Match` because they use `run_if(in_state(PlayMode::Exploring))`. Bag/shop currently run on `GameState::Playing` — you must gate those (see §3 step 14) or they open during a fight.

`src/screens/mod.rs` already has `PlayMode::Match` and `ActiveMatch`. Do not add `PlayMode::Shopping` or `PlayMode::Inventory`.

---

## 2. Files — who owns what

| File | Job | Touch? |
| --- | --- | --- |
| `src/combat/state.rs` | `MatchSession`, `MatchPhase`, `LitCig`, `ROUND_AP` | Yes — write the whole file |
| `src/combat/staging.rs` | **New.** Hide world `Npc`; spawn/show fight copies | Yes — new file |
| `src/combat/round.rs` | **New.** Loadout confirm → puffs → enemy hit → end | Yes — new file |
| `src/combat/ui.rs` | **New.** Match overlay spawn + button markers | Yes — new file |
| `src/combat/mod.rs` | `CombatPlugin`, `pub mod` every combat file | Yes — replace the stub |
| `src/lib.rs` | `PackwatchPlugin` already has `pub mod combat` | Yes — **add `CombatPlugin` to `add_plugins`** |
| `src/items/item_def.rs` | `ap_cost` + `puffs` on `ItemStats` and constructors | Yes |
| `src/items/types/cig.rs` | Per-cig AP in `CigTypes::def` | Yes — AP numbers only |
| `src/items/types/beer.rs` | Optional beer AP | Later |
| `src/items/types/gum.rs` | Leave dizziness numbers; combat treats gum as a guard | No (unless you add `guards` on the def) |
| `src/npc/npc_stats.rs` | `dizziness_limit` field + `named()` | Yes |
| `src/npc/npc_kind.rs` | Guide `dizziness_limit` and test `attack` | Yes |
| `src/player/components.rs` | `PlayerStats` next to `Player` | Yes |
| `src/player/spawn.rs` | Insert `PlayerStats` on the player spawn | Yes — one extra component |
| `src/player/mod.rs` | Re-export `PlayerStats` if spawn needs it | Yes if you split the type |
| `src/interactions/use_item.rs` | E on Guide starts a match; shopkeeper still shops | Yes — one new branch |
| `src/interactions/mod.rs` | `use_item` already registered | No |
| `src/interactions/page.rs` `focus.rs` `prompt.rs` | Inspect overlay | No |
| `src/items/mod.rs` | Gate `bag_interact` / `shop_interact` to Exploring | Yes — `run_if` only |
| `src/items/bag.rs` `shop.rs` | Overlay internals | No |
| `src/items/inventory.rs` | `has` / `remove` already exist | No |
| `src/screens/mod.rs` | `PlayMode::Match`, `ActiveMatch` already exist | No |
| `src/camera/` `src/player/movement.rs` | Already Exploring-only | No |
| `src/scene/setup.rs` | World NPC positions | No for v1 |
| `src/templates/model.rs` | `spawn_scene_model` — reuse from staging, do not rewrite | Reuse only |

**Rule:** fight **numbers** live in `MatchSession`. Fight **models** live in `staging.rs`. Fight **buttons** live in `ui.rs`. E that **starts** the fight lives in `use_item.rs`. World NPCs stay in `scene/setup.rs`.

---

## 3. Where to write what (do this order)

### Step 1 — `src/items/item_def.rs`

On `ItemStats`, add:

```rust
pub ap_cost: u32,
pub puffs: u32,
```

Update **every** constructor in this file (`none`, `cig`, `beer`, `gum`) so they still compile:

- `cig(...)` → `ap_cost` argument (or default 1) and `puffs: 4`
- `beer(...)` → `ap_cost` (2 for now), `puffs: 0`
- `gum(...)` → `ap_cost: 0`, `puffs: 0`
- `none()` → both 0

Do not put AP on `ItemDef` as a second copy. Read it via `kind.def().stats.ap_cost`.

### Step 2 — `src/items/types/cig.rs`

In `CigTypes::def`, you already unpack `(name, description, player, enemy)`. Extend that tuple with AP, then pass it into `ItemStats::cig`.

```text
MarlboroRed              AP 2
DoubleHappiness11mg      AP 2
CamelYellow              AP 2
everything else          AP 1
```

Do not change dizziness numbers here. Do not touch `model()`.

### Step 3 — `src/npc/npc_stats.rs`

Add `pub dizziness_limit: f32` next to `dizziness`.

`dizziness` = current (starts 0 at match start, stored on **session**, not mutated on the world NPC).

`dizziness_limit` = KO line.

Update `NpcStats::named` to set `dizziness_limit` (e.g. `60.0` for unused kinds).

### Step 4 — `src/npc/npc_kind.rs`

In the `NpcKind::Guide => NpcStats { ... }` arm:

- set `dizziness_limit: 80.0`
- set `attack: 15.0` for testing (file currently has `100.0`, which one-shots)

Do not add a new `NpcKind`. Shopkeeper stays a shop.

### Step 5 — `src/player/components.rs`

Under `Player`, add:

```rust
#[derive(Component)]
pub struct PlayerStats {
    pub dizziness: f32,
    pub dizziness_limit: f32,
}
```

Default limit `100.0`. Current `dizziness: 0.0`.

### Step 6 — `src/player/spawn.rs`

In `spawn_player`, add `PlayerStats { dizziness: 0.0, dizziness_limit: 100.0 }` to the **same** spawn tuple as `Player`. Do not put it on the camera child.

Re-export `PlayerStats` from `src/player/mod.rs` (`pub use components::{Player, PlayerStats}`).

### Step 7 — `src/combat/state.rs` (file exists, currently a stub)

**This file is data only.** No `fn` systems, no `Commands`, no UI. Replace the stub (`ROUND_AP` + broken `MatchPhase`) with the types below.

Imports:

```rust
use bevy::prelude::*;

use crate::items::ItemKind;
use crate::npc::NpcStats;
use crate::player::PlayerStats;
```

Keep `pub const ROUND_AP: u32 = 5;` (already in the file).

Add:

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MatchPhase {
    Loadout,
    Resolve,
}

pub struct LitCig {
    pub kind: ItemKind,
    pub puffs_left: u32,
}

#[derive(Resource)]
pub struct MatchSession {
    pub phase: MatchPhase,
    pub round: u32,
    pub ap_cap: u32,
    pub ap_left: u32,
    pub loadout: Vec<ItemKind>,
    pub lit: Vec<LitCig>,
    pub guards: u32,
    pub player_dizziness: f32,
    pub player_limit: f32,
    pub enemy_name: String,
    pub enemy_dizziness: f32,
    pub enemy_limit: f32,
    pub enemy_attack: f32,
}
```

`from_fighters` copies **limits and attack** off the components. Current dizziness on the session starts at `0.0` even if the world NPC’s `dizziness` field is 0. Do not write back to `NpcStats` later.

```rust
impl MatchSession {
    pub fn from_fighters(player: &PlayerStats, enemy: &NpcStats) -> Self {
        Self {
            phase: MatchPhase::Loadout,
            round: 1,
            ap_cap: ROUND_AP,
            ap_left: ROUND_AP,
            loadout: Vec::new(),
            lit: Vec::new(),
            guards: 0,
            player_dizziness: 0.0,
            player_limit: player.dizziness_limit,
            enemy_name: enemy.name.clone(),
            enemy_dizziness: 0.0,
            enemy_limit: enemy.dizziness_limit,
            enemy_attack: enemy.attack,
        }
    }

    pub fn next_round(&mut self) {
        self.phase = MatchPhase::Loadout;
        self.round += 1;
        self.ap_left = self.ap_cap;
        self.loadout.clear();
        self.lit.clear();
        // keep dizziness + guards
    }

    pub fn player_down(&self) -> bool {
        self.player_dizziness >= self.player_limit
    }

    pub fn enemy_down(&self) -> bool {
        self.enemy_dizziness >= self.enemy_limit
    }
}
```

`ActiveMatch` stays in `src/screens/mod.rs`. Do not put `opponent: Entity` on `MatchSession`.

`ItemStats` fields you will read later: `kind.def().stats.ap_costs` and `kind.def().stats.puffs` (`Option<u32>` — cigs `Some(4)`, gum/beer `None`).

---

### Step 8 — `src/combat/staging.rs` (done)

Hide/show world NPCs, pose a Guide copy in front of the camera. Do not despawn the world opponent.

**In this file now:**

- `MatchProp`
- `hide_world_npcs` / `show_world_npcs`
- `fighter_model`
- `spawn_match_page` spawns a non-NPC copy with `spawn_scene_model` (`.not_interactable()`, no `.as_npc()`), then inserts `MatchProp` + `DespawnOnExit(PlayMode::Match)`

**Also done:** `spawn_scene_model` in `src/templates/model.rs` returns `Entity`.

Step 11 registers these on `OnEnter(PlayMode::Match)` / `OnExit`.

---

### Step 9 — `src/combat/ui.rs` (done)

Overlay + text sync. No inventory mutation. Style copied from `src/items/shop.rs`.

**In this file now:** `MatchPage`, text markers, `MatchConfirmButton`, `MatchItemButton`, `spawn_match_ui`, `update_match_texts`. Item buttons are unique cig/gum kinds from the bag. Cursor unlocks on spawn.

#### Markers (all in this file)

```rust
#[derive(Component)]
pub struct MatchPage;

#[derive(Component)]
pub struct MatchApText;

#[derive(Component)]
pub struct MatchPlayerDizzyText;

#[derive(Component)]
pub struct MatchEnemyDizzyText;

#[derive(Component)]
pub struct MatchLoadoutText;

#[derive(Component)]
pub struct MatchGuardsText;

#[derive(Component)]
pub struct MatchConfirmButton;

#[derive(Component, Clone, Copy)]
pub struct MatchItemButton {
    pub kind: ItemKind,
}
```

#### `spawn_match_ui`

Schedule: `OnEnter(PlayMode::Match)`.

Params: `mut commands`, `inventory: Res<Inventory>`, `mut cursor_options: Single<&mut CursorOptions>`.

1. `set_cursor_locked(&mut cursor_options, false);` so the player can click buttons.
2. Spawn a full-screen overlay:

```rust
commands.spawn((
    MatchPage,
    DespawnOnExit(PlayMode::Match),
    Node {
        width: percent(100),
        height: percent(100),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.35)),
))
```

3. Child panel (~720px wide, column, padding). Inside it, spawn labeled `Text` nodes and attach the text markers (`MatchApText`, etc.). Start the strings as `""` — `update_match_texts` fills them next frame.
4. Item row: loop `inventory.slots(Pocket::Items)`. For each `Some(stack)` whose kind is `ItemKind::Cig(_) | ItemKind::Gum(_)` (skip beer in v1), spawn a `Button` with `MatchItemButton { kind: stack.kind }` and a child `Text::new(kind.def().name)`.
5. Confirm `Button` + `MatchConfirmButton` + child text `"Confirm"`.

Do not spawn this from `interactions/page.rs`. Do not put `InspectionPage` / `ShopPage` / `InventoryPage` on this overlay.

Duplicate stacks of the same cig can spawn two buttons. That is fine for v1.

#### `update_match_texts`

Schedule: `Update`, only in `PlayMode::Match`.

Params: `session: Res<MatchSession>`, plus `Query<&mut Text, With<MatchApText>>` (and the other text markers). Use `Without<>` if two `Query<&mut Text>` fight.

If `session` is missing (`Option<Res<MatchSession>>`), return.

Write:

```text
AP: {session.ap_left} / {session.ap_cap}
You: {session.player_dizziness.round()} / {session.player_limit}
{session.enemy_name}: {session.enemy_dizziness.round()} / {session.enemy_limit}
Loadout: comma-separated kind.def().name (or "(empty)")
Guards: {session.guards}
```

Assign with `**text = format!(...);` like shop/bag.

No clicking here. Clicks are step 10.

---

### Step 10 — `src/combat/round.rs` (done)

Rules only. No `Node` spawn. No GLTF.

**In this file now:** `init_session`, `clear_session` (also re-locks cursor), `match_loadout_input`, `match_confirm`, `match_resolve`. Resolve takes `session.lit` first so puff math can mutate dizziness without a double borrow.

Helpers in this file (private):

```rust
fn is_cig(kind: ItemKind) -> bool {
    matches!(kind, ItemKind::Cig(_))
}

fn is_gum(kind: ItemKind) -> bool {
    matches!(kind, ItemKind::Gum(_))
}

fn loadout_count(session: &MatchSession, kind: ItemKind) -> u32 {
    session.loadout.iter().filter(|item| **item == kind).count() as u32
}
```

#### `init_session`

Schedule: `OnEnter(PlayMode::Match)` (with staging + UI).

```rust
pub fn init_session(
    mut commands: Commands,
    active: Res<ActiveMatch>,
    players: Query<&PlayerStats, With<Player>>,
    npcs: Query<&NpcStats>,
)
```

- `let Some(opponent) = active.opponent() else { return; };`
- `let Ok(player) = players.single() else { return; };`
- `let Ok(enemy) = npcs.get(opponent) else { return; };`
- `commands.insert_resource(MatchSession::from_fighters(player, enemy));`

World `NpcStats` is still on the **hidden** Guide entity, so this query works.

#### `clear_session`

Schedule: `OnExit(PlayMode::Match)`.

```rust
pub fn clear_session(mut commands: Commands, mut active: ResMut<ActiveMatch>) {
    commands.remove_resource::<MatchSession>();
    active.clear();
}
```

Also lock the cursor again: `set_cursor_locked(..., true)` (add `Single<&mut CursorOptions>`).

#### `match_loadout_input`

Schedule: `Update` in Match, **before** confirm. Skip unless `session.phase == MatchPhase::Loadout`.

Params: `mut session: ResMut<MatchSession>`, `inventory: Res<Inventory>`, `buttons: Query<(&Interaction, &MatchItemButton), Changed<Interaction>>`.

For each `Interaction::Pressed`:

1. `kind = button.kind`
2. `needed = loadout_count(&session, kind) + 1`
3. `if !inventory.has(kind, needed) { continue; }`
4. If `is_cig(kind)`:
   - `if !inventory.has(ItemKind::Lighter, 1) { continue; }`
   - `cost = kind.def().stats.ap_costs`
   - `if cost > session.ap_left { continue; }`
   - `session.ap_left -= cost`
5. If `is_gum(kind)`: AP stays (gum `ap_costs` is already 0)
6. Skip beer in v1 (`continue`)
7. `session.loadout.push(kind)`

Do not `inventory.remove` here. Removing is Confirm.

Optional: Escape while `Loadout` pops the last loadout item and refunds that item’s `ap_costs`. Not required for v1.

#### `match_confirm`

Schedule: `Update` in Match, after loadout input. Skip unless `Loadout`.

Params: `mut session`, `mut inventory: ResMut<Inventory>`, `confirm: Query<&Interaction, (Changed<Interaction>, With<MatchConfirmButton>)>`, `keyboard: Res<ButtonInput<KeyCode>>`.

Fire when Confirm is `Pressed` **or** `keyboard.just_pressed(KeyCode::Enter)`.

If `session.loadout.is_empty() { return; }` (v1: must pick something).

Then:

```rust
for kind in session.loadout.clone() {
    inventory.remove(kind, 1);
    let def = kind.def();
    if is_cig(kind) {
        let puffs = def.stats.puffs.unwrap_or(4);
        session.lit.push(LitCig { kind, puffs_left: puffs });
    } else if is_gum(kind) {
        session.guards += 1;
    }
}
session.loadout.clear();
session.phase = MatchPhase::Resolve;
```

Never `remove` `ItemKind::Lighter`.

#### `match_resolve`

Schedule: `Update` in Match, after confirm. Skip unless `session.phase == MatchPhase::Resolve`.

Params: `mut session`, `mut next_mode: ResMut<NextState<PlayMode>>`.

V1 runs the whole resolve in **one** frame (no puff animation):

```text
for each LitCig in session.lit:
    let puffs = cig.puffs_left.max(1) as f32
    let stats = cig.kind.def().stats
    per_enemy  = stats.enemy_dizziness / puffs
    per_player = stats.player_dizziness / puffs
    while cig.puffs_left > 0:
        session.enemy_dizziness += per_enemy
        session.player_dizziness += per_player
        cig.puffs_left -= 1
        if session.enemy_down() || session.player_down() → go Exploring, return

if session.guards > 0:
    session.guards -= 1
else:
    session.player_dizziness += session.enemy_attack

if session.player_down() || session.enemy_down():
    next_mode.set(PlayMode::Exploring)
else:
    session.next_round()
```

Use a `for cig in session.lit.iter_mut()` loop. After the puff loop, `session.lit.clear()`.

`next_mode.set(PlayMode::Exploring)` runs `OnExit(Match)`: copies despawn, NPCs unhide, `clear_session`.

Win/lose UI can wait. V1 just dumps you back in the room.

Do **not** put this logic in `use_item.rs`.

---

### Step 11 — `src/combat/mod.rs` (done)

`CombatPlugin` lives here. OnEnter order: hide NPCs → `spawn_match_page` → `spawn_match_ui` → `init_session`. Do not register combat systems on `InteractionPlugin` or `ItemsPlugin`.

```rust
pub mod round;
pub mod staging;
pub mod state;
pub mod ui;

use bevy::prelude::*;

use crate::screens::PlayMode;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayMode::Match),
            (
                staging::hide_world_npcs,
                staging::spawn_match_page,
                ui::spawn_match_ui,
                round::init_session,
            ).chain(),
        )
        .add_systems(
            OnExit(PlayMode::Match),
            (staging::show_world_npcs, round::clear_session).chain(),
        )
        .add_systems(
            Update,
            (
                ui::update_match_texts,
                round::match_loadout_input,
                round::match_confirm,
                round::match_resolve,
            )
                .chain()
                .run_if(in_state(PlayMode::Match)),
        );
    }
}
```

`.chain()` on enter so hide runs before the copy is spawned.

Make `hide_world_npcs`, `spawn_match_stage`, `spawn_match_ui`, `init_session`, etc. `pub` so `mod.rs` can name them.

---

### Step 12 — `src/lib.rs` (done)

`CombatPlugin` is in `PackwatchPlugin` after `ItemsPlugin`.

In `PackwatchPlugin::build`, add:

```rust
use combat::CombatPlugin;
```

and put `CombatPlugin` in `add_plugins((...))` (after `ItemsPlugin` is fine).

If you skip this, Match state can change and nothing combat-related will run.

---

### Step 13 — `src/interactions/use_item.rs` (done)

E on Guide starts a match. Shopkeeper still shops. Everyone else still inspects. Do not add systems to `src/interactions/mod.rs`.

Add params:

```rust
mut active: ResMut<ActiveMatch>,
mut next_mode: ResMut<NextState<PlayMode>>,
```

Imports: `crate::screens::{ActiveMatch, PlayMode}`.

Inside `use_item`, **after** the shopkeeper branch, **before** `inspect_info.get(entity)`:

```rust
if kinds.get(entity) == Ok(&NpcKind::Guide) {
    open.close();
    active.start(entity);
    next_mode.set(PlayMode::Match);
    set_cursor_locked(&mut cursor_options, false);
    return;
}
```

Shopkeeper still opens shop. Any other NPC still opens inspect.

Do not start a match from `src/scene/setup.rs`.

Focus/look already stop in Match (`run_if(Exploring)`). You do not freeze them again here.

---

### Step 14 — `src/items/mod.rs` (done)

Shop/bag input is Exploring-only. Wallet HUD still updates during a fight.

Split it:

```rust
.add_systems(
    Update,
    (
        update_wallet_hud,
        update_bag_visuals,
        update_shop_visuals,
        sync_shop_preview,
        rotate_preview,
        sync_preview_layers,
        animate_purchase,
    )
        .chain()
        .run_if(in_state(GameState::Playing)),
)
.add_systems(
    Update,
    (shop_interact, bag_interact)
        .chain()
        .run_if(in_state(PlayMode::Exploring)),
)
```

Import `PlayMode` from `crate::screens::PlayMode`.

Do not edit `shop.rs` / `bag.rs` internals. Wallet HUD can stay visible during a fight.

---

### After step 14 — compile check

`cargo check`. Typical misses:

- `spawn_scene_model` does not return `Entity` yet — add `model.id()`
- two `Query<&mut Text>` without `Without<OtherMarker>`
- `MatchSession` used in UI before `init_session` runs: order them on `OnEnter` with `.chain()`, UI update uses `Option<Res<MatchSession>>`
- `NpcKind::ShopKeeper` vs `ShopKeeper` — use whatever `src/npc/npc_kind.rs` actually has (`ShopKeeper`, `Guide`)
- `ap_costs` (plural) on `ItemStats`, not `ap_cost`

---

## 4. Do not hide-and-respawn the world

World NPCs own `Npc`, `NpcKind`, `NpcStats`, colliders, inspect data. If you despawn them:

- `ActiveMatch` points at a dead entity
- shop/guide lose their world place
- GLTFs load again every fight

On **enter Match** (`staging.rs` + `round.rs`):

1. `ActiveMatch::start(world_opponent)` — already done in `use_item`
2. Hide every `With<Npc>`
3. Spawn copies in fight slots: same GLTF, `not_interactable()`, `MatchProp`, `DespawnOnExit(PlayMode::Match)`
4. Place the opponent in front of the camera; ignore crowd NPCs for v1

On **exit Match**:

1. Copies die with `DespawnOnExit(PlayMode::Match)`
2. `show_world_npcs`
3. `ActiveMatch::clear()` in `clear_session`

Fight numbers live on `MatchSession`, not on the copy.

```
World Guide (hidden)     MatchProp copy (visible, in front of camera)
entity 88  ◄── ActiveMatch stores this
NpcKind::Guide
NpcStats
```

---

## 5. Numbers

### Round AP

```rust
// src/combat/state.rs
pub const ROUND_AP: u32 = 5;
```

Spend up to 5 per loadout. Unused AP is waste, not banked.

### Cig = AP + 4 puffs

`ItemStats.puffs` is 4 for cigs. Def dizziness is **per stick**, split across puffs:

```text
per_puff_enemy  = def.stats.enemy_dizziness  / puffs
per_puff_player = def.stats.player_dizziness / puffs
```

Marlboro Red is 8 / 18. Apply that four times and the fight is over.

**Lighter:** `inventory.has(ItemKind::Lighter, 1)` to add a cig. Do not `remove` the lighter.

**Inventory:** on Confirm, `inventory.remove(kind, 1)` per loadout item. Use existing methods in `src/items/inventory.rs`.

### Gum = one guard, not a heal

Do not apply gum `player_dizziness` (-22 / -10) in combat. Gum: 0 AP, `guards += 1`. Enemy hit: spend a guard or take `enemy_attack`.

### Beer

Skip until cigs + gum work. Slot: AP cost, `puffs: 0`, Confirm applies `sp_attack` once.

### KO

Session copies `player_dizziness` / `enemy_dizziness`. KO when `>=` that fighter’s `dizziness_limit`. Do not write HP back onto the world `NpcStats`.

---

## 6. `MatchSession` (in `state.rs` only)

`ActiveMatch` = who you picked in the world. `MatchSession` = the fight in progress.

```rust
pub enum MatchPhase {
    Loadout,
    Resolve,
}

pub struct LitCig {
    pub kind: ItemKind,
    pub puffs_left: u32,
}

#[derive(Resource)]
pub struct MatchSession {
    pub phase: MatchPhase,
    pub round: u32,
    pub ap_cap: u32,
    pub ap_left: u32,
    pub loadout: Vec<ItemKind>,
    pub lit: Vec<LitCig>,
    pub guards: u32,
    pub player_dizziness: f32,
    pub player_limit: f32,
    pub enemy_dizziness: f32,
    pub enemy_limit: f32,
    pub enemy_attack: f32,
}
```

New round: reset `ap_left`, `loadout`, `lit`. Keep dizziness and leftover guards.

---

## 7. Round loop (implemented in `round.rs`)

```
Loadout   match_loadout_input + UI buttons
Confirm   match_confirm  (consume items, light cigs, add guards)
Resolve   match_resolve  (puffs, then one enemy hit, then win/lose/next)
```

V1: Confirm burns all puffs immediately, then the enemy swings once. No puff animation.

---

## 8. How a fight starts (implemented in `use_item.rs`)

V1: E on `NpcKind::Guide` → `ActiveMatch.start` + `PlayMode::Match`. Shopkeeper still opens shop. Everyone else still inspects.

Later: Fight button on the inspect page. Not now.

---

## 9. UI (implemented in `ui.rs`)

One overlay, `DespawnOnExit(PlayMode::Match)`:

- Enemy name + dizziness / limit
- Player dizziness / limit
- `AP left / ROUND_AP`
- Loadout list
- Guards
- Item buttons + Confirm

---

## 10. Do not

- Despawn world NPCs to “move” them
- Store fight HP on the world `NpcStats`
- Put match systems in `InteractionPlugin`
- Open inspect/shop/bag during Match
- Apply full cig dizziness four times
- Consume the lighter
- Spend money in combat
- Rewrite look / move / E
- Name the module `match`

---

## 11. First slice (stop here before juice)

1. E on Guide → Match, NPCs hidden, Guide copy in front of you
2. `MatchSession` with AP 5
3. Confirm cigs + optional gum
4. Puffs apply split dizziness; gum blocks one hit
5. Win/lose → Exploring, copies gone, NPCs visible

No puff animation, no crowd, no beer, no inspect Fight button until that loop works.
