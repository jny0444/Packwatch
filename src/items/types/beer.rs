use crate::items::item_def::ItemDef;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BeerTypes {
    BudweiserMagnum,
    KingfisherUltra,
    KingfisherStrong,
    Corona,
    PeoplesLager,
    Godfather,
    Guinness,
    BroCode,
}

impl BeerTypes {
    pub fn def(self) -> ItemDef {
        let (name, description, dizziness_delta) = match self {
            BeerTypes::BudweiserMagnum => ("Budweiser Magnum", "A short strong bottle.", 18.0),
            BeerTypes::KingfisherUltra => {
                ("Kingfisher Ultra", "Light beer. Still spins a little.", 8.0)
            }
            BeerTypes::KingfisherStrong => ("Kingfisher Strong", "Hits harder than ultra.", 16.0),
            BeerTypes::Corona => ("Corona", "Small bottle. Easy to underestimate.", 10.0),
            BeerTypes::PeoplesLager => ("People's Lager", "Cheap lager. Goes down quick.", 11.0),
            BeerTypes::Godfather => ("Godfather", "Heavy. Don't mix with smokes.", 20.0),
            BeerTypes::Guinness => ("Guinness", "Thick. Slow dizziness.", 13.0),
            BeerTypes::BroCode => ("Bro Code", "Small and mean.", 14.0),
        };

        ItemDef::item(name, description, dizziness_delta, false)
    }
}
