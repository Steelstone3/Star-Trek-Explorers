use rand::random;
use rand_derive2::RandGen;
use crate::{
    components::ship::name::{faction_name::FactionName, federation_ship_name::FederationShipName},
    entities::ship::Ship, systems::ship_identifier_generation::{generate_random_identifier, generate_seed},
};

#[derive(RandGen)]
struct FederationShip {
    ship_identifier: String,
    name: FederationShipName,
    faction: FactionName,
}

impl Default for FederationShip {
    fn default() -> Self {
        Self {
            ship_identifier: generate_random_identifier(generate_seed(), FactionName::Federation),
            name: random(),
            faction: FactionName::Federation,
        }
    }
}

impl Ship for FederationShip {
    fn display_ship_name(&self) {
        println!(
            "{} - {} | {}",
            self.ship_identifier, self.name, self.faction
        )
    }
}

#[cfg(test)]
mod federation_ship_should {
    use super::*;

    #[test]
    fn create_a_default_ship() {
        // Given
        let ship = FederationShip::default();

        // Then
        assert_ne!(String::default(), ship.ship_identifier);
        assert_eq!(FactionName::Federation, ship.faction);
    }
}
