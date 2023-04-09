use crate::{
    components::ship::{
        names::{
            faction_name::FactionName, federation_ship_class::FederationShipClass,
            federation_ship_name::FederationShipName, ship_identification::ShipIdentification,
        },
        ship_capabilities::ship_systems::ShipSystems,
    },
    systems::random_generation::generate_seed,
};
use rand::random;
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct FederationShip {
    pub name: FederationShipName,
    pub class: FederationShipClass,
    pub ship_identification: ShipIdentification,
    pub ship_systems: ShipSystems,
}

impl Default for FederationShip {
    fn default() -> Self {
        Self {
            name: random(),
            class: random(),
            ship_identification: ShipIdentification::new(generate_seed(), FactionName::Federation),
            ship_systems: ShipSystems::default(),
        }
    }
}

#[cfg(test)]
mod federation_ship_should {
    use super::*;
    use crate::components::ship::ship_capabilities::{
        hull::Hull, phaser::Phaser, shield::Shield, torpedo::Torpedo,
    };

    #[test]
    fn create_a_default_ship() {
        // Given
        let ship = FederationShip::default();

        // Then
        assert_ne!(String::default(), ship.name.to_string());
        assert_ne!(String::default(), ship.ship_identification.serial_number);
        assert_eq!(FactionName::Federation, ship.ship_identification.faction);
        assert_eq!(Shield::default(), ship.ship_systems.shield);
        assert_eq!(Hull::default(), ship.ship_systems.hull);
        assert_eq!(Phaser::default(), ship.ship_systems.phaser);
        assert_eq!(Torpedo::default(), ship.ship_systems.torpedo);
    }
}
