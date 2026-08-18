# Working on the interaction page

This is the overlay that opens when you look at something and press **E**. Right now it is a dark panel with a title. You will grow it into notes, clues, item descriptions, and later inventory.

Do **not** rewrite look, walk, or the E/Esc loop unless something is broken. Add content **on top**.

---

## 1. What already works

| Action | Result |
| --- | --- |
| Look at the cube + **E** | Page opens. Title = `InspectInfo.title` |
| **Esc** | Page closes. Mouse locks. You can walk again |
| Page open | No look, no WASD, no “E to interact”, no crosshair |
| Page closed | Crosshair back. FPS mode |

You spawn the page **once** when the game enters Playing (`spawn_page`). E only flips `Visibility` and fills the title. Do not spawn a new page every time.

---

## 2. Files (keep camera/player out)

| File | Job |
| --- | --- |
| `src/interactions/page.rs` | Layout: panel, title, future body/buttons |
| `src/interactions/use_item.rs` | **E**: open page, copy data from the object into the UI |
| `src/interactions/components.rs` | `InspectInfo` (data on the object) + markers (`InspectionPage`, `InspectionTitle`) |
| `src/scene/setup.rs` | Put `Interactable` + `InspectInfo` on things in the world |
| `src/interactions/focus.rs` | Raycast: “am I looking at it?” (leave this) |
| `src/interactions/prompt.rs` | “E to interact” (leave this) |

**Rule:** world objects own **data**. The page owns **how it looks**. `use_item` copies data into the page when you press E.

---

## 3. The tree you spawn

```
InspectionPage          full screen, centered, Hidden until E
 └── panel              dark box, 360px wide, padding
      └── InspectionTitle    text, filled on E
```

Add new widgets as **siblings of the title**, inside the panel: body text, a photo, a Close hint, later buttons.

Give each widget a **marker component** (like `InspectionTitle`) so `use_item` can find it with a `Query`.

---

## 4. Suggested order of work

Do these in order. Each step is a small, playable change.

### Step A — Body text (do this first)

1. Add a field on `InspectInfo`, e.g. `body: String`.
2. Add a marker `InspectionBody` and a second `Text` child under the panel (smaller font, gray).
3. In `setup.rs`, set the cube’s body: a sentence about the cube.
4. In `use_item`, copy `info.body` into that text the same way you copy the title.

Check: E on the cube shows title **and** a paragraph. Esc still closes.

### Step B — Layout that can grow

The panel is a single column. When you add the body:

- On the **panel** `Node`, set `flex_direction: FlexDirection::Column` and `row_gap: px(12)` so title sits above body.
- Give the body a smaller `font_size` than the title.
- If the paragraph is long, set `max_width` on the panel (you already have `width: px(360)`).

This is UI, not 3D. Same tools as the start screen: `Node`, `Text`, `BackgroundColor`.

### Step C — Different objects, different pages

Copy the cube spawn. Change mesh, position, **and** `InspectInfo`. One page UI; many objects. E fills the same title/body from whichever entity is in `OpenInspection`.

You do **not** need one page prefab per item.

### Step D — Close hint / extra chrome

Add a third text: `"Esc to close"`. It does not need `InspectInfo`; it is always the same. Spawn it in `spawn_page` only.

### Step E — Later (not yet)

- Image / portrait: `ImageNode` + a path in `InspectInfo` (`Handle<Image>` from `AssetServer`)
- Clickable **Close** button: `Button` child; on click do the same as Esc in `close_page`
- Inventory: E would *take* the item. That is a new resource, not more title text
- Animations: pulse/fade on the panel is the same idea as start-screen text, only `run_if` when `OpenInspection` is `Some`

---

## 5. How open/close actually works

`OpenInspection` is the switch.

- `None` — exploring
- `Some(entity)` — this object’s page is open

`use_item.rs` (E):

1. Require focus + a closed page.
2. Set `OpenInspection` to that entity.
3. Show `InspectionPage`.
4. Copy `InspectInfo` into UI texts.
5. Free the mouse (`CursorGrabMode::None`).

`close_page.rs` (Esc):

1. Set `OpenInspection` to `None`.
2. Hide the page.
3. Lock the mouse again.

Look, move, focus, prompt, and crosshair all key off `OpenInspection`. If you add a new FPS system, return early when the page is open.

---

## 6. Patterns to copy

**New field on every inspectable**

```text
InspectInfo { title, body }
        ↓  use_item on E
InspectionTitle / InspectionBody  (Text on the page)
```

**New UI that is the same for every item** (footer, dim backdrop)

- Spawn it in `spawn_page`
- No `InspectInfo` field

**Marker + query** (same as title):

```text
Query<&mut Text, With<InspectionBody>>
```

Fill it in `use_item`. Empty it in `close_page` only if leftover text would flash next time you open.

---

## 7. Do not

- Spawn a new `InspectionPage` on every E (you will stack overlays).
- Put page layout in `scene/setup.rs` or `player/`.
- Drive look/WASD from the page. Freeze them via `OpenInspection` only.
- Make the floor `Interactable` unless you want E on the ground.
- Fight the mouse: while the page is open the OS cursor should stay **visible** so you can click UI later.

---

## 8. Checklist when something “does nothing”

- Did you add `Interactable` **and** `InspectInfo` on the object?
- Does the collider exist? The ray hits colliders, not the mesh picture.
- Is `OpenInspection` still `Some` from last time? Esc first.
- New text: did you add the marker **and** fill it in `use_item`?
- Plugin still registers `spawn_page`, `use_item`, `close_page` in `interactions/mod.rs`.

When you want a body paragraph, image slot, or close button, start with **Step A**.
