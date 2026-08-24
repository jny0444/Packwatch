use bevy::prelude::*;

use crate::{
    combat::state::{BattleMenu, MatchPhase, MatchSession},
    items::{DECK_MAX_CARDS, Deck, ItemKind},
    screens::PlayMode,
};

const CREAM: Color = Color::srgb(0.98, 0.95, 0.82);
const INK: Color = Color::srgb(0.16, 0.18, 0.2);
const TEAL: Color = Color::srgb(0.16, 0.36, 0.55);
const TEAL_DARK: Color = Color::srgb(0.1, 0.24, 0.4);
const BORDER: Color = Color::srgb(0.22, 0.22, 0.25);
const SELECT: Color = Color::srgb(0.86, 0.28, 0.2);
const IDLE: Color = Color::srgb(0.93, 0.9, 0.78);
const BAR_BACK: Color = Color::srgb(0.18, 0.18, 0.2);

#[derive(Component)]
pub struct MatchPage;

#[derive(Component, Clone, Copy)]
pub(crate) enum BattleText {
    EnemyName,
    PlayerName,
    EnemyDizzy,
    PlayerDizzy,
    Message,
}

#[derive(Component, Clone, Copy)]
pub(crate) enum DizzyBar {
    Enemy,
    Player,
}

#[derive(Component, Clone, Copy)]
pub(crate) enum BattlePanel {
    Command,
    Moves,
}

#[derive(Component, Clone, Copy)]
pub struct CommandButton {
    pub index: usize,
}

#[derive(Component, Clone, Copy)]
pub struct MoveButton {
    pub index: usize,
}

#[derive(Component)]
pub struct BackButton;

pub(crate) fn spawn_match_ui(mut commands: Commands) {
    commands
        .spawn((
            MatchPage,
            DespawnOnExit(PlayMode::Match),
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(px(18)),
                ..default()
            },
        ))
        .with_children(|root| {
            spawn_enemy_hud(root);
            spawn_bottom(root);
        });
}

fn spawn_enemy_hud(root: &mut ChildSpawnerCommands) {
    root.spawn((Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::FlexStart,
        width: percent(100),
        ..default()
    },))
        .with_children(|row| {
            status_box(row, true);
        });
}

fn spawn_bottom(root: &mut ChildSpawnerCommands) {
    root.spawn((Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(10),
        width: percent(100),
        ..default()
    },))
        .with_children(|bottom| {
            bottom
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexEnd,
                    width: percent(100),
                    ..default()
                },))
                .with_children(|row| {
                    status_box(row, false);
                });

            bottom
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        width: percent(100),
                        min_height: px(168),
                        border: UiRect::all(px(4)),
                        ..default()
                    },
                    BackgroundColor(TEAL),
                    BorderColor::all(BORDER),
                ))
                .with_children(|bar| {
                    bar.spawn((
                        Node {
                            width: percent(58),
                            height: percent(100),
                            padding: UiRect::all(px(16)),
                            border: UiRect::all(px(3)),
                            ..default()
                        },
                        BackgroundColor(CREAM),
                        BorderColor::all(Color::srgb(0.45, 0.42, 0.3)),
                    ))
                    .with_children(|msg| {
                        msg.spawn((
                            BattleText::Message,
                            Text::new(""),
                            TextFont {
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                            TextColor(INK),
                        ));
                    });

                    bar.spawn((
                        Node {
                            width: percent(42),
                            height: percent(100),
                            padding: UiRect::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(TEAL_DARK),
                    ))
                    .with_children(|right| {
                        spawn_command_panel(right);
                        spawn_moves_panel(right);
                    });
                });
        });
}

fn status_box(parent: &mut ChildSpawnerCommands, enemy: bool) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                width: px(340),
                padding: UiRect::all(px(12)),
                border: UiRect::all(px(4)),
                ..default()
            },
            BackgroundColor(CREAM),
            BorderColor::all(BORDER),
        ))
        .with_children(|box_| {
            box_.spawn((
                if enemy {
                    BattleText::EnemyName
                } else {
                    BattleText::PlayerName
                },
                Text::new(if enemy { "" } else { "YOU" }),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(INK),
            ));

            box_.spawn((
                Node {
                    width: percent(100),
                    height: px(14),
                    padding: UiRect::all(px(2)),
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(BAR_BACK),
                BorderColor::all(INK),
            ))
            .with_children(|track| {
                track.spawn((
                    if enemy {
                        DizzyBar::Enemy
                    } else {
                        DizzyBar::Player
                    },
                    Node {
                        width: percent(0),
                        height: percent(100),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.78, 0.32)),
                ));
            });

            box_.spawn((
                if enemy {
                    BattleText::EnemyDizzy
                } else {
                    BattleText::PlayerDizzy
                },
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(INK),
            ));
        });
}

fn spawn_command_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            BattlePanel::Command,
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|panel| {
            command_btn(panel, 0, "FIGHT");
            command_btn(panel, 1, "RUN");
        });
}

