use crate::{
    components::ship::{
        damage::{self, DamageTaker},
        hull::Hull,
        name::{faction_name::FactionName, federation_ship_name::FederationShipName},
        phaser::Phaser,
        shield::Shield,
        torpedo::Torpedo,
    },
    entities::ship::Ship,
    systems::ship_identifier_generation::{generate_random_identifier, generate_seed},
};
use rand::random;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub struct FederationShip {
    ship_identifier: String,
    name: FederationShipName,
    faction: FactionName,
    shield: Shield,
    hull: Hull,
    phaser: Phaser,
    torpedo: Torpedo,
}

impl Default for FederationShip {
    fn default() -> Self {
        Self {
            ship_identifier: generate_random_identifier(generate_seed(), FactionName::Federation),
            name: random(),
            faction: FactionName::Federation,
            shield: Default::default(),
            hull: Default::default(),
            phaser: Default::default(),
            torpedo: Default::default(),
        }
    }
}

impl Ship for FederationShip {
    fn display_ship_name(&self) {
        println!(
            "| Name: {} {} | Faction: {} |",
            self.ship_identifier, self.name, self.faction
        )
    }
}

impl DamageTaker for FederationShip {
    fn take_damage(&mut self, damage: u8) {
        todo!()
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
        assert_eq!(Shield::default(), ship.shield);
        assert_eq!(Hull::default(), ship.hull);
        assert_eq!(Phaser::default(), ship.phaser);
        assert_eq!(Torpedo::default(), ship.torpedo);
    }
}
