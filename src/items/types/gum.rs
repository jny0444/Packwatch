use crate::items::item_def::ItemDef;
use crate::items::Pocket;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GumTypes {
    MintStrongGum,
    LightGum,
}

impl GumTypes {
    pub fn def(self) -> ItemDef {
        let (name, description, dizziness_delta) = match self {
            GumTypes::MintStrongGum => ("Mint Strong Gum", "Cuts dizziness more than light gum.", -22.0),
            GumTypes::LightGum => ("Light Gum", "Takes the edge off.", -10.0),
        };

        ItemDef {
            name,
            description,
            pocket: Pocket::Items,
            max_stack: 999,
            tossable: true,
            needs_lighter: false,
            dizziness_delta,
        }
    }
}
