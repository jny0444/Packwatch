use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    asset::RenderAssetUsages,
    camera::{RenderTarget, visibility::RenderLayers},
    input::mouse::MouseMotion,
    math::Affine2,
    prelude::*,
    render::render_resource::{TextureDimension, TextureFormat, TextureUsages},
    ui::widget::ViewportNode,
    world_serialization::WorldInstanceReady,
};

use crate::{
    items::{
        Deck, Inventory, ItemKind, PlayerDeck,
        hud::WalletHud,
        types::{BeerTypes, CigTypes, GumTypes},
        wallet::Wallet,
    },
    screens::GameState,
    templates::FixGltfAlpha,
};

const BUTTON_IDLE: Color = Color::srgb(0.2, 0.2, 0.22);
const BUTTON_SELECTED: Color = Color::srgb(0.32, 0.36, 0.42);
const BUY_IDLE: Color = Color::srgb(0.22, 0.38, 0.28);
const BUY_FULL: Color = Color::srgb(0.28, 0.18, 0.18);
const FLASH_SECS: f32 = 0.45;
const ROW_GAP: f32 = 8.0;
const PREVIEW_LAYER: usize = 1;
const PREVIEW_POS: Vec3 = Vec3::new(0.0, -200.0, 0.0);
const PREVIEW_SPIN_SPEED: f32 = 0.7;
const PREVIEW_DRAG_SPEED: f32 = 0.008;

#[derive(Clone, Copy)]
pub struct ShopListing {
    pub kind: ItemKind,
    pub price: u32,
}

const fn listing(kind: ItemKind, price: u32) -> ShopListing {
    ShopListing { kind, price }
}

pub const KIOSK_STOCK: &[ShopListing] = &[
    listing(ItemKind::Lighter, 10),
    listing(ItemKind::Cig(CigTypes::MarlboroGold), 25),
    listing(ItemKind::Cig(CigTypes::DoubleHappiness11mg), 30),
    listing(ItemKind::Cig(CigTypes::StellarDoubleShift), 22),
    listing(ItemKind::Cig(CigTypes::ClassicIndieMint), 15),
    listing(ItemKind::Cig(CigTypes::CamelYellow), 28),
    listing(ItemKind::Cig(CigTypes::Cashtri), 12),
    listing(ItemKind::Cig(CigTypes::Mond), 18),
    listing(ItemKind::Beer(BeerTypes::BudweiserMagnum), 40),
    listing(ItemKind::Beer(BeerTypes::KingfisherStrong), 40),
    listing(ItemKind::Beer(BeerTypes::Corona), 40),
    listing(ItemKind::Beer(BeerTypes::Guinness), 40),
    listing(ItemKind::Gum(GumTypes::MintStrongGum), 15),
    listing(ItemKind::Gum(GumTypes::LightGum), 10),
];

#[derive(Component)]
pub struct ShopPage;

#[derive(Component)]
pub(crate) struct ShopList;

#[derive(Component, Clone, Copy)]
pub(crate) struct ShopRow {
    index: usize,
}

#[derive(Component)]
pub(crate) struct ShopBuyConfirm;

#[derive(Component, Clone, Copy)]
pub(crate) enum ShopQtyButton {
    Minus,
    Plus,
}

#[derive(Component, Clone, Copy)]
pub(crate) enum ShopField {
    Name,
    Desc,
    Price,
    Qty,
}

#[derive(Component)]
pub(crate) struct ShopConfirm;

#[derive(Component)]
pub(crate) struct PurchasePulse {
    remaining: f32,
}

#[derive(Component)]
pub(crate) struct ShopPreviewCamera;

#[derive(Component)]
pub(crate) struct ShopPreviewStage;

#[derive(Component)]
pub(crate) struct ShopPreviewViewport;

#[derive(Component)]
pub(crate) struct ShopPreviewModel(ItemKind);

#[derive(Component)]
pub(crate) struct ShopDropdownToggle;

