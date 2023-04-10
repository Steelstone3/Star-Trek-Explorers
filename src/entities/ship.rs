use crate::{
    components::ship::{
        names::{
            faction_name::FactionName, federation_ship_class::FederationShipClass,
            federation_ship_name::FederationShipName, klingon_ship_class::KlingonShipClass,
            klingon_ship_name::KlingonShipName, ship_identification::ShipIdentification,
        },
        ship_capabilities::ship_systems::ShipSystems,
    },
    systems::random_generation::generate_seed,
};
use rand::random;
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct Ship {
    pub name: String,
    pub class: String,
    pub ship_identification: ShipIdentification,
    pub ship_systems: ShipSystems,
}

impl Ship {
    pub fn new_federation_ship() -> Self {
        let name: FederationShipName = random();
        let class: FederationShipClass = random();

        Self {
            name: name.to_string(),
            class: class.to_string(),
            ship_identification: ShipIdentification::new(generate_seed(), FactionName::Federation),
            ship_systems: ShipSystems::default(),
        }
    }

    pub fn new_klingon_ship() -> Self {
        let name: KlingonShipName = random();
        let class: KlingonShipClass = random();

        Self {
            name: name.to_string(),
            class: class.to_string(),
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
        assert_ne!(String::default(), ship.name);
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
        assert_ne!(String::default(), ship.name);
        assert_ne!(String::default(), ship.ship_identification.serial_number);
        assert_eq!(FactionName::KlingonEmpire, ship.ship_identification.faction);
        assert_eq!(Shield::default(), ship.ship_systems.shield);
        assert_eq!(Hull::default(), ship.ship_systems.hull);
        assert_eq!(Phaser::default(), ship.ship_systems.phaser);
        assert_eq!(Torpedo::default(), ship.ship_systems.torpedo);
    }
}
