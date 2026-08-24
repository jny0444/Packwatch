use crate::{
    combat::state::MatchSession,
    items::ItemKind,
    npc::NpcKind,
};

pub fn pick_index(session: &MatchSession) -> Option<usize> {
    let hand = &session.enemy_hand;
    if hand.is_empty() {
        return None;
    }

    match session.enemy_kind {
        NpcKind::Guide => pick_guide(hand, session.enemy_dizziness),
        NpcKind::LightSmoker => pick_light(
            hand,
            session.enemy_dizziness,
            session.player_dizziness,
        ),
        NpcKind::HeavySmoker => pick_heavy(hand, session.enemy_dizziness),
        NpcKind::ShopKeeper => None,
    }
}

fn find_kind(hand: &[ItemKind], pred: impl Fn(ItemKind) -> bool) -> Option<usize> {
    hand.iter().position(|kind| pred(*kind))
}

fn cig_hit(kind: ItemKind) -> f32 {
    kind.def().stats.enemy_dizziness
}

fn pick_guide(hand: &[ItemKind], self_d: f32) -> Option<usize> {
    if self_d >= 25.0
        && let Some(index) = find_kind(hand, |kind| matches!(kind, ItemKind::Gum(_)))
    {
        return Some(index);
    }
    hand.iter()
        .enumerate()
        .filter(|(_, kind)| matches!(kind, ItemKind::Cig(_)))
        .min_by(|a, b| cig_hit(*a.1).total_cmp(&cig_hit(*b.1)))
        .map(|(index, _)| index)
        .or(Some(0))
}

fn pick_light(hand: &[ItemKind], self_d: f32, foe_d: f32) -> Option<usize> {
    if self_d > foe_d + 6.0
        && let Some(index) = find_kind(hand, |kind| matches!(kind, ItemKind::Gum(_)))
    {
        return Some(index);
    }
    strongest_cig(hand)
        .or_else(|| find_kind(hand, |kind| matches!(kind, ItemKind::Gum(_))))
        .or(Some(0))
}

fn pick_heavy(hand: &[ItemKind], self_d: f32) -> Option<usize> {
    if self_d >= 28.0
        && let Some(index) = find_kind(hand, |kind| matches!(kind, ItemKind::Gum(_)))
    {
        return Some(index);
    }
    find_kind(hand, |kind| matches!(kind, ItemKind::Beer(_)))
        .or_else(|| strongest_cig(hand))
        .or_else(|| find_kind(hand, |kind| matches!(kind, ItemKind::Gum(_))))
        .or(Some(0))
}

fn strongest_cig(hand: &[ItemKind]) -> Option<usize> {
    hand.iter()
        .enumerate()
        .filter(|(_, kind)| matches!(kind, ItemKind::Cig(_)))
        .max_by(|a, b| cig_hit(*a.1).total_cmp(&cig_hit(*b.1)))
        .map(|(index, _)| index)
}