#[derive(Component)]
pub(crate) struct ShopDropdownLabel;

#[derive(Component)]
pub(crate) struct ShopDropdownChevron;

#[derive(Component)]
pub(crate) struct ShopDropdownMenu;

#[derive(Resource)]
pub(crate) struct ShopUi {
    selected: usize,
    qty: u32,
    dropdown_open: bool,
}

#[derive(Resource, Default)]
pub(crate) struct SpendFlash {
    remaining: f32,
    item_name: String,
}

impl Default for ShopUi {
    fn default() -> Self {
        Self {
            selected: 0,
            qty: 1,
            dropdown_open: false,
        }
    }
}

impl ShopUi {
    fn listing(&self) -> ShopListing {
        KIOSK_STOCK[self.selected]
    }

    fn max_qty(&self, inventory: &Inventory) -> u32 {
        let cap = self.listing().kind.def().max_stack.min(99);
        cap.min(inventory.space_for(self.listing().kind))
    }

    fn clamp_qty(&mut self, inventory: &Inventory) {
        let max = self.max_qty(inventory);
        self.qty = if max == 0 { 1 } else { self.qty.clamp(1, max) };
    }
}

fn preview_transform(kind: ItemKind) -> Transform {
    // `center` cancels out a glTF origin that sits away from the mesh, so it has
    // to be rotated alongside the model to keep the mesh on the stage pivot.
    let (scale, center, rotation) = match kind {
        // Blender packs are ~22×6×34 and already centered. The old cig pose
        // scaled the tiny Sketchfab pack by 10.5, which put the camera inside
        // these. Stand the pack up (X 90°) and pull back with a 3/4 yaw.
        ItemKind::Cig(CigTypes::Cashtri) => {
            (0.24, Vec3::ZERO, Quat::from_rotation_y(-0.55))
        }
        ItemKind::Cig(_) if kind.resolved_model().starts_with("customASSets/") => (
            0.03,
            Vec3::ZERO,
            Quat::from_rotation_y(-0.5) * Quat::from_rotation_x(FRAC_PI_2),
        ),
        ItemKind::Cig(_) => (
            10.5,
            Vec3::new(1.14, -6.22, -1.22),
            Quat::from_rotation_x(FRAC_PI_2),
        ),
        ItemKind::Beer(_) => (0.39, Vec3::new(0.0, -0.06, 0.0), Quat::IDENTITY),
        ItemKind::Gum(_) => (0.36, Vec3::ZERO, Quat::IDENTITY),
        ItemKind::Lighter => (2.8, Vec3::ZERO, Quat::IDENTITY),
    };

    Transform {
        translation: rotation * center,
        rotation,
        scale: Vec3::splat(scale),
    }
}

fn shop_visible(shop: &Query<&Visibility, With<ShopPage>>) -> bool {
    shop.single()
        .is_ok_and(|visibility| *visibility == Visibility::Visible)
}

pub(crate) fn spawn_shop_page(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let camera = spawn_preview_world(&mut commands, &mut images, &asset_server);
    spawn_shop_ui(&mut commands, camera);
}

fn spawn_preview_world(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    asset_server: &AssetServer,
) -> Entity {
    let mut image = Image::new_uninit(
        default(),
        TextureDimension::D2,
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::all(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    let image_handle = images.add(image);

    let layers = RenderLayers::layer(PREVIEW_LAYER);

    let camera = commands
        .spawn((
            ShopPreviewCamera,
            DespawnOnExit(GameState::Playing),
            Camera3d::default(),
            Camera {
                order: -1,
                is_active: false,
                clear_color: ClearColorConfig::Custom(Color::srgb(0.08, 0.08, 0.1)),
                ..default()
            },
            RenderTarget::Image(image_handle.into()),
            Transform::from_translation(PREVIEW_POS + Vec3::new(0.0, 0.2, 1.6))
                .looking_at(PREVIEW_POS, Vec3::Y),
            layers.clone(),
        ))
        .id();

    commands
        .spawn((
            ShopPreviewStage,
            DespawnOnExit(GameState::Playing),
            Transform::from_translation(PREVIEW_POS),
            Visibility::Visible,
            layers.clone(),
        ))
        .with_children(|stage| {
            stage.spawn((
                PointLight {
                    intensity: 120_000.0,
                    range: 8.0,
                    shadow_maps_enabled: false,
                    ..default()
                },
                Transform::from_xyz(1.2, 1.6, 2.0),
                layers.clone(),
            ));

            spawn_preview_model(stage, asset_server, layers.clone(), KIOSK_STOCK[0].kind);
        });

    camera
}

fn preview_root(asset_server: &AssetServer, kind: ItemKind) -> WorldAssetRoot {
    WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(kind.resolved_model())))
}

