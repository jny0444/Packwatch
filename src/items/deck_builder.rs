use bevy::{ecs::query::QueryFilter, prelude::*, window::CursorOptions};

use crate::{
    camera::set_cursor_locked,
    interactions::components::{InspectionPage, OpenInspection},
    items::{
        Inventory, ItemKind, Pocket, ShopPage,
        bag::InventoryPage,
        deck::{DECK_AP, DECK_MAX_CARDS, Deck, PlayerDeck},
        save::save_inventory,
        wallet::Wallet,
    },
    screens::GameState,
};

const BUTTON_IDLE: Color = Color::srgb(0.2, 0.2, 0.22);
const BUTTON_CAN: Color = Color::srgb(0.28, 0.32, 0.28);
const BUTTON_BLOCKED: Color = Color::srgb(0.16, 0.16, 0.17);
const SLOT_PACKED: Color = Color::srgb(0.36, 0.24, 0.16);
const PIP_EMPTY: Color = Color::srgb(0.22, 0.22, 0.24);
const PIP_FILL: Color = Color::srgb(0.85, 0.42, 0.16);

#[derive(Component)]
pub struct DeckPage;

#[derive(Component, Clone, Copy)]
pub(crate) struct DeckSource {
    index: usize,
}

#[derive(Component, Clone, Copy)]
pub(crate) struct DeckCard {
    index: usize,
}

#[derive(Component, Clone, Copy)]
pub(crate) struct DeckPip {
    index: usize,
}

#[derive(Component, Clone, Copy)]
pub(crate) enum DeckField {
    Ap,
    Hint,
}

#[derive(Resource, Default)]
pub(crate) struct DeckUi {
    hint: String,
}

fn page_visible(query: &Query<&Visibility, impl QueryFilter>) -> bool {
    query
        .single()
        .is_ok_and(|visibility| *visibility == Visibility::Visible)
}

pub(crate) fn spawn_deck_page(mut commands: Commands) {
    commands
        .spawn((
            DeckPage,
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
                    flex_direction: FlexDirection::Column,
                    row_gap: px(16),
                    width: px(920),
                    height: percent(82),
                    padding: UiRect::all(px(20)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.12, 0.14)),
            ))
            .with_children(|panel| {
                spawn_header(panel);
                panel
                    .spawn((Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: px(20),
                        width: percent(100),
                        flex_grow: 1.0,
                        min_height: px(0),
                        ..default()
                    },))
                    .with_children(|body| {
                        spawn_source_list(body);
                        spawn_pack_list(body);
                    });
                spawn_footer(panel);
            });
        });
}

fn spawn_header(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: percent(100),
            ..default()
        },))
        .with_children(|header| {
            header.spawn((
                Text::new("PACK"),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            header
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(12),
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|ap| {
                    ap.spawn((
                        DeckField::Ap,
                        Text::new(""),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.85, 0.85, 0.7)),
                    ));
                    ap.spawn((Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: px(6),
                        ..default()
                    },))
                    .with_children(|pips| {
                        for index in 0..DECK_AP {
                            pips.spawn((
                                DeckPip {
                                    index: index as usize,
                                },
                                Node {
                                    width: px(16),
                                    height: px(16),
                                    ..default()
                                },
                                BackgroundColor(PIP_EMPTY),
                            ));
                        }
                    });
                });
        });
}

fn spawn_source_list(body: &mut ChildSpawnerCommands) {
    body.spawn((Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(10),
        width: px(420),
        height: percent(100),
        min_height: px(0),
        ..default()
    },))
    .with_children(|left| {
        left.spawn((
            Text::new("BAG"),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            },
            TextColor(Color::srgb(0.72, 0.72, 0.74)),
        ));
        left.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                width: percent(100),
                flex_grow: 1.0,
                min_height: px(0),
                overflow: Overflow::scroll_y(),
                ..default()
            },
        ))
        .with_children(|list| {
            for index in 0..Pocket::Items.max_slots() {
                spawn_row(list, DeckSource { index }, "");
            }
        });
    });
}

