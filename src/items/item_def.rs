use crate::items::Pocket;

#[derive(Clone, Copy)]
pub struct ItemStats {
    pub player_dizziness: f32,
    pub enemy_dizziness: f32,
    pub sp_attack: f32,
    pub ap_costs: u32,
    pub puffs: Option<u32>,
}

impl ItemStats {
    pub const fn none() -> Self {
        Self {
            player_dizziness: 0.0,
            enemy_dizziness: 0.0,
            sp_attack: 0.0,
            ap_costs: 0,
            puffs: None,
        }
    }

    pub const fn cig(
        player_dizziness: f32,
        enemy_dizziness: f32,
        ap_costs: u32,
        puffs: u32,
    ) -> Self {
        Self {
            player_dizziness,
            enemy_dizziness,
            sp_attack: 0.0,
            ap_costs,
            puffs: Some(puffs),
        }
    }

    pub const fn beer(sp_attack: f32) -> Self {
        Self {
            player_dizziness: 0.0,
            enemy_dizziness: 0.0,
            sp_attack,
            ap_costs: 0,
            puffs: None,
        }
    }

    pub const fn gum(player_dizziness: f32) -> Self {
        Self {
            player_dizziness,
            enemy_dizziness: 0.0,
            sp_attack: 0.0,
            ap_costs: 0,
            puffs: None,
        }
    }
}

pub struct ItemDef {
    pub name: &'static str,
    pub description: &'static str,
    pub pocket: Pocket,
    pub max_stack: u32,
    pub needs_lighter: bool,
    pub stats: ItemStats,
}

impl ItemDef {
    pub const fn item(
        name: &'static str,
        description: &'static str,
        stats: ItemStats,
        needs_lighter: bool,
    ) -> Self {
        Self {
            name,
            description,
            pocket: Pocket::Items,
            max_stack: 999,
            needs_lighter,
            stats,
        }
    }

    pub const fn key_item(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            pocket: Pocket::KeyItems,
            max_stack: 1,
            needs_lighter: false,
            stats: ItemStats::none(),
        }
    }
}
