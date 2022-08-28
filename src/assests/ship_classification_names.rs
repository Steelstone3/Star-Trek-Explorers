use rand_derive2::RandGen;
use std::fmt::{Display, Formatter};

pub const FEDERATION_SHIP_CLASSIFICATIONS: [ShipClassification; 6] = [
    ShipClassification::Defiant,
    ShipClassification::Intrepid,
    ShipClassification::Consitution,
    ShipClassification::Ambassador,
    ShipClassification::Galaxy,
    ShipClassification::Sovereign,
];

pub const KLINGON_SHIP_CLASSIFICATIONS: [ShipClassification; 2] = [
    ShipClassification::BirdOfPrey,
    ShipClassification::D7BattleCruiser,
];

#[derive(RandGen, PartialEq, Clone, Copy)]
pub enum ShipClassification {
    //Federation
    Defiant,
    Intrepid,
    Consitution,
    Ambassador,
    Galaxy,
    Sovereign,
    //Klingon
    BirdOfPrey,
    D7BattleCruiser,
}

impl Display for ShipClassification {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipClassification::Defiant => {
                write!(formatter, "Defiant Class")
            }

            ShipClassification::Intrepid => {
                write!(formatter, "Intrepid Class")
            }

            ShipClassification::Consitution => {
                write!(formatter, "Consitution Class")
            }

            ShipClassification::Ambassador => {
                write!(formatter, "Ambassador Class")
            }

            ShipClassification::Galaxy => {
                write!(formatter, "Galaxy Class")
            }

            ShipClassification::Sovereign => {
                write!(formatter, "Sovereign Class")
            }

            ShipClassification::BirdOfPrey => {
                write!(formatter, "Bird Of Prey",)
            }

            ShipClassification::D7BattleCruiser => {
                write!(formatter, "D7 Battle Cruiser",)
            }
        }
    }
}
