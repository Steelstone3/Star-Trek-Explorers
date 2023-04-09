use crate::{
    components::ship::{
        damage::DamageTaker,
        hull::Hull,
        names::{faction_name::FactionName, federation_ship_name::FederationShipName},
        phaser::Phaser,
        shield::Shield,
        torpedo::Torpedo,
    },
    systems::{
        random_generation::generate_seed, ship_identifier_generation::generate_random_identifier,
    },
};
use rand::random;
use rand_derive2::RandGen;

use super::ship::Ship;

#[derive(PartialEq, Debug, RandGen)]
pub struct FederationShip {
    serial_number: String,
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
            serial_number: generate_random_identifier(generate_seed(), FactionName::Federation),
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
        // TODO add ship class
        println!("| Name: {} {} | Class:  |", self.serial_number, self.name)
    }

    fn display_ship_name_and_faction(&self) {
        println!(
            // TODO add ship class
            "| Name: {} {} | Class:  | Faction: {} |",
            self.serial_number, self.name, self.faction
        )
    }
 
    fn display_offensive_capabilities(&self) {
        println!(
            "| Phaser Damage: {} | Torpedo Damage: {} |",
            self.phaser.maximum_damage, self.torpedo.maximum_damage
        )
    }

    fn display_defensive_capabilities(&self) {
        println!(
            "| Shield Strength: {} | Hull Integrity: {} |",
            self.shield.maximum, self.hull.maximum
        )
    }

    fn take_damage_from_hostile_ship(&mut self, damage: u8) {
        DamageTaker::take_damage(self, damage)
    }

}

impl DamageTaker for FederationShip {
    fn take_damage(&mut self, damage: u8) {
        let current_shield = self.shield.current;
        self.shield.take_damage(damage);

        if self.shield.current == 0 {
            let damage_remainder = damage - current_shield;
            self.hull.take_damage(damage_remainder);
        }
    }
}

#[cfg(test)]
mod federation_ship_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_a_default_ship() {
        // Given
        let ship = FederationShip::default();

        // Then
        assert_ne!(String::default(), ship.name.to_string());
        assert_ne!(String::default(), ship.serial_number);
        assert_eq!(FactionName::Federation, ship.faction);
        assert_eq!(Shield::default(), ship.shield);
        assert_eq!(Hull::default(), ship.hull);
        assert_eq!(Phaser::default(), ship.phaser);
        assert_eq!(Torpedo::default(), ship.torpedo);
    }

    #[rstest]
    #[case(10, 90, 100)]
    #[case(100, 0, 100)]
    #[case(101, 0, 99)]
    #[case(120, 0, 80)]
    #[case(200, 0, 0)]
    #[case(201, 0, 0)]
    fn take_damage_to_federation_ship(
        #[case] damage: u8,
        #[case] current_shields: u8,
        #[case] current_hull: u8,
    ) {
        // Given
        let mut ship = FederationShip::default();

        // When
        ship.take_damage_from_hostile_ship(damage);

        // Then
        assert_eq!(current_shields, ship.shield.current);
        assert_eq!(current_hull, ship.hull.current);
    }
}
