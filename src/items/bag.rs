use bevy::{ecs::query::QueryFilter, prelude::*, window::CursorOptions};

use crate::{
    camera::set_cursor_locked,
    interactions::components::{InspectionPage, OpenInspection},
    items::{Inventory, Pocket, ShopPage},
    screens::GameState,
};

const BUTTON_IDLE: Color = Color::srgb(0.2, 0.2, 0.22);
const BUTTON_SELECTED: Color = Color::srgb(0.32, 0.36, 0.42);
const SLOT_COLS: usize = 5;

#[derive(Component)]
pub struct InventoryPage;

#[derive(Component, Clone, Copy)]
pub(crate) struct BagSlot {
    pocket: Pocket,
    index: usize,
}

#[derive(Component, Clone, Copy)]
pub(crate) struct BagTab {
    pocket: Pocket,
}

#[derive(Component, Clone, Copy)]
pub(crate) struct BagPocket {
    pocket: Pocket,
}

#[derive(Component, Clone, Copy)]
pub(crate) enum BagField {
    Name,
    Desc,
}

#[derive(Resource)]
pub(crate) struct BagUi {
    pocket: Pocket,
    selected: usize,
}

impl Default for BagUi {
    fn default() -> Self {
        Self {
            pocket: Pocket::Items,
            selected: 0,
        }
    }
}

impl BagUi {
    fn clamp_selected(&mut self) {
        let max = self.pocket.max_slots().saturating_sub(1);
        self.selected = self.selected.min(max);
    }
}

pub(crate) fn spawn_bag_page(mut commands: Commands) {
    commands
        .spawn((
            InventoryPage,
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
                    column_gap: px(20),
                    width: px(920),
                    height: percent(82),
                    padding: UiRect::all(px(20)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.12, 0.14)),
            ))
            .with_children(|panel| {
                spawn_bag_left(panel);
                spawn_bag_right(panel);
            });
        });
}

fn spawn_bag_left(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(12),
            width: px(540),
            height: percent(100),
            min_height: px(0),
            ..default()
        },))
        .with_children(|left| {
            left.spawn((
                Text::new("BAG"),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            spawn_tabs(left);
            spawn_pocket_block(left, Pocket::Items);
            spawn_pocket_block(left, Pocket::KeyItems);
        });
}

fn spawn_tabs(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            column_gap: px(8),
            ..default()
        },))
        .with_children(|row| {
            spawn_tab(row, Pocket::Items);
            spawn_tab(row, Pocket::KeyItems);
        });
}

