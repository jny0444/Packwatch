#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Pocket {
    Items,
    KeyItems,
}

impl Pocket {
    pub const ALL: [Pocket; 2] = [Pocket::Items, Pocket::KeyItems];

    pub fn max_slots(self) -> usize {
        match self {
            Pocket::Items => 20,
            Pocket::KeyItems => 10,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Pocket::Items => "ITEMS",
            Pocket::KeyItems => "KEY ITEMS",
        }
    }
}
