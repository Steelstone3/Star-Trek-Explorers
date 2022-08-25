use super::ship_status::ShipSystems;
use crate::assests::{
    faction_names::Faction, federation_ship_classification_names::FederationShipClassification,
    federation_ship_names::FederationShipName,
};
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
            systems: ShipSystems::default(),
        }
    }

    pub fn status(&self) {
        println!(
            "Scanning Ship...\nName: {} Faction: {} Class: {}",
            self.name, self.faction, self.class
        );
    }

    pub fn defensive_status(&self) {
        Self::status(self);
        println!(
            "Shield strength: {} Hull integrity: {}\n",
            self.systems.shield_strength, self.systems.hull_integrity
        )
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
        assert_eq!(
            Faction::FederationOfPlanets,
            FederationShip::default().faction
        );
    }

    #[test]
    fn have_default_ship_systems() {
        let ship_systems = FederationShip::default().systems;
        let default = ShipSystems::default();

        assert_eq!(ship_systems.shield_strength, default.shield_strength);
        assert_eq!(ship_systems.hull_integrity, default.hull_integrity);
        assert_eq!(ship_systems.phaser_max_damage, default.phaser_max_damage);
        assert_eq!(ship_systems.phaser_min_damage, default.phaser_min_damage);
        assert_eq!(ship_systems.torpedo_max_damage, default.torpedo_max_damage);
        assert_eq!(ship_systems.torpedo_min_damage, default.torpedo_min_damage);
    }
}
