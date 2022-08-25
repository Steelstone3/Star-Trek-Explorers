use super::ship_status::ShipStatus;
use crate::assests::{
    faction_names::Faction, klingon_ship_names::KlingonShipName, klingon_ship_classification::KlingonShipClassification
};
use rand::random;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub struct KlingonShip {
    pub name: KlingonShipName,
    pub faction: Faction,
    pub class: KlingonShipClassification,
    pub systems: ShipStatus,
}

impl KlingonShip {
    pub fn new() -> Self {
        KlingonShip {
            name: random(),
            class: random(),
            faction: Faction::KlingonEmpire,
            systems: random(),
        }
    }
}

impl Default for KlingonShip {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod federation_ship_should {
    use super::*;

    #[test]
    fn be_part_of_the_federation_of_planets_faction() {
        assert_eq!(
            Faction::KlingonEmpire,
            KlingonShip::default().faction
        );
    }
}
