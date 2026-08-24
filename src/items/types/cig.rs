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
        use crate::items::item_def::ItemStats;

        let (name, description, player, enemy, ap_costs, puffs) = match self {
            CigTypes::MarlboroRed => (
                "Marlboro Red",
                "Full flavor. Hits the head fast.",
                8.0,
                18.0,
                2,
                4,
            ),
            CigTypes::MarlboroGold => ("Marlboro Gold", "Lights. Still adds up.", 4.0, 9.0, 1, 4),
            CigTypes::MarlboroCompact => (
                "Marlboro Compact",
                "Short stick. Medium hit.",
                5.0,
                12.0,
                1,
                4,
            ),
            CigTypes::MarlboroAdvance => (
                "Marlboro Advance",
                "Smoother smoke. Mild dizziness.",
                4.0,
                10.0,
                1,
                4,
            ),
            CigTypes::DoubleHappiness11mg => (
                "Double Happiness 11mg",
                "Strong. Sit down after.",
                9.0,
                20.0,
                2,
                4,
            ),
            CigTypes::DoubleHappiness6mg => (
                "Double Happiness 6mg",
                "Lighter stick. Creeps in.",
                3.0,
                8.0,
                1,
                4,
            ),
            CigTypes::ClassicIndieMint => (
                "Classic Indie Mint",
                "Menthol. Cold smoke, spinning later.",
                6.0,
                13.0,
                1,
                4,
            ),
            CigTypes::ClassicConnect => (
                "Classic Connect",
                "Compact. Everyday smoke.",
                5.0,
                11.0,
                1,
                4,
            ),
            CigTypes::CamelYellow => ("Camel Yellow", "Heavy. Thick smoke.", 7.0, 16.0, 2, 4),
            CigTypes::CamelBlue => ("Camel Blue", "Milder camel. Slow burn.", 4.0, 10.0, 1, 4),
            CigTypes::CamelConnect => ("Camel Connect", "Short pack smoke.", 5.0, 11.0, 1, 4),
        };

        ItemDef::item(
            name,
            description,
            ItemStats::cig(player, enemy, ap_costs, puffs),
            true,
        )
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
