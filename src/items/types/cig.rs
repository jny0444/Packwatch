use serde::{Deserialize, Serialize};

use crate::items::item_def::ItemDef;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CigTypes {
    MarlboroRed,
    MarlboroGold,
    MarlboroCompact,
    MarlboroAdvance,
    DoubleHappiness11mg,
    DoubleHappiness6mg,
    ClassicIndieMint,
    ClassicConnect,
    CamelYellow,
    CamelBlue,
    CamelConnect,
}

impl CigTypes {
    pub fn def(self) -> ItemDef {
        let (name, description, dizziness_delta) = match self {
            CigTypes::MarlboroRed => ("Marlboro Red", "Full flavor. Hits the head fast.", 18.0),
            CigTypes::MarlboroGold => ("Marlboro Gold", "Lights. Still adds up.", 9.0),
            CigTypes::MarlboroCompact => ("Marlboro Compact", "Short stick. Medium hit.", 12.0),
            CigTypes::MarlboroAdvance => {
                ("Marlboro Advance", "Smoother smoke. Mild dizziness.", 10.0)
            }
            CigTypes::DoubleHappiness11mg => {
                ("Double Happiness 11mg", "Strong. Sit down after.", 20.0)
            }
            CigTypes::DoubleHappiness6mg => {
                ("Double Happiness 6mg", "Lighter stick. Creeps in.", 8.0)
            }
            CigTypes::ClassicIndieMint => (
                "Classic Indie Mint",
                "Menthol. Cold smoke, spinning later.",
                13.0,
            ),
            CigTypes::ClassicConnect => ("Classic Connect", "Compact. Everyday smoke.", 11.0),
            CigTypes::CamelYellow => ("Camel Yellow", "Heavy. Thick smoke.", 16.0),
            CigTypes::CamelBlue => ("Camel Blue", "Milder camel. Slow burn.", 10.0),
            CigTypes::CamelConnect => ("Camel Connect", "Short pack smoke.", 11.0),
        };

        ItemDef::item(name, description, dizziness_delta, true)
    }

    pub fn model(self) -> &'static str {
        match self {
            CigTypes::MarlboroRed => "models/items/cigs/marlboro_red.glb",
            CigTypes::MarlboroGold => "models/items/cigs/marlboro_gold.glb",
            CigTypes::MarlboroCompact => "models/items/cigs/marlboro_compact.glb",
            CigTypes::MarlboroAdvance => "models/items/cigs/marlboro_advance.glb",
            CigTypes::DoubleHappiness11mg => "models/items/cigs/double_happiness_11mg.glb",
            CigTypes::DoubleHappiness6mg => "models/items/cigs/double_happiness_6mg.glb",
            CigTypes::ClassicIndieMint => "models/items/cigs/classic_indie_mint.glb",
            CigTypes::ClassicConnect => "models/items/cigs/classic_connect.glb",
            CigTypes::CamelYellow => "models/items/cigs/camel_yellow.glb",
            CigTypes::CamelBlue => "models/items/cigs/camel_blue.glb",
            CigTypes::CamelConnect => "models/items/cigs/camel_connect.glb",
        }
    }
}
