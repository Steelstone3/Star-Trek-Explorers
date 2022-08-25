use crate::assests::faction_names::Faction;
use crate::assests::federation_ship_classification_names::FederationShipClassification;
use crate::assests::federation_ship_names::FederationShipName;
use crate::models::ship_systems::ShipSystems;
use rand::random;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub struct FederationShip {
    pub name: FederationShipName,
    pub faction: Faction,
    pub class: FederationShipClassification,
    pub systems: ShipSystems,
}

impl FederationShip {
    pub fn new() -> Self {
        FederationShip {
            name: random(),
            class: random(),
            faction: Faction::FederationOfPlanets,
            systems: random(),
        }
    }
}

impl Default for FederationShip {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod federation_ship_should {
    use super::*;
    #[test]
    fn be_part_of_the_federation_of_planets_faction() {
        assert_eq!(Faction::FederationOfPlanets, FederationShip::new().faction);
    }
}
