use bevy::prelude::Resource;

use crate::{
    items::{Deck, ItemKind},
    npc::{NpcKind, NpcStats},
    player::PlayerStats,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    Enemy,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BattleMenu {
    Command,
    Moves,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MatchPhase {
    Command,
    Resolve,
    Result,
}

#[derive(Clone, Copy)]
pub enum FightEvent {
    Use { side: Side, kind: ItemKind },
    FinishTurn,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MatchOutcome {
    Win,
    Lose,
    Draw,
    Ran,
}

#[derive(Resource)]
pub struct MatchSession {
    pub phase: MatchPhase,
    pub menu: BattleMenu,
    pub selected: usize,
    pub player_hand: Vec<ItemKind>,
    pub enemy_hand: Vec<ItemKind>,
    pub player_dizziness: f32,
    pub player_limit: f32,
    pub enemy_dizziness: f32,
    pub enemy_limit: f32,
    pub enemy_name: String,
    pub enemy_kind: NpcKind,
    pub stake: u32,
    pub message: String,
    pub delay: f32,
    pub queue: Vec<FightEvent>,
    pub outcome: Option<MatchOutcome>,
}

impl MatchSession {
    pub fn new(player: &PlayerStats, enemy: &NpcStats, kind: NpcKind, pack: &Deck) -> Self {
        Self {
            phase: MatchPhase::Command,
            menu: BattleMenu::Command,
            selected: 0,
            player_hand: pack.cards().to_vec(),
            enemy_hand: kind.deck().cards().to_vec(),
            player_dizziness: 0.0,
            player_limit: player.dizziness_limit.max(1.0),
            enemy_dizziness: 0.0,
            enemy_limit: enemy.dizziness_limit.max(1.0),
            enemy_name: enemy.name.clone(),
            enemy_kind: kind,
            stake: kind.stake(),
            message: "What will YOU do?".into(),
            delay: 0.0,
            queue: Vec::new(),
            outcome: None,
        }
    }

    pub fn command_count(&self) -> usize {
        2
    }

    pub fn move_count(&self) -> usize {
        self.player_hand.len()
    }

    pub fn clamp_selected(&mut self) {
        let max = match self.menu {
            BattleMenu::Command => self.command_count().saturating_sub(1),
            BattleMenu::Moves => self.move_count().saturating_sub(1),
        };
        self.selected = self.selected.min(max);
    }

    pub fn player_down(&self) -> bool {
        self.player_dizziness >= self.player_limit
    }

    pub fn enemy_down(&self) -> bool {
        self.enemy_dizziness >= self.enemy_limit
    }

    pub fn anyone_down(&self) -> bool {
        self.player_down() || self.enemy_down()
    }
}

pub fn apply_move(
    user_dizzy: &mut f32,
    foe_dizzy: &mut f32,
    kind: ItemKind,
    user_limit: f32,
    foe_limit: f32,
) {
    let stats = kind.def().stats;
    match kind {
        ItemKind::Cig(_) => {
            let puffs = stats.puffs.unwrap_or(4).max(1);
            let per_user = stats.player_dizziness / puffs as f32;
            let per_foe = stats.enemy_dizziness / puffs as f32;
            for _ in 0..puffs {
                *user_dizzy += per_user;
                *foe_dizzy += per_foe;
                if *user_dizzy >= user_limit || *foe_dizzy >= foe_limit {
                    break;
                }
            }
        }
        ItemKind::Beer(_) => {
            *foe_dizzy += stats.sp_attack;
        }
        ItemKind::Gum(_) => {
            *user_dizzy = (*user_dizzy + stats.player_dizziness).max(0.0);
        }
        ItemKind::Lighter => {}
    }
}

pub fn announce(side: Side, kind: ItemKind, enemy_name: &str) -> String {
    let who = match side {
        Side::Player => "You",
        Side::Enemy => enemy_name,
    };
    let verb = match kind {
        ItemKind::Cig(_) => "smoked",
        ItemKind::Beer(_) => "drank",
        ItemKind::Gum(_) => "chewed",
        ItemKind::Lighter => "used",
    };
    format!("{who} {verb} {}!", kind.def().name)
}
