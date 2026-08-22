use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const STARTING_BALANCE: u32 = 500;

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub balance: u32,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            balance: STARTING_BALANCE,
        }
    }

    pub fn can_afford(&self, cost: u32) -> bool {
        self.balance >= cost
    }

    pub fn spend(&mut self, cost: u32) -> bool {
        if !self.can_afford(cost) {
            return false;
        }
        self.balance -= cost;
        true
    }

    pub fn add(&mut self, amount: u32) {
        self.balance = self.balance.saturating_add(amount);
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}
