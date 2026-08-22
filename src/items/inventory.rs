use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::items::{ItemKind, Pocket};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Stack {
    pub kind: ItemKind,
    pub count: u32,
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Inventory {
    items: Vec<Option<Stack>>,
    key_items: Vec<Option<Stack>>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: vec![None; Pocket::Items.max_slots()],
            key_items: vec![None; Pocket::KeyItems.max_slots()],
        }
    }

    pub fn slots(&self, pocket: Pocket) -> &[Option<Stack>] {
        match pocket {
            Pocket::Items => &self.items,
            Pocket::KeyItems => &self.key_items,
        }
    }

    fn slots_mut(&mut self, pocket: Pocket) -> &mut Vec<Option<Stack>> {
        match pocket {
            Pocket::Items => &mut self.items,
            Pocket::KeyItems => &mut self.key_items,
        }
    }

    pub fn add(&mut self, kind: ItemKind, mut amount: u32) -> bool {
        if amount == 0 {
            return true;
        }

        let def = kind.def();
        let max_stack = def.max_stack;
        let mut next = self.clone();

        {
            let slots = next.slots_mut(def.pocket);

            for slot in slots.iter_mut() {
                if amount == 0 {
                    break;
                }
                let Some(stack) = slot else {
                    continue;
                };
                if stack.kind != kind {
                    continue;
                }
                let space = max_stack.saturating_sub(stack.count);
                let put = amount.min(space);
                stack.count += put;
                amount -= put;
            }

            for slot in slots.iter_mut() {
                if amount == 0 {
                    break;
                }
                if slot.is_some() {
                    continue;
                }
                let put = amount.min(max_stack);
                *slot = Some(Stack { kind, count: put });
                amount -= put;
            }
        }

        if amount != 0 {
            return false;
        }

        *self = next;
        true
    }

    pub fn has(&self, kind: ItemKind, n: u32) -> bool {
        if n == 0 {
            return true;
        }

        let def = kind.def();
        let count: u32 = self
            .slots(def.pocket)
            .iter()
            .flatten()
            .filter(|stack| stack.kind == kind)
            .map(|stack| stack.count)
            .sum();
        count >= n
    }

    pub fn remove(&mut self, kind: ItemKind, mut amount: u32) -> bool {
        if amount == 0 {
            return true;
        }
        if !self.has(kind, amount) {
            return false;
        }

        let def = kind.def();
        for slot in self.slots_mut(def.pocket) {
            if amount == 0 {
                break;
            }
            let Some(stack) = slot else {
                continue;
            };
            if stack.kind != kind {
                continue;
            }
            let take = amount.min(stack.count);
            stack.count -= take;
            amount -= take;
            if stack.count == 0 {
                *slot = None;
            }
        }

        amount == 0
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}