fn spawn_pack_list(body: &mut ChildSpawnerCommands) {
    body.spawn((Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(10),
        flex_grow: 1.0,
        height: percent(100),
        min_height: px(0),
        ..default()
    },))
    .with_children(|right| {
        right.spawn((
            Text::new("THIS ROUND"),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            },
            TextColor(Color::srgb(0.72, 0.72, 0.74)),
        ));
        right
            .spawn((Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                width: percent(100),
                flex_grow: 1.0,
                min_height: px(0),
                ..default()
            },))
            .with_children(|list| {
                for index in 0..DECK_MAX_CARDS {
                    spawn_row(list, DeckCard { index }, "");
                }
            });
    });
}

fn spawn_row(parent: &mut ChildSpawnerCommands, marker: impl Component, label: &'static str) {
    parent
        .spawn((
            Button,
            marker,
            Node {
                width: percent(100),
                padding: UiRect::all(px(12)),
                flex_shrink: 0.0,
                ..default()
            },
            BackgroundColor(BUTTON_IDLE),
        ))
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_footer(panel: &mut ChildSpawnerCommands) {
    panel.spawn((
        DeckField::Hint,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextColor(Color::srgb(0.72, 0.72, 0.74)),
    ));
}

pub(crate) fn deck_interact(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut open: ResMut<OpenInspection>,
    mut ui: ResMut<DeckUi>,
    mut player_deck: ResMut<PlayerDeck>,
    inventory: Res<Inventory>,
    wallet: Res<Wallet>,
    shop: Query<
        &Visibility,
        (
            With<ShopPage>,
            Without<InspectionPage>,
            Without<InventoryPage>,
            Without<DeckPage>,
        ),
    >,
    inspect: Query<
        &Visibility,
        (
            With<InspectionPage>,
            Without<ShopPage>,
            Without<InventoryPage>,
            Without<DeckPage>,
        ),
    >,
    bag: Query<
        &Visibility,
        (
            With<InventoryPage>,
            Without<ShopPage>,
            Without<InspectionPage>,
            Without<DeckPage>,
        ),
    >,
    mut deck: Query<
        &mut Visibility,
        (
            With<DeckPage>,
            Without<ShopPage>,
            Without<InspectionPage>,
            Without<InventoryPage>,
        ),
    >,
    sources: Query<(&Interaction, &DeckSource), Changed<Interaction>>,
    cards: Query<(&Interaction, &DeckCard), Changed<Interaction>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        if page_visible(&shop) || page_visible(&inspect) || page_visible(&bag) {
            return;
        }

        let Ok(mut visibility) = deck.single_mut() else {
            return;
        };

        if *visibility == Visibility::Visible {
            *visibility = Visibility::Hidden;
            open.close();
            set_cursor_locked(&mut cursor_options, true);
            save_inventory(&inventory, &wallet, &player_deck.0);
        } else {
            player_deck.0.clamp_to_inventory(&inventory);
            ui.hint = String::new();
            *visibility = Visibility::Visible;
            open.open_bag();
            set_cursor_locked(&mut cursor_options, false);
        }
        return;
    }

    if !deck
        .single()
        .is_ok_and(|visibility| *visibility == Visibility::Visible)
    {
        return;
    }

    if keyboard.just_pressed(KeyCode::Backspace) || keyboard.just_pressed(KeyCode::Delete) {
        if player_deck.0.pop().is_some() {
            ui.hint = String::new();
            save_inventory(&inventory, &wallet, &player_deck.0);
        }
        return;
    }

    for (interaction, source) in &sources {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Some(kind) = source_kind(&inventory, source.index) else {
            continue;
        };
        match player_deck.0.can_add(kind, Some(&inventory)) {
            Ok(()) => {
                player_deck.0.try_add(kind, Some(&inventory));
                ui.hint = String::new();
                save_inventory(&inventory, &wallet, &player_deck.0);
            }
            Err(reason) => ui.hint = reason.to_string(),
        }
    }

    for (interaction, card) in &cards {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if player_deck.0.remove_at(card.index).is_some() {
            ui.hint = String::new();
            save_inventory(&inventory, &wallet, &player_deck.0);
        }
    }
}

