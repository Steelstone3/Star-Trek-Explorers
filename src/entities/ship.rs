use crate::{
    components::ship::{
        names::{
            faction_name::FactionName,
            ship_class::{get_random_federation_class, get_random_klingon_class, ShipClass},
            ship_identification::ShipIdentification,
            ship_name::{get_random_federation_name, get_random_klingon_name, ShipName},
        },
        ship_capabilities::ship_systems::ShipSystems,
    },
    systems::random_generation::generate_seed,
};
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct Ship {
    pub name: ShipName,
    pub class: ShipClass,
    pub ship_identification: ShipIdentification,
    pub ship_systems: ShipSystems,
}

impl Ship {
    pub fn new_federation_ship() -> Self {
        Self {
            name: get_random_federation_name(),
            class: get_random_federation_class(),
            ship_identification: ShipIdentification::new(generate_seed(), FactionName::Federation),
            ship_systems: ShipSystems::default(),
        }
    }

    pub fn new_klingon_ship() -> Self {
        Self {
            name: get_random_klingon_name(),
            class: get_random_klingon_class(),
            ship_identification: ShipIdentification::new(
                generate_seed(),
                FactionName::KlingonEmpire,
            ),
            ship_systems: ShipSystems::default(),
        }
    }
}

#[cfg(test)]
mod ship_should {
    use super::*;
    use crate::components::ship::ship_capabilities::{
        hull::Hull, phaser::Phaser, shield::Shield, torpedo::Torpedo,
    };

    #[test]
    fn create_a_default_ship() {
        // Given
        let ship = Ship::new_federation_ship();

        // Then
        assert_ne!(String::default(), ship.ship_identification.serial_number);
        assert_eq!(FactionName::Federation, ship.ship_identification.faction);
        assert_eq!(Shield::default(), ship.ship_systems.shield);
        assert_eq!(Hull::default(), ship.ship_systems.hull);
        assert_eq!(Phaser::default(), ship.ship_systems.phaser);
        assert_eq!(Torpedo::default(), ship.ship_systems.torpedo);
    }

    #[test]
    fn create_a_default_ship_2() {
        // GWiven
        let ship = Ship::new_klingon_ship();

        // Then
        assert_ne!(String::default(), ship.ship_identification.serial_number);
        assert_eq!(FactionName::KlingonEmpire, ship.ship_identification.faction);
        assert_eq!(Shield::default(), ship.ship_systems.shield);
        assert_eq!(Hull::default(), ship.ship_systems.hull);
        assert_eq!(Phaser::default(), ship.ship_systems.phaser);
        assert_eq!(Torpedo::default(), ship.ship_systems.torpedo);
    }
}
