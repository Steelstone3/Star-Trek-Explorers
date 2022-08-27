use std::fmt::Display;

use super::ship_status::ShipSystems;
use crate::assests::{
    faction_names::Faction, klingon_ship_classification::KlingonShipClassification,
    klingon_ship_names::KlingonShipName,
};
use rand::random;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub struct KlingonShip {
    pub name: KlingonShipName,
    pub faction: Faction,
    pub class: KlingonShipClassification,
    pub systems: ShipSystems,
}

impl KlingonShip {
    pub fn new() -> Self {
        KlingonShip {
            name: random(),
            class: random(),
            faction: Faction::KlingonEmpire,
            systems: ShipSystems::default(),
        }
    }

    pub fn credentials(&self) -> String {
        format!(
            "Scanning Ship...\n  | Name: {} | Faction: {} | Class: {} |",
            self.name, self.faction, self.class
        )
    }

    pub fn defensive_capabilities(&self) -> String {
        format!(
            "| Name: {} | Faction: {} | Class: {} | Shield strength: {} | Hull integrity: {} |",
            self.name, self.faction, self.class, self.systems.shield_strength, self.systems.hull_integrity
        )
    }
}

impl Default for KlingonShip {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for KlingonShip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.defensive_capabilities() )
    }
}

#[cfg(test)]
mod federation_ship_should {
    use super::*;

    #[test]
    fn be_part_of_the_klingon_empire_faction() {
        assert_eq!(Faction::KlingonEmpire, KlingonShip::default().faction);
    }

    #[test]
    fn have_default_ship_systems() {
        let ship_systems = KlingonShip::default().systems;
        let default = ShipSystems::default();

        assert_eq!(ship_systems.shield_strength, default.shield_strength);
        assert_eq!(ship_systems.hull_integrity, default.hull_integrity);
        assert_eq!(ship_systems.phaser_max_damage, default.phaser_max_damage);
        assert_eq!(ship_systems.phaser_min_damage, default.phaser_min_damage);
        assert_eq!(ship_systems.torpedo_max_damage, default.torpedo_max_damage);
        assert_eq!(ship_systems.torpedo_min_damage, default.torpedo_min_damage);
    }
}
