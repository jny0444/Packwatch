use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::items::{Inventory, ItemKind};

pub const DECK_AP: u32 = 5;
pub const DECK_MAX_CARDS: usize = 8;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Deck {
    cards: Vec<ItemKind>,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct PlayerDeck(pub Deck);

impl Deck {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn from_cards(cards: &[ItemKind]) -> Self {
        let mut deck = Self::new();
        for &kind in cards {
            if !deck.try_add(kind, None) {
                break;
            }
        }
        deck
    }

    pub fn cards(&self) -> &[ItemKind] {
        &self.cards
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn ap_cost(kind: ItemKind) -> u32 {
        kind.def().stats.ap_costs
    }

    pub fn ap_used(&self) -> u32 {
        self.cards.iter().map(|kind| Self::ap_cost(*kind)).sum()
    }

    pub fn ap_left(&self) -> u32 {
        DECK_AP.saturating_sub(self.ap_used())
    }

    pub fn count(&self, kind: ItemKind) -> u32 {
        self.cards.iter().filter(|card| **card == kind).count() as u32
    }

    pub fn can_add(
        &self,
        kind: ItemKind,
        inventory: Option<&Inventory>,
    ) -> Result<(), &'static str> {
        if matches!(kind, ItemKind::Lighter) {
            return Err("Lighter stays in key items.");
        }
        if self.cards.len() >= DECK_MAX_CARDS {
            return Err("Pack is full.");
        }
        let cost = Self::ap_cost(kind);
        if cost > self.ap_left() {
            return Err("Not enough AP.");
        }
        let Some(inventory) = inventory else {
            return Ok(());
        };
        if kind.def().needs_lighter && !inventory.has(ItemKind::Lighter, 1) {
            return Err("Need a lighter.");
        }
        if !inventory.has(kind, self.count(kind) + 1) {
            return Err("Not enough in the bag.");
        }
        Ok(())
    }

    pub fn try_add(&mut self, kind: ItemKind, inventory: Option<&Inventory>) -> bool {
        if self.can_add(kind, inventory).is_err() {
            return false;
        }
        self.cards.push(kind);
        true
    }

    pub fn remove_at(&mut self, index: usize) -> Option<ItemKind> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<ItemKind> {
        self.cards.pop()
    }

    pub fn clamp_to_inventory(&mut self, inventory: &Inventory) {
        let mut kept = Vec::new();
        for kind in self.cards.drain(..) {
            let already = kept.iter().filter(|card| **card == kind).count() as u32;
            let used: u32 = kept.iter().map(|card| Self::ap_cost(*card)).sum();
            if inventory.has(kind, already + 1)
                && kept.len() < DECK_MAX_CARDS
                && used + Self::ap_cost(kind) <= DECK_AP
                && (!kind.def().needs_lighter || inventory.has(ItemKind::Lighter, 1))
            {
                kept.push(kind);
            }
        }
        self.cards = kept;
    }
}
