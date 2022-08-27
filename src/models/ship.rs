use std::fmt::Display;

use super::ship_status::ShipSystems;
use crate::assests::{
    faction_names::Faction, federation_ship_classification_names::FederationShipClassification,
    federation_ship_names::FederationShipName,
    klingon_ship_classification_names::KlingonShipClassification,
    klingon_ship_names::KlingonShipName,
};
use rand_derive2::RandGen;

#[derive(RandGen)]
pub struct Ship {
    pub name: String,
    pub faction: Faction,
    pub class: String,
    pub systems: ShipSystems,
}

impl Ship {
    pub fn create_federation_ship() -> Self {
        Ship {
            name: FederationShipName::generate_random().to_string(),
            faction: Faction::FederationOfPlanets,
            class: FederationShipClassification::generate_random().to_string(),
            systems: ShipSystems::default(),
        }
    }

    pub fn create_klingon_ship() -> Self {
        Ship {
            name: KlingonShipName::generate_random().to_string(),
            faction: Faction::KlingonEmpire,
            class: KlingonShipClassification::generate_random().to_string(),
            systems: ShipSystems::default(),
        }
    }

    pub fn credentials(&self) {
        println!(
            "Scanning Ship...\n  | Name: {} | Faction: {} | Class: {} |",
            self.name, self.faction, self.class
        );
    }

    pub fn overall_capabilities(&self) {
        println!("Ship Capabilities:\n  | Name: {} | Faction: {} | Class: {} |\nOffensive:\n  Phaser Damage: {} | Torpedo Damage {} |\nDefensive:\n  Shield Strength: {} | Hull Integrity: {} |",
            self.name, 
            self.faction, 
            self.class,
            self.systems.phaser_max_damage,
            self.systems.torpedo_max_damage,
            self.systems.shield_strength,
            self.systems.hull_integrity
        )
    }

    pub fn offensive_capabilities(&self) -> String {
        format!(
            "| Name: {} | Faction: {} | Class: {} | Phaser Damage: {} | Torpedo Damage: {} |",
            self.name,
            self.faction,
            self.class,
            self.systems.phaser_max_damage,
            self.systems.torpedo_max_damage
        )
    }

    pub fn defensive_capabilities(&self) -> String {
        format!(
            "| Name: {} | Faction: {} | Class: {} | Shield Strength: {} | Hull Integrity: {} |",
            self.name,
            self.faction,
            self.class,
            self.systems.shield_strength,
            self.systems.hull_integrity
        )
    }
}

impl Display for Ship {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.defensive_capabilities())
    }
}

#[cfg(test)]
mod ship_should {
    use super::*;

    #[test]
    fn create_federation_ship_part_of_the_federation_of_planets_faction() {
        assert_eq!(
            Faction::FederationOfPlanets,
            Ship::create_federation_ship().faction
        );
    }

    #[test]
    fn create_klingon_ship_part_of_the_klingon_empire_faction() {
        assert_eq!(
            Faction::KlingonEmpire,
            Ship::create_klingon_ship().faction
        );
    }

    #[test]
    fn provide_offensive_capabilities_read_out() {
        assert!(!String::is_empty(
            &Ship::create_federation_ship().offensive_capabilities()
        ))
    }

    #[test]
    fn provide_defensive_capabilities_read_out() {
        assert!(!String::is_empty(
            &Ship::create_federation_ship().defensive_capabilities()
        ))
    }

    #[test]
    fn have_default_ship_systems() {
        let ship_systems = Ship::create_federation_ship().systems;
        let default = ShipSystems::default();

        assert_eq!(ship_systems.shield_strength, default.shield_strength);
        assert_eq!(ship_systems.hull_integrity, default.hull_integrity);
        assert_eq!(ship_systems.phaser_max_damage, default.phaser_max_damage);
        assert_eq!(ship_systems.phaser_min_damage, default.phaser_min_damage);
        assert_eq!(ship_systems.torpedo_max_damage, default.torpedo_max_damage);
        assert_eq!(ship_systems.torpedo_min_damage, default.torpedo_min_damage);
    }
}
