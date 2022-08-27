use rand_derive2::RandGen;
use std::fmt::{Display, Formatter};

#[derive(RandGen)]
pub enum KlingonShipClassification {
    BirdOfPrey,
    D7BattleCruiser,
}

impl Display for KlingonShipClassification {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KlingonShipClassification::BirdOfPrey => {
                write!(formatter, "Bird Of Prey",)
            }

            KlingonShipClassification::D7BattleCruiser => {
                write!(formatter, "D7 Battle Cruiser",)
            }
        }
    }
}