fn command_btn(parent: &mut ChildSpawnerCommands, index: usize, label: &'static str) {
    parent
        .spawn((
            Button,
            CommandButton { index },
            Node {
                width: percent(100),
                padding: UiRect::axes(px(12), px(10)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(IDLE),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
                TextColor(INK),
            ));
        });
}

fn spawn_moves_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            BattlePanel::Moves,
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                width: percent(100),
                height: percent(100),
                display: Display::None,
                overflow: Overflow::scroll_y(),
                ..default()
            },
        ))
        .with_children(|panel| {
            for index in 0..DECK_MAX_CARDS {
                panel
                    .spawn((
                        Button,
                        MoveButton { index },
                        Node {
                            width: percent(100),
                            padding: UiRect::axes(px(10), px(8)),
                            ..default()
                        },
                        BackgroundColor(IDLE),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(""),
                            TextFont {
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextColor(INK),
                        ));
                    });
            }
            panel
                .spawn((
                    Button,
                    BackButton,
                    Node {
                        width: percent(100),
                        padding: UiRect::axes(px(10), px(8)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(IDLE),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("BACK"),
                        TextFont {
                            font_size: FontSize::Px(16.0),
                            ..default()
                        },
                        TextColor(INK),
                    ));
                });
        });
}

pub(crate) fn update_match_ui(
    session: Option<Res<MatchSession>>,
    mut labels: Query<(&BattleText, &mut Text)>,
    mut bars: Query<(&DizzyBar, &mut Node, &mut BackgroundColor), Without<MoveButton>>,
    mut panels: Query<(&BattlePanel, &mut Node), (Without<DizzyBar>, Without<MoveButton>)>,
    mut commands: Query<
        (&CommandButton, &mut BackgroundColor),
        (Without<DizzyBar>, Without<MoveButton>),
    >,
    mut moves: Query<
        (&MoveButton, &Children, &mut BackgroundColor, &mut Node),
        (Without<CommandButton>, Without<DizzyBar>, Without<BattlePanel>),
    >,
    mut move_labels: Query<&mut Text, Without<BattleText>>,
) {
    let Some(session) = session else {
        return;
    };

    for (kind, mut text) in &mut labels {
        **text = match kind {
            BattleText::EnemyName => session.enemy_name.to_uppercase(),
            BattleText::PlayerName => "YOU".into(),
            BattleText::EnemyDizzy => format!(
                "DIZZY  {} / {}",
                session.enemy_dizziness.round() as i32,
                session.enemy_limit.round() as i32
            ),
            BattleText::PlayerDizzy => format!(
                "DIZZY  {} / {}",
                session.player_dizziness.round() as i32,
                session.player_limit.round() as i32
            ),
            BattleText::Message => session.message.clone(),
        };
    }

    let enemy_ratio = (session.enemy_dizziness / session.enemy_limit).clamp(0.0, 1.0);
    let player_ratio = (session.player_dizziness / session.player_limit).clamp(0.0, 1.0);
    for (bar, mut node, mut color) in &mut bars {
        let ratio = match bar {
            DizzyBar::Enemy => enemy_ratio,
            DizzyBar::Player => player_ratio,
        };
        node.width = percent(ratio * 100.0);
        *color = BackgroundColor(bar_color(ratio));
    }

    let show_moves = session.phase == MatchPhase::Command && session.menu == BattleMenu::Moves;
    let show_commands = session.phase == MatchPhase::Command && session.menu == BattleMenu::Command;
    for (panel, mut node) in &mut panels {
        node.display = match panel {
            BattlePanel::Command if show_commands => Display::Flex,
            BattlePanel::Moves if show_moves => Display::Flex,
            _ => Display::None,
        };
    }

    for (button, mut color) in &mut commands {
        *color = BackgroundColor(
            if session.menu == BattleMenu::Command && button.index == session.selected {
                SELECT
            } else {
                IDLE
            },
        );
    }

    for (button, children, mut color, mut node) in &mut moves {
        let kind = session.player_hand.get(button.index).copied();
        match kind {
            Some(kind) => {
                node.display = Display::Flex;
                *color = BackgroundColor(if show_moves && button.index == session.selected {
                    SELECT
                } else {
                    IDLE
                });
                if let Some(&child) = children.first()
                    && let Ok(mut text) = move_labels.get_mut(child)
                {
                    **text = move_label(kind);
                }
            }
            None => {
                node.display = Display::None;
            }
        }
    }
}

fn bar_color(ratio: f32) -> Color {
    if ratio < 0.34 {
        Color::srgb(0.3, 0.78, 0.32)
    } else if ratio < 0.67 {
        Color::srgb(0.92, 0.78, 0.18)
    } else {
        Color::srgb(0.9, 0.22, 0.18)
    }
}

fn move_label(kind: ItemKind) -> String {
    let tag = match kind {
        ItemKind::Cig(_) => format!("AP {}", Deck::ap_cost(kind)),
        ItemKind::Beer(_) => "BEER".into(),
        ItemKind::Gum(_) => "GUM".into(),
        ItemKind::Lighter => "KEY".into(),
    };
    format!("{}   {tag}", kind.def().name)
}
