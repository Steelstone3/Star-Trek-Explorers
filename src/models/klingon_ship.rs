use super::ship_status::ShipSystems;
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
    fn be_part_of_the_klingon_empire_faction() {
        assert_eq!(
            Faction::KlingonEmpire,
            KlingonShip::default().faction
        );
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
