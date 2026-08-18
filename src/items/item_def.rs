use crate::items::Pocket;

pub struct ItemDef {
    pub name: &'static str,
    pub description: &'static str,
    pub pocket: Pocket,
    pub max_stack: u32,
    pub tossable: bool,
    pub needs_lighter: bool,
    pub dizziness_delta: f32,
}

impl ItemDef {
    pub const fn item(
        name: &'static str,
        description: &'static str,
        dizziness_delta: f32,
        needs_lighter: bool,
    ) -> Self {
        Self {
            name,
            description,
            pocket: Pocket::Items,
            max_stack: 999,
            tossable: true,
            needs_lighter,
            dizziness_delta,
        }
    }

    pub const fn key_item(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            pocket: Pocket::KeyItems,
            max_stack: 1,
            tossable: false,
            needs_lighter: false,
            dizziness_delta: 0.0,
        }
    }
}
