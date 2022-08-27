use std::fmt::Display;

use super::ship_status::ShipSystems;
use crate::assests::{
    faction_names::Faction, federation_ship_classification_names::FederationShipClassification,
    federation_ship_names::FederationShipName,
    klingon_ship_classification_names::KlingonShipClassification,
    klingon_ship_names::KlingonShipName,
};
use rand_derive2::RandGen;

const SHIELD_DAMAGE_MODIFIER_MAX: u32 = 5;
const SHIELD_DAMAGE_MODIFIER_MIN: u32 = 1;
const HULL_DAMAGE_MODIFIER_MAX: u32 = 5;
const HULL_DAMAGE_MODIFIER_MIN: u32 = 1;

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

    pub fn fire_phasers(&self, defending_ship: &mut Ship) {
        let phaser_shield_damage = self.systems.phaser_max_damage * SHIELD_DAMAGE_MODIFIER_MAX;
        let phaser_hull_damage = self.systems.phaser_min_damage * HULL_DAMAGE_MODIFIER_MIN;
        let has_shield_depleted = defending_ship.systems.shield_strength == 0;

        if phaser_shield_damage >= defending_ship.systems.shield_strength {
            defending_ship.systems.shield_strength = 0;
        } else if defending_ship.systems.shield_strength > 0 {
            defending_ship.systems.shield_strength -= phaser_shield_damage;
        }

        if has_shield_depleted && phaser_hull_damage >= defending_ship.systems.hull_integrity {
            defending_ship.systems.hull_integrity = 0;
        } else if has_shield_depleted && defending_ship.systems.hull_integrity > 0 {
            defending_ship.systems.hull_integrity -= phaser_hull_damage;
        }
    }

    pub fn fire_torpedoes(&self, defending_ship: &mut Ship) {
        let torpedo_shield_damage = self.systems.torpedo_min_damage * SHIELD_DAMAGE_MODIFIER_MIN;
        let torpedo_hull_damage = self.systems.torpedo_max_damage * HULL_DAMAGE_MODIFIER_MAX;
        let has_shield_depleted = defending_ship.systems.shield_strength == 0;
        if torpedo_shield_damage >= defending_ship.systems.shield_strength {
            defending_ship.systems.shield_strength = 0;
        } else if defending_ship.systems.shield_strength > 0 {
            defending_ship.systems.shield_strength -= torpedo_shield_damage;
        }

        if has_shield_depleted && torpedo_hull_damage >= defending_ship.systems.hull_integrity {
            defending_ship.systems.hull_integrity = 0;
        } else if has_shield_depleted && defending_ship.systems.hull_integrity > 0 {
            defending_ship.systems.hull_integrity -= torpedo_hull_damage;
        }
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
        assert_eq!(Faction::KlingonEmpire, Ship::create_klingon_ship().faction);
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

        assert_eq!(default.shield_strength, ship_systems.shield_strength);
        assert_eq!(default.hull_integrity, ship_systems.hull_integrity);
        assert_eq!(default.phaser_max_damage, ship_systems.phaser_max_damage);
        assert_eq!(default.phaser_min_damage, ship_systems.phaser_min_damage);
        assert_eq!(default.torpedo_max_damage, ship_systems.torpedo_max_damage);
        assert_eq!(default.torpedo_min_damage, ship_systems.torpedo_min_damage);
    }

    #[test]
    fn fire_phasers_at_shielded_hostile() {
        let player = Ship::create_federation_ship();
        let mut hostile = Ship::create_klingon_ship();

        player.fire_phasers(&mut hostile);
        player.fire_phasers(&mut hostile);

        assert_eq!(50, hostile.systems.shield_strength);
        assert_eq!(100, hostile.systems.hull_integrity);
    }

    #[test]
    fn fire_phasers_at_unshielded_hostile() {
        let player = Ship::create_federation_ship();
        let mut hostile = Ship::create_klingon_ship();
        hostile.systems.shield_strength = 0;

        player.fire_phasers(&mut hostile);
        player.fire_phasers(&mut hostile);

        assert_eq!(0, hostile.systems.shield_strength);
        assert_eq!(98, hostile.systems.hull_integrity);
    }

    #[test]
    fn fire_phasers_at_opponent_where_the_attack_will_deplete_the_hostile_target_shields() {
        let player = Ship::create_federation_ship();
        let mut hostile = Ship::create_klingon_ship();
        hostile.systems.shield_strength = 20;

        player.fire_phasers(&mut hostile);

        assert_eq!(0, hostile.systems.shield_strength);
        assert_eq!(100, hostile.systems.hull_integrity);
    }

    #[test]
    fn fire_phasers_that_destroys_opponent() {
        let mut player = Ship::create_federation_ship();
        player.systems.phaser_max_damage = 1000;
        player.systems.phaser_min_damage = 1000;
        let mut hostile = Ship::create_klingon_ship();
        hostile.systems.shield_strength = 20;

        player.fire_phasers(&mut hostile);
        player.fire_phasers(&mut hostile);

        assert_eq!(0, hostile.systems.shield_strength);
        assert_eq!(0, hostile.systems.hull_integrity);
    }

    #[test]
    fn fire_torpedoes_at_shielded_hostile() {
        let player = Ship::create_federation_ship();
        let mut hostile = Ship::create_klingon_ship();

        player.fire_torpedoes(&mut hostile);
        player.fire_torpedoes(&mut hostile);

        assert_eq!(98, hostile.systems.shield_strength);
        assert_eq!(100, hostile.systems.hull_integrity);
    }

    #[test]
    fn fire_torpedoes_at_unshielded_hostile() {
        let player = Ship::create_federation_ship();
        let mut hostile = Ship::create_klingon_ship();
        hostile.systems.shield_strength = 0;

        player.fire_torpedoes(&mut hostile);
        player.fire_torpedoes(&mut hostile);

        assert_eq!(0, hostile.systems.shield_strength);
        assert_eq!(50, hostile.systems.hull_integrity);
    }

    #[test]
    fn fire_torpedoes_at_opponent_where_the_attack_will_deplete_the_hostile_target_shields() {
        let player = Ship::create_federation_ship();
        let mut hostile = Ship::create_klingon_ship();
        hostile.systems.shield_strength = 2;

        player.fire_torpedoes(&mut hostile);
        player.fire_torpedoes(&mut hostile);

        assert_eq!(0, hostile.systems.shield_strength);
        assert_eq!(100, hostile.systems.hull_integrity);
    }

    #[test]
    fn fire_torpedoes_that_destroys_opponent() {
        let mut player = Ship::create_federation_ship();
        player.systems.torpedo_max_damage = 1000;
        player.systems.torpedo_min_damage = 1000;
        let mut hostile = Ship::create_klingon_ship();
        hostile.systems.shield_strength = 2;

        player.fire_torpedoes(&mut hostile);
        player.fire_torpedoes(&mut hostile);

        assert_eq!(0, hostile.systems.shield_strength);
        assert_eq!(0, hostile.systems.hull_integrity);
    }
}