fn spawn_preview_model(
    stage: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    layers: RenderLayers,
    kind: ItemKind,
) {
    stage.spawn((
        ShopPreviewModel(kind),
        FixGltfAlpha,
        preview_root(asset_server, kind),
        preview_transform(kind),
        Visibility::Inherited,
        layers,
    ));
}

fn preview_uv_angle(kind: ItemKind) -> Option<f32> {
    if matches!(kind, ItemKind::Cig(CigTypes::Cashtri)) {
        return None;
    }
    kind.resolved_model()
        .starts_with("customASSets/")
        .then_some(FRAC_PI_2 + PI)
}

fn uv_rotate(angle: f32) -> Affine2 {
    Affine2::from_translation(Vec2::splat(0.5))
        * Affine2::from_angle_translation(angle, Vec2::ZERO)
        * Affine2::from_translation(Vec2::splat(-0.5))
}

pub(crate) fn apply_preview_uv(
    ready: On<WorldInstanceReady>,
    preview: Query<&ShopPreviewModel>,
    children: Query<&Children>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(model) = preview.get(ready.entity) else {
        return;
    };
    if !model.0.resolved_model().starts_with("customASSets/") {
        return;
    }
    let uv = preview_uv_angle(model.0)
        .map(uv_rotate)
        .unwrap_or(Affine2::IDENTITY);

    for entity in children.iter_descendants(ready.entity) {
        let Ok(handle) = mesh_materials.get(entity) else {
            continue;
        };
        let Some(mut material) = materials.get_mut(handle.id()) else {
            continue;
        };
        material.uv_transform = uv;
    }
}

fn spawn_shop_ui(commands: &mut Commands, camera: Entity) {
    commands
        .spawn((
            ShopPage,
            DespawnOnExit(GameState::Playing),
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Visibility::Hidden,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(24),
                    width: px(860),
                    height: percent(78),
                    padding: UiRect::all(px(24)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.09, 0.09, 0.1)),
            ))
            .with_children(|panel| {
                spawn_item_list(panel);
                spawn_detail_pane(panel, camera);
            });
        });
}

