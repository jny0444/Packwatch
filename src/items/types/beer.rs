use serde::{Deserialize, Serialize};

use crate::items::item_def::ItemDef;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum BeerTypes {
    BudweiserMagnum,
    KingfisherStrong,
    Corona,
    Guinness,
}

impl BeerTypes {
    pub fn def(self) -> ItemDef {
        let (name, description, dizziness_delta) = match self {
            BeerTypes::BudweiserMagnum => ("Budweiser Magnum", "A short strong bottle.", 18.0),
            BeerTypes::KingfisherStrong => ("Kingfisher Strong", "Hits harder than ultra.", 16.0),
            BeerTypes::Corona => ("Corona", "Small bottle. Easy to underestimate.", 10.0),
            BeerTypes::Guinness => ("Guinness", "Thick. Slow dizziness.", 13.0),
        };

        ItemDef::item(name, description, dizziness_delta, false)
    }

    pub fn model(self) -> &'static str {
        match self {
            BeerTypes::BudweiserMagnum => "models/items/beer/budweiser_magnum.glb",
            BeerTypes::KingfisherStrong => "models/items/beer/kingfisher_strong.glb",
            BeerTypes::Corona => "models/items/beer/corona.glb",
            BeerTypes::Guinness => "models/items/beer/guinness.glb",
        }
    }
}
