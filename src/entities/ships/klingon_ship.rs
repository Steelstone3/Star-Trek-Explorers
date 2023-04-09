use crate::{
    components::ship::{
        names::{
            faction_name::FactionName, klingon_ship_class::KlingonShipClass,
            klingon_ship_name::KlingonShipName, ship_identification::ShipIdentification,
        },
        ship_capabilities::ship_systems::ShipSystems,
    },
    systems::random_generation::generate_seed,
};
use rand::random;
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct KlingonShip {
    pub name: KlingonShipName,
    pub class: KlingonShipClass,
    pub ship_identification: ShipIdentification,
    pub ship_systems: ShipSystems,
}

impl Default for KlingonShip {
    fn default() -> Self {
        Self {
            name: random(),
            class: random(),
            ship_identification: ShipIdentification::new(
                generate_seed(),
                FactionName::KlingonEmpire,
            ),
            ship_systems: ShipSystems::default(),
        }
    }
}

#[cfg(test)]
mod klingon_ship_should {
    use crate::components::ship::ship_capabilities::{
        hull::Hull, phaser::Phaser, shield::Shield, torpedo::Torpedo,
    };

    use super::*;

    #[test]
    fn create_a_default_ship() {
        // Given
        let ship = KlingonShip::default();

        // Then
        assert_ne!(String::default(), ship.name.to_string());
        assert_ne!(String::default(), ship.ship_identification.serial_number);
        assert_eq!(FactionName::KlingonEmpire, ship.ship_identification.faction);
        assert_eq!(Shield::default(), ship.ship_systems.shield);
        assert_eq!(Hull::default(), ship.ship_systems.hull);
        assert_eq!(Phaser::default(), ship.ship_systems.phaser);
        assert_eq!(Torpedo::default(), ship.ship_systems.torpedo);
    }
}
