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
    StellarDoubleShift,
    CamelYellow,
    CamelBlue,
    CamelConnect,
    Bidi,
    Cashtri,
    Mond,
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
            CigTypes::MarlboroGold => ("Malboro Gold", "Lights. Still adds up.", 4.0, 9.0, 1, 4),
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
                "Double Happiness",
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
                "Indiemint",
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
            CigTypes::StellarDoubleShift => (
                "Doubleshift",
                "Double the fun, double the menthol",
                5.0,
                11.0,
                1,
                4,
            ),
            CigTypes::CamelYellow => ("Camel", "Heavy. Thick smoke.", 7.0, 16.0, 2, 4),
            CigTypes::CamelBlue => ("Camel Blue", "Milder camel. Slow burn.", 4.0, 10.0, 1, 4),
            CigTypes::CamelConnect => ("Camel Connect", "Short pack smoke.", 5.0, 11.0, 1, 4),
            CigTypes::Bidi => ("Bidi", "Leaf wrap. Burns fast, hits hard.", 6.0, 14.0, 1, 3),
            CigTypes::Cashtri => ("Kingfisher", "me toh lab rep hun.", 5.0, 12.0, 1, 4),
            CigTypes::Mond => ("Mond", "SAY A SIII", 6.0, 14.0, 1, 4),
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
            CigTypes::MarlboroGold => "customASSets/Malboro Gold.glb",
            CigTypes::MarlboroCompact => "models/items/cigs/marlboro_compact.glb",
            CigTypes::MarlboroAdvance => "models/items/cigs/marlboro_advance.glb",
            CigTypes::DoubleHappiness11mg | CigTypes::DoubleHappiness6mg => {
                "customASSets/Double Happiness.glb"
            }
            CigTypes::ClassicIndieMint => "customASSets/Indiemint (1).glb",
            CigTypes::ClassicConnect => "models/items/cigs/classic_connect.glb",
            CigTypes::StellarDoubleShift => "customASSets/Doubleshift (1).glb",
            CigTypes::CamelYellow | CigTypes::CamelBlue | CigTypes::CamelConnect => {
                "customASSets/Camel (1).glb"
            }
            CigTypes::Bidi => "models/items/cigs/bidi.glb",
            CigTypes::Cashtri => "customASSets/Cashtri Model.glb",
            CigTypes::Mond => "customASSets/mond.glb",
        }
    }
}
