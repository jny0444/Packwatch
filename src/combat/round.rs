use bevy::{prelude::*, window::CursorOptions};

use crate::{
    camera::set_cursor_locked,
    combat::{
        ai,
        state::{
            BattleMenu, FightEvent, MatchOutcome, MatchPhase, MatchSession, Side, announce,
            apply_move,
        },
        ui::{BackButton, CommandButton, MoveButton},
    },
    items::{Inventory, ItemKind, PlayerDeck, save::save_inventory, wallet::Wallet},
    npc::{NpcKind, NpcStats},
    player::{Player, PlayerStats},
    screens::{ActiveMatch, PlayMode},
};

const STEP_SECS: f32 = 1.05;

pub fn init_session(
    mut commands: Commands,
    active: Res<ActiveMatch>,
    players: Query<&PlayerStats, With<Player>>,
    npcs: Query<(&NpcStats, &NpcKind)>,
    pack: Res<PlayerDeck>,
) {
    let Some(opponent) = active.opponent() else {
        return;
    };
    let Ok(player) = players.single() else {
        return;
    };
    let Ok((enemy, kind)) = npcs.get(opponent) else {
        return;
    };
    commands.insert_resource(MatchSession::new(player, enemy, *kind, &pack.0));
}

pub fn clear_session(
    mut commands: Commands,
    mut active: ResMut<ActiveMatch>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    commands.remove_resource::<MatchSession>();
    active.clear();
    set_cursor_locked(&mut cursor_options, true);
}

pub fn match_input(
    mut session: ResMut<MatchSession>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    command_buttons: Query<(&Interaction, &CommandButton), Changed<Interaction>>,
    move_buttons: Query<(&Interaction, &MoveButton), Changed<Interaction>>,
    back: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut next_mode: ResMut<NextState<PlayMode>>,
    mut wallet: ResMut<Wallet>,
    mut inventory: ResMut<Inventory>,
    mut pack: ResMut<PlayerDeck>,
) {
    match session.phase {
        MatchPhase::Command => {
            auto_continue(&mut session);
            if session.phase == MatchPhase::Command {
                read_command(
                    &mut session,
                    &keyboard,
                    &command_buttons,
                    &move_buttons,
                    &back,
                    &inventory,
                    &mut wallet,
                );
            }
            if session.phase == MatchPhase::Result {
                finish_round(&mut session, &mut wallet, &mut inventory, &mut pack);
            }
        }
        MatchPhase::Resolve => {}
        MatchPhase::Result => {
            let clicked = mouse.just_pressed(MouseButton::Left)
                || keyboard.just_pressed(KeyCode::Enter)
                || keyboard.just_pressed(KeyCode::NumpadEnter)
                || keyboard.just_pressed(KeyCode::Escape)
                || keyboard.just_pressed(KeyCode::Space);
            if clicked {
                pack.0.clamp_to_inventory(&inventory);
                save_inventory(&inventory, &wallet, &pack.0);
                next_mode.set(PlayMode::Exploring);
            }
        }
    }
}

fn read_command(
    session: &mut MatchSession,
    keyboard: &ButtonInput<KeyCode>,
    command_buttons: &Query<(&Interaction, &CommandButton), Changed<Interaction>>,
    move_buttons: &Query<(&Interaction, &MoveButton), Changed<Interaction>>,
    back: &Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    inventory: &Inventory,
    wallet: &mut Wallet,
) {
    if session.menu == BattleMenu::Moves
        && (keyboard.just_pressed(KeyCode::Escape)
            || back
                .iter()
                .any(|interaction| *interaction == Interaction::Pressed))
    {
        session.menu = BattleMenu::Command;
        session.selected = 0;
        session.message = "What will YOU do?".into();
        return;
    }

    if session.menu == BattleMenu::Command {
        if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::ArrowRight) {
            session.selected = (session.selected + 1) % session.command_count();
        }
        if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::ArrowLeft) {
            session.selected =
                (session.selected + session.command_count() - 1) % session.command_count();
        }
        for (interaction, button) in command_buttons {
            if *interaction == Interaction::Pressed {
                session.selected = button.index;
                confirm_command(session, inventory, wallet);
                return;
            }
        }
        if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter) {
            confirm_command(session, inventory, wallet);
        }
        return;
    }

    let count = session.move_count();
    if count == 0 {
        session.menu = BattleMenu::Command;
        session.message = "You have no moves packed!".into();
        return;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::ArrowRight) {
        session.selected = (session.selected + 1) % count;
    }
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::ArrowLeft) {
        session.selected = (session.selected + count - 1) % count;
    }
    for (interaction, button) in move_buttons {
        if *interaction == Interaction::Pressed && button.index < session.player_hand.len() {
            session.selected = button.index;
            confirm_move(session, inventory);
            return;
        }
    }
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter) {
        confirm_move(session, inventory);
    }
}

fn confirm_command(session: &mut MatchSession, _inventory: &Inventory, wallet: &mut Wallet) {
    match session.selected {
        0 => {
            if session.player_hand.is_empty() {
                session.message = "You have no moves packed!".into();
                return;
            }
            session.menu = BattleMenu::Moves;
            session.selected = 0;
            session.message = "Choose a move.".into();
        }
        _ => {
            session.outcome = Some(MatchOutcome::Ran);
            session.phase = MatchPhase::Result;
            session.message = format!(
                "You ran. Paid ${} to the {}.",
                wallet.balance.min(session.stake),
                session.enemy_name
            );
        }
    }
}