fn spawn_tab(parent: &mut ChildSpawnerCommands, pocket: Pocket) {
    parent
        .spawn((
            Button,
            BagTab { pocket },
            Node {
                padding: UiRect::axes(px(12), px(8)),
                ..default()
            },
            BackgroundColor(BUTTON_IDLE),
        ))
        .with_children(|tab| {
            tab.spawn((
                Text::new(pocket.label()),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_pocket_block(parent: &mut ChildSpawnerCommands, pocket: Pocket) {
    let cols = SLOT_COLS;
    let rows = pocket.max_slots().div_ceil(cols);
    parent
        .spawn((
            BagPocket { pocket },
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                width: percent(100),
                flex_grow: 1.0,
                min_height: px(0),
                display: if pocket == Pocket::Items {
                    Display::Flex
                } else {
                    Display::None
                },
                ..default()
            },
            Visibility::Inherited,
        ))
        .with_children(|block| {
            for row in 0..rows {
                block
                    .spawn((Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: px(8),
                        width: percent(100),
                        flex_grow: 1.0,
                        min_height: px(0),
                        ..default()
                    },))
                    .with_children(|line| {
                        for col in 0..cols {
                            let index = row * cols + col;
                            if index < pocket.max_slots() {
                                spawn_slot(line, pocket, index);
                            }
                        }
                    });
            }
        });
}

fn spawn_slot(parent: &mut ChildSpawnerCommands, pocket: Pocket, index: usize) {
    parent
        .spawn((
            Button,
            BagSlot { pocket, index },
            Node {
                width: percent(100),
                height: percent(100),
                padding: UiRect::all(px(8)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_grow: 1.0,
                min_height: px(0),
                ..default()
            },
            BackgroundColor(BUTTON_IDLE),
        ))
        .with_children(|slot| {
            slot.spawn((
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_bag_right(panel: &mut ChildSpawnerCommands) {
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
                BagField::Name,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            right.spawn((
                BagField::Desc,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgb(0.72, 0.72, 0.74)),
            ));
        });
}

fn page_visible(query: &Query<&Visibility, impl QueryFilter>) -> bool {
    query
        .single()
        .is_ok_and(|visibility| *visibility == Visibility::Visible)
}

pub(crate) fn bag_interact(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut open: ResMut<OpenInspection>,
    mut ui: ResMut<BagUi>,
    shop: Query<
        &Visibility,
        (
            With<ShopPage>,
            Without<InspectionPage>,
            Without<InventoryPage>,
        ),
    >,
    inspect: Query<
        &Visibility,
        (
            With<InspectionPage>,
            Without<ShopPage>,
            Without<InventoryPage>,
        ),
    >,
    mut bag: Query<
        &mut Visibility,
        (
            With<InventoryPage>,
            Without<ShopPage>,
            Without<InspectionPage>,
        ),
    >,
    tabs: Query<(&Interaction, &BagTab), Changed<Interaction>>,
    slots: Query<(&Interaction, &BagSlot), Changed<Interaction>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        if page_visible(&shop) || page_visible(&inspect) {
            return;
        }

        let Ok(mut visibility) = bag.single_mut() else {
            return;
        };

        if *visibility == Visibility::Visible {
            *visibility = Visibility::Hidden;
            open.close();
            set_cursor_locked(&mut cursor_options, true);
        } else {
            *visibility = Visibility::Visible;
            open.open_bag();
            set_cursor_locked(&mut cursor_options, false);
        }
        return;
    }

    if !bag
        .single()
        .is_ok_and(|visibility| *visibility == Visibility::Visible)
    {
        return;
    }

    for (interaction, tab) in &tabs {
        if *interaction == Interaction::Pressed {
            ui.pocket = tab.pocket;
            ui.selected = 0;
            ui.clamp_selected();
        }
    }

    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed {
            ui.pocket = slot.pocket;
            ui.selected = slot.index;
            ui.clamp_selected();
        }
    }

    let max = ui.pocket.max_slots();
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        ui.selected = (ui.selected + 1).min(max - 1);
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        ui.selected = ui.selected.saturating_sub(1);
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        ui.selected = (ui.selected + SLOT_COLS).min(max - 1);
    }
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        ui.selected = ui.selected.saturating_sub(SLOT_COLS);
    }
}

pub(crate) fn update_bag_visuals(
    inventory: Res<Inventory>,
    ui: Res<BagUi>,
    mut slots: Query<(&BagSlot, &Children, &mut BackgroundColor)>,
    mut tabs: Query<(&BagTab, &mut BackgroundColor), Without<BagSlot>>,
    mut fields: Query<(&BagField, &mut Text)>,
    mut texts: Query<&mut Text, Without<BagField>>,
    mut pockets: Query<(&BagPocket, &mut Node), Without<InventoryPage>>,
) {
    for (tab, mut color) in &mut tabs {
        *color = BackgroundColor(if tab.pocket == ui.pocket {
            BUTTON_SELECTED
        } else {
            BUTTON_IDLE
        });
    }

    for (pocket, mut node) in &mut pockets {
        node.display = if pocket.pocket == ui.pocket {
            Display::Flex
        } else {
            Display::None
        };
    }

    for (slot, children, mut color) in &mut slots {
        *color = BackgroundColor(if slot.pocket == ui.pocket && slot.index == ui.selected {
            BUTTON_SELECTED
        } else {
            BUTTON_IDLE
        });

        let label = match inventory
            .slots(slot.pocket)
            .get(slot.index)
            .copied()
            .flatten()
        {
            Some(stack) if stack.count > 1 => {
                format!("{}\n×{}", stack.kind.def().name, stack.count)
            }
            Some(stack) => stack.kind.def().name.to_string(),
            None => String::new(),
        };

        if let Some(&child) = children.first()
            && let Ok(mut text) = texts.get_mut(child)
        {
            **text = label;
        }
    }

    let selected = inventory
        .slots(ui.pocket)
        .get(ui.selected)
        .copied()
        .flatten();
    for (field, mut text) in &mut fields {
        **text = match (field, selected) {
            (BagField::Name, Some(stack)) => stack.kind.def().name.to_string(),
            (BagField::Desc, Some(stack)) => stack.kind.def().description.to_string(),
            (BagField::Name, None) => String::new(),
            (BagField::Desc, None) => String::new(),
        };
    }
}