fn spawn_item_list(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(16),
            width: px(280),
            height: percent(100),
            ..default()
        },))
        .with_children(|left| {
            left.spawn((
                Text::new("Shop"),
                TextFont {
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            left.spawn((
                Text::new("Stock"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.55, 0.55, 0.58)),
            ));
            spawn_dropdown(left);
        });
}

fn spawn_dropdown(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            width: percent(100),
            ..default()
        },))
        .with_children(|wrap| {
            wrap.spawn((
                Button,
                ShopDropdownToggle,
                Node {
                    width: percent(100),
                    padding: UiRect::axes(px(14), px(12)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    column_gap: px(8),
                    border: UiRect::all(px(1)),
                    ..default()
                },
                BackgroundColor(BUTTON_IDLE),
                BorderColor::all(Color::srgb(0.28, 0.28, 0.3)),
            ))
            .with_children(|toggle| {
                toggle.spawn((
                    ShopDropdownLabel,
                    Text::new(""),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                toggle.spawn((
                    ShopDropdownChevron,
                    Text::new("▾"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.72)),
                ));
            });

            wrap.spawn((
                ShopList,
                ShopDropdownMenu,
                Node {
                    position_type: PositionType::Absolute,
                    top: px(48),
                    left: px(0),
                    width: percent(100),
                    max_height: px(320),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(ROW_GAP),
                    padding: UiRect::all(px(6)),
                    overflow: Overflow::scroll_y(),
                    border: UiRect::all(px(1)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.14, 0.14, 0.16)),
                BorderColor::all(Color::srgb(0.28, 0.28, 0.3)),
                GlobalZIndex(40),
                ScrollPosition::default(),
                Visibility::Hidden,
            ))
            .with_children(|list| {
                for (index, listing) in KIOSK_STOCK.iter().enumerate() {
                    list.spawn((
                        Button,
                        ShopRow { index },
                        Node {
                            width: percent(100),
                            padding: UiRect::axes(px(12), px(10)),
                            flex_shrink: 0.0,
                            ..default()
                        },
                        BackgroundColor(BUTTON_IDLE),
                    ))
                    .with_children(|row| {
                        row.spawn((
                            Text::new(format!("{}  ${}", listing.kind.def().name, listing.price)),
                            TextFont {
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                }
            });
        });
}

fn spawn_detail_pane(panel: &mut ChildSpawnerCommands, camera: Entity) {
    panel
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(12),
            flex_grow: 1.0,
            height: percent(100),
            ..default()
        },))
        .with_children(|right| {
            right.spawn((
                Node {
                    width: percent(100),
                    height: px(300),
                    border: UiRect::all(px(1)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.22, 0.22, 0.24)),
                ViewportNode::new(camera),
                ShopPreviewViewport,
                Interaction::default(),
            ));
            right.spawn((
                ShopField::Name,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            right.spawn((
                ShopField::Desc,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgb(0.72, 0.72, 0.74)),
            ));
            right.spawn((
                ShopField::Price,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.7)),
            ));
            spawn_qty_row(right);
            right
                .spawn((
                    Button,
                    ShopBuyConfirm,
                    Node {
                        width: percent(100),
                        padding: UiRect::all(px(14)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(BUY_IDLE),
                    UiTransform::IDENTITY,
                ))
                .with_children(|buy| {
                    buy.spawn((
                        Text::new("Buy"),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            right.spawn((
                ShopConfirm,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgba(0.45, 0.85, 0.55, 0.0)),
            ));
        });
}

fn spawn_qty_row(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            column_gap: px(12),
            align_items: AlignItems::Center,
            ..default()
        },))
        .with_children(|row| {
            spawn_qty_button(row, ShopQtyButton::Minus, "−");
            row.spawn((
                ShopField::Qty,
                Text::new("1"),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            spawn_qty_button(row, ShopQtyButton::Plus, "+");
        });
}

fn spawn_qty_button(parent: &mut ChildSpawnerCommands, kind: ShopQtyButton, label: &'static str) {
    parent
        .spawn((
            Button,
            kind,
            Node {
                width: px(36),
                height: px(36),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BUTTON_IDLE),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub(crate) fn shop_interact(
    shop: Query<&Visibility, With<ShopPage>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    rows: Query<(&Interaction, &ShopRow), Changed<Interaction>>,
    qty_buttons: Query<(&Interaction, &ShopQtyButton), Changed<Interaction>>,
    buy_buttons: Query<(Entity, &Interaction), (Changed<Interaction>, With<ShopBuyConfirm>)>,
    buy_entities: Query<Entity, With<ShopBuyConfirm>>,
    toggle: Query<&Interaction, (Changed<Interaction>, With<ShopDropdownToggle>)>,
    mut ui: ResMut<ShopUi>,
    mut wallet: ResMut<Wallet>,
    mut inventory: ResMut<Inventory>,
    mut flash: ResMut<SpendFlash>,
    deck: Res<PlayerDeck>,
    mut commands: Commands,
) {
    if !shop_visible(&shop) {
        ui.dropdown_open = false;
        return;
    }

    let count = KIOSK_STOCK.len();
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        ui.selected = (ui.selected + 1) % count;
        ui.clamp_qty(&inventory);
    }
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        ui.selected = (ui.selected + count - 1) % count;
        ui.clamp_qty(&inventory);
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        ui.qty = (ui.qty + 1).min(ui.max_qty(&inventory).max(1));
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        ui.qty = ui.qty.saturating_sub(1).max(1);
    }

    for interaction in &toggle {
        if *interaction == Interaction::Pressed {
            ui.dropdown_open = !ui.dropdown_open;
        }
    }

    for (interaction, row) in &rows {
        if *interaction == Interaction::Pressed {
            ui.selected = row.index;
            ui.clamp_qty(&inventory);
            ui.dropdown_open = false;
        }
    }

    for (interaction, button) in &qty_buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button {
            ShopQtyButton::Minus => ui.qty = ui.qty.saturating_sub(1).max(1),
            ShopQtyButton::Plus => ui.qty = (ui.qty + 1).min(ui.max_qty(&inventory).max(1)),
        }
    }

    let buy_clicked = buy_buttons.iter().any(|(entity, interaction)| {
        if *interaction == Interaction::Pressed {
            try_buy(
                &mut ui,
                &mut wallet,
                &mut inventory,
                &mut flash,
                &deck.0,
                &mut commands,
                entity,
            );
            true
        } else {
            false
        }
    });

    if !buy_clicked
        && (keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter))
        && let Ok(entity) = buy_entities.single()
    {
        try_buy(
            &mut ui,
            &mut wallet,
            &mut inventory,
            &mut flash,
            &deck.0,
            &mut commands,
            entity,
        );
    }
}

fn try_buy(
    ui: &mut ShopUi,
    wallet: &mut Wallet,
    inventory: &mut Inventory,
    flash: &mut SpendFlash,
    deck: &Deck,
    commands: &mut Commands,
    buy_entity: Entity,
) {
    let listing = ui.listing();
    let qty = ui.qty;
    if !inventory.can_add(listing.kind, qty) {
        return;
    }
    let cost = listing.price.saturating_mul(qty);
    if !wallet.spend(cost) {
        return;
    }
    if !inventory.add(listing.kind, qty) {
        wallet.add(cost);
        return;
    }
    ui.clamp_qty(inventory);

    flash.remaining = FLASH_SECS;
    flash.item_name = if qty == 1 {
        listing.kind.def().name.to_string()
    } else {
        format!("{} × {}", listing.kind.def().name, qty)
    };
    commands.entity(buy_entity).insert(PurchasePulse {
        remaining: FLASH_SECS,
    });
    crate::items::save::save_inventory(inventory, wallet, deck);
}

pub(crate) fn update_shop_visuals(
    shop: Query<&Visibility, With<ShopPage>>,
    ui: Res<ShopUi>,
    inventory: Res<Inventory>,
    mut rows: Query<(&ShopRow, &mut BackgroundColor, &ComputedNode), Without<ShopBuyConfirm>>,
    mut fields: Query<(&ShopField, &mut Text)>,
    mut labels: Query<&mut Text, (With<ShopDropdownLabel>, Without<ShopField>)>,
    mut chevrons: Query<
        &mut Text,
        (
            With<ShopDropdownChevron>,
            Without<ShopField>,
            Without<ShopDropdownLabel>,
        ),
    >,
    mut menus: Query<&mut Visibility, (With<ShopDropdownMenu>, Without<ShopPage>)>,
    mut lists: Query<(&mut ScrollPosition, &ComputedNode), With<ShopList>>,
    mut buy: Query<&mut BackgroundColor, (With<ShopBuyConfirm>, Without<PurchasePulse>)>,
    mut toggles: Query<
        &mut BackgroundColor,
        (
            With<ShopDropdownToggle>,
            Without<ShopBuyConfirm>,
            Without<ShopRow>,
            Without<PurchasePulse>,
        ),
    >,
) {
    if !shop_visible(&shop) {
        return;
    }

    let listing = ui.listing();
    let def = listing.kind.def();
    let total = listing.price.saturating_mul(ui.qty);

    if let Ok(mut label) = labels.single_mut() {
        **label = format!("{}  ${}", def.name, listing.price);
    }
    if let Ok(mut chevron) = chevrons.single_mut() {
        **chevron = if ui.dropdown_open { "▴" } else { "▾" }.into();
    }
    if let Ok(mut menu) = menus.single_mut() {
        *menu = if ui.dropdown_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    let mut row_top = 0.0;
    let mut row_height = 0.0;
    for (row, mut color, row_node) in &mut rows {
        *color = BackgroundColor(if row.index == ui.selected {
            BUTTON_SELECTED
        } else {
            BUTTON_IDLE
        });
        let height = row_node.size().y * row_node.inverse_scale_factor();
        if row.index < ui.selected {
            row_top += height + ROW_GAP;
        } else if row.index == ui.selected {
            row_height = height;
        }
    }

    for (field, mut text) in &mut fields {
        **text = match field {
            ShopField::Name => def.name.to_string(),
            ShopField::Desc => def.description.to_string(),
            ShopField::Price => format!("${} each   ·   ${total} total", listing.price),
            ShopField::Qty => format!("{}", ui.qty),
        };
    }

    if let Ok(mut color) = toggles.single_mut() {
        *color = BackgroundColor(if ui.dropdown_open {
            BUTTON_SELECTED
        } else {
            BUTTON_IDLE
        });
    }

    if let Ok(mut color) = buy.single_mut() {
        *color = BackgroundColor(if inventory.can_add(listing.kind, ui.qty) {
            BUY_IDLE
        } else {
            BUY_FULL
        });
    }

    if ui.dropdown_open && let Ok((mut scroll, node)) = lists.single_mut() {
        let view = node.size().y * node.inverse_scale_factor();
        let content = node.content_size().y * node.inverse_scale_factor();
        let max_scroll = (content - view).max(0.0);
        let row_bottom = row_top + row_height;
        if row_top < scroll.y {
            scroll.y = row_top;
        } else if row_bottom > scroll.y + view {
            scroll.y = row_bottom - view;
        }
        scroll.y = scroll.y.clamp(0.0, max_scroll);
    }
}

pub(crate) fn sync_shop_preview(
    shop: Query<&Visibility, With<ShopPage>>,
    ui: Res<ShopUi>,
    asset_server: Res<AssetServer>,
    mut cameras: Query<&mut Camera, With<ShopPreviewCamera>>,
    mut preview: Query<(&mut ShopPreviewModel, &mut Transform, &mut WorldAssetRoot)>,
    mut stages: Query<&mut Transform, (With<ShopPreviewStage>, Without<ShopPreviewModel>)>,
) {
    let open = shop_visible(&shop);
    if let Ok(mut camera) = cameras.single_mut() {
        camera.is_active = open;
    }
    if !open {
        return;
    }

    let kind = ui.listing().kind;
    let Ok((mut model, mut transform, mut root)) = preview.single_mut() else {
        return;
    };
    if model.0 == kind {
        return;
    }

    model.0 = kind;
    *transform = preview_transform(kind);

    // A new item shouldn't inherit the angle the player dragged the last one to.
    for mut stage_transform in &mut stages {
        stage_transform.rotation = Quat::IDENTITY;
    }

    let next = preview_root(&asset_server, kind);
    if root.0 != next.0 {
        *root = next;
    }
}

/// Left-drag anywhere over the preview to spin the model by hand. Bevy keeps a
/// node `Pressed` after the cursor leaves it, so the grab survives fast drags.
pub(crate) fn drag_preview(
    shop: Query<&Visibility, With<ShopPage>>,
    viewport: Query<&Interaction, With<ShopPreviewViewport>>,
    mut motion: MessageReader<MouseMotion>,
    mut stages: Query<&mut Transform, With<ShopPreviewStage>>,
) {
    let dragging = shop_visible(&shop)
        && viewport
            .single()
            .is_ok_and(|interaction| *interaction == Interaction::Pressed);

    if !dragging {
        motion.clear();
        return;
    }

    let delta: Vec2 = motion.read().map(|motion| motion.delta).sum();
    if delta == Vec2::ZERO {
        return;
    }

    for mut transform in &mut stages {
        transform.rotate_y(delta.x * PREVIEW_DRAG_SPEED);
        transform.rotate_x(delta.y * PREVIEW_DRAG_SPEED);
    }
}

pub(crate) fn rotate_preview(
    shop: Query<&Visibility, With<ShopPage>>,
    viewport: Query<&Interaction, With<ShopPreviewViewport>>,
    time: Res<Time>,
    mut stages: Query<&mut Transform, With<ShopPreviewStage>>,
) {
    if !shop_visible(&shop) {
        return;
    }
    let dragging = viewport
        .single()
        .is_ok_and(|interaction| *interaction == Interaction::Pressed);
    if dragging {
        return;
    }
    for mut transform in &mut stages {
        transform.rotate_y(time.delta_secs() * PREVIEW_SPIN_SPEED);
    }
}

pub(crate) fn sync_preview_layers(
    roots: Query<Entity, With<ShopPreviewStage>>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    let layers = RenderLayers::layer(PREVIEW_LAYER);
    for root in &roots {
        apply_preview_layer(root, &layers, &children, &mut commands);
    }
}

fn apply_preview_layer(
    entity: Entity,
    layers: &RenderLayers,
    children: &Query<&Children>,
    commands: &mut Commands,
) {
    commands.entity(entity).insert(layers.clone());
    let Ok(kids) = children.get(entity) else {
        return;
    };
    for child in kids {
        apply_preview_layer(*child, layers, children, commands);
    }
}

pub(crate) fn animate_purchase(
    time: Res<Time>,
    mut flash: ResMut<SpendFlash>,
    mut commands: Commands,
    mut buttons: Query<
        (
            Entity,
            &mut PurchasePulse,
            &mut BackgroundColor,
            &mut UiTransform,
        ),
        With<ShopBuyConfirm>,
    >,
    mut confirm: Query<(&mut Text, &mut TextColor), With<ShopConfirm>>,
    mut wallet_hud: Query<&mut TextColor, (With<WalletHud>, Without<ShopConfirm>)>,
) {
    let dt = time.delta_secs();

    for (entity, mut pulse, mut color, mut transform) in &mut buttons {
        pulse.remaining -= dt;
        let t = (pulse.remaining / FLASH_SECS).clamp(0.0, 1.0);
        *color = BackgroundColor(Color::srgb(0.22, 0.38 + 0.22 * t, 0.28 + 0.08 * t));
        transform.scale = Vec2::splat(1.0 + 0.04 * t);

        if pulse.remaining <= 0.0 {
            *color = BackgroundColor(BUY_IDLE);
            transform.scale = Vec2::ONE;
            commands.entity(entity).remove::<PurchasePulse>();
        }
    }

    if flash.remaining > 0.0 {
        flash.remaining = (flash.remaining - dt).max(0.0);
    }
    let t = (flash.remaining / FLASH_SECS).clamp(0.0, 1.0);

    if let Ok(mut color) = wallet_hud.single_mut() {
        *color = TextColor(Color::srgb(1.0 - 0.4 * t, 1.0, 1.0 - 0.35 * t));
    }

    if let Ok((mut text, mut color)) = confirm.single_mut() {
        if t > 0.0 {
            **text = format!("Purchased {}", flash.item_name);
            *color = TextColor(Color::srgba(0.45, 0.85, 0.55, t));
        } else if !text.is_empty() {
            **text = String::new();
            *color = TextColor(Color::srgba(0.45, 0.85, 0.55, 0.0));
        }
    }
}