fn source_kind(inventory: &Inventory, index: usize) -> Option<ItemKind> {
    inventory
        .slots(Pocket::Items)
        .get(index)
        .copied()
        .flatten()
        .map(|stack| stack.kind)
}

pub(crate) fn update_deck_visuals(
    inventory: Res<Inventory>,
    player_deck: Res<PlayerDeck>,
    ui: Res<DeckUi>,
    mut sources: Query<(&DeckSource, &Children, &mut BackgroundColor, &mut Node)>,
    mut cards: Query<
        (&DeckCard, &Children, &mut BackgroundColor, &mut Node),
        Without<DeckSource>,
    >,
    mut pips: Query<(&DeckPip, &mut BackgroundColor), (Without<DeckSource>, Without<DeckCard>)>,
    mut fields: Query<(&DeckField, &mut Text)>,
    mut texts: Query<&mut Text, Without<DeckField>>,
) {
    let deck = &player_deck.0;
    let used = deck.ap_used();

    for (source, children, mut color, mut node) in &mut sources {
        let slot = inventory
            .slots(Pocket::Items)
            .get(source.index)
            .copied()
            .flatten();
        match slot {
            Some(stack) => {
                node.display = Display::Flex;
                let packed = deck.count(stack.kind);
                let can = deck.can_add(stack.kind, Some(&inventory)).is_ok();
                *color = BackgroundColor(if can {
                    BUTTON_CAN
                } else {
                    BUTTON_BLOCKED
                });
                let packed_note = if packed > 0 {
                    format!("  ·  packed {packed}")
                } else {
                    String::new()
                };
                let count = if stack.count > 1 {
                    format!("  ×{}", stack.count)
                } else {
                    String::new()
                };
                let label = format!(
                    "{}{count}   AP {}{packed_note}",
                    stack.kind.def().name,
                    Deck::ap_cost(stack.kind)
                );
                if let Some(&child) = children.first()
                    && let Ok(mut text) = texts.get_mut(child)
                {
                    **text = label;
                }
            }
            None => {
                node.display = Display::None;
                *color = BackgroundColor(BUTTON_IDLE);
                if let Some(&child) = children.first()
                    && let Ok(mut text) = texts.get_mut(child)
                {
                    **text = String::new();
                }
            }
        }
    }

    for (card, children, mut color, mut node) in &mut cards {
        let slot = deck.cards().get(card.index).copied();
        let show = (deck.cards().len() + 1).min(DECK_MAX_CARDS);
        node.display = if card.index < show {
            Display::Flex
        } else {
            Display::None
        };
        match slot {
            Some(kind) => {
                *color = BackgroundColor(SLOT_PACKED);
                let label = format!("{}   AP {}", kind.def().name, Deck::ap_cost(kind));
                if let Some(&child) = children.first()
                    && let Ok(mut text) = texts.get_mut(child)
                {
                    **text = label;
                }
            }
            None => {
                *color = BackgroundColor(BUTTON_IDLE);
                if let Some(&child) = children.first()
                    && let Ok(mut text) = texts.get_mut(child)
                {
                    **text = if card.index == deck.cards().len() {
                        "empty slot".to_string()
                    } else {
                        String::new()
                    };
                }
            }
        }
    }

    for (pip, mut color) in &mut pips {
        *color = BackgroundColor(if (pip.index as u32) < used {
            PIP_FILL
        } else {
            PIP_EMPTY
        });
    }

    let default_hint = if inventory.slots(Pocket::Items).iter().all(|slot| slot.is_none()) {
        "Buy smokes at the kiosk, then pack them here."
    } else {
        "Click a bag item to pack it. Click a packed item to pull it. Backspace drops the last."
    };

    for (field, mut text) in &mut fields {
        **text = match field {
            DeckField::Ap => format!("AP  {used} / {DECK_AP}"),
            DeckField::Hint => {
                if ui.hint.is_empty() {
                    default_hint.to_string()
                } else {
                    ui.hint.clone()
                }
            }
        };
    }
}