fn confirm_move(session: &mut MatchSession, inventory: &Inventory) {
    let Some(&kind) = session.player_hand.get(session.selected) else {
        return;
    };
    if !can_play(kind, inventory) {
        session.message = if kind.def().needs_lighter {
            "You need a lighter.".into()
        } else {
            "You don't have that anymore.".into()
        };
        return;
    }
    session.player_hand.remove(session.selected);
    start_turn(session, Some(kind));
}

fn can_play(kind: ItemKind, inventory: &Inventory) -> bool {
    if kind.def().needs_lighter && !inventory.has(ItemKind::Lighter, 1) {
        return false;
    }
    inventory.has(kind, 1)
}

fn start_turn(session: &mut MatchSession, player_move: Option<ItemKind>) {
    let enemy_move = ai::pick_index(session).and_then(|index| {
        (index < session.enemy_hand.len()).then(|| session.enemy_hand.remove(index))
    });

    session.queue.clear();
    if let Some(kind) = player_move {
        session.queue.push(FightEvent::Use {
            side: Side::Player,
            kind,
        });
    }
    if let Some(kind) = enemy_move {
        session.queue.push(FightEvent::Use {
            side: Side::Enemy,
            kind,
        });
    }
    session.queue.push(FightEvent::FinishTurn);
    session.phase = MatchPhase::Resolve;
    session.menu = BattleMenu::Command;
    session.selected = 0;
    session.delay = 0.01;
}

fn auto_continue(session: &mut MatchSession) {
    if session.player_hand.is_empty() && session.enemy_hand.is_empty() {
        session.phase = MatchPhase::Result;
    }
}

pub fn match_resolve(
    time: Res<Time>,
    mut session: ResMut<MatchSession>,
    mut inventory: ResMut<Inventory>,
    mut wallet: ResMut<Wallet>,
    mut pack: ResMut<PlayerDeck>,
) {
    if session.phase != MatchPhase::Resolve {
        return;
    }

    session.delay -= time.delta_secs();
    if session.delay > 0.0 {
        return;
    }

    if session.queue.is_empty() {
        session.phase = MatchPhase::Command;
        session.message = "What will YOU do?".into();
        return;
    }

    let event = session.queue.remove(0);
    match event {
        FightEvent::Use { side, kind } => {
            session.message = announce(side, kind, &session.enemy_name);
            match side {
                Side::Player => {
                    let _ = inventory.remove(kind, 1);
                    let mut user = session.player_dizziness;
                    let mut foe = session.enemy_dizziness;
                    apply_move(
                        &mut user,
                        &mut foe,
                        kind,
                        session.player_limit,
                        session.enemy_limit,
                    );
                    session.player_dizziness = user;
                    session.enemy_dizziness = foe;
                }
                Side::Enemy => {
                    let mut user = session.enemy_dizziness;
                    let mut foe = session.player_dizziness;
                    apply_move(
                        &mut user,
                        &mut foe,
                        kind,
                        session.enemy_limit,
                        session.player_limit,
                    );
                    session.enemy_dizziness = user;
                    session.player_dizziness = foe;
                }
            }
            if session.anyone_down() {
                session.queue.clear();
                session.phase = MatchPhase::Result;
                session.delay = STEP_SECS;
                finish_round(&mut session, &mut wallet, &mut inventory, &mut pack);
                return;
            }
            session.delay = STEP_SECS;
        }
        FightEvent::FinishTurn => {
            if session.player_hand.is_empty() && session.enemy_hand.is_empty() {
                session.phase = MatchPhase::Result;
                finish_round(&mut session, &mut wallet, &mut inventory, &mut pack);
            } else if session.player_hand.is_empty() {
                start_turn(&mut session, None);
            } else {
                session.phase = MatchPhase::Command;
                session.menu = BattleMenu::Command;
                session.selected = 0;
                session.message = "What will YOU do?".into();
                session.delay = 0.0;
            }
        }
    }
}

fn finish_round(
    session: &mut MatchSession,
    wallet: &mut Wallet,
    inventory: &mut Inventory,
    pack: &mut PlayerDeck,
) {
    if session.outcome == Some(MatchOutcome::Ran) {
        let paid = wallet.balance.min(session.stake);
        wallet.spend(paid);
        session.message = format!(
            "You ran. Paid ${paid} to the {}.\nClick or Enter to leave.",
            session.enemy_name
        );
    } else if session.enemy_down() {
        wallet.add(session.stake);
        session.outcome = Some(MatchOutcome::Win);
        session.message = format!(
            "The {} hit their limit and blacked out.\nThey paid you ${}.\nClick or Enter to leave.",
            session.enemy_name, session.stake
        );
    } else if session.player_down() {
        let paid = wallet.balance.min(session.stake);
        wallet.spend(paid);
        session.outcome = Some(MatchOutcome::Lose);
        session.message = format!(
            "You hit your limit and blacked out.\nPaid ${paid} to the {}.\nClick or Enter to leave.",
            session.enemy_name
        );
    } else {
        let player = session.player_dizziness;
        let enemy = session.enemy_dizziness;
        let (outcome, line) = if (player - enemy).abs() < 0.5 {
            (MatchOutcome::Draw, "It's a draw. Nobody pays.".into())
        } else if player > enemy {
            let paid = wallet.balance.min(session.stake);
            wallet.spend(paid);
            (
                MatchOutcome::Lose,
                format!(
                    "You were dizzier. Paid ${paid} to the {}.",
                    session.enemy_name
                ),
            )
        } else {
            wallet.add(session.stake);
            (
                MatchOutcome::Win,
                format!(
                    "The {} was dizzier. They paid you ${}.",
                    session.enemy_name, session.stake
                ),
            )
        };
        session.outcome = Some(outcome);
        session.message = format!("{line}\nClick or Enter to leave.");
    }

    pack.0.clamp_to_inventory(inventory);
    save_inventory(inventory, wallet, &pack.0);
}
