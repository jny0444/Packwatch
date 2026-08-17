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
