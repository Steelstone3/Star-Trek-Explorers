use super::ship::Ship;
use crate::{
    components::ship::{
        names::{
            faction_name::FactionName, federation_ship_class::FederationShipClass,
            federation_ship_name::FederationShipName, ship_identification::ShipIdentification,
        },
        ship_capabilities::{
            damage::{DamageDealer, DamageTaker},
            ship_systems::ShipSystems,
        },
    },
    systems::{
        random_generation::generate_seed, ship_identifier_generation::generate_random_identifier,
    },
};
use rand::random;
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct FederationShip {
    name: FederationShipName,
    class: FederationShipClass,
    pub ship_identification: ShipIdentification,
    pub ship_systems: ShipSystems,
}

impl Default for FederationShip {
    fn default() -> Self {
        Self {
            name: random(),
            class: random(),
            ship_identification: ShipIdentification {
                serial_number: generate_random_identifier(generate_seed(), FactionName::Federation),
                faction: FactionName::Federation,
            },

            ship_systems: ShipSystems {
                shield: Default::default(),
                hull: Default::default(),
                phaser: Default::default(),
                torpedo: Default::default(),
            },
        }
    }
}

impl Ship for FederationShip {
    fn display_ship_name(&self) {
        println!(
            "| Name: {} {} | Class: {} |",
            self.ship_identification.serial_number, self.name, self.class
        )
    }

    fn display_ship_name_and_faction(&self) {
        println!(
            "| Name: {} {} | Class: {} | Faction: {} |",
            self.ship_identification.serial_number,
            self.name,
            self.class,
            self.ship_identification.faction
        )
    }

    fn display_offensive_capabilities(&self) {
        println!(
            "| Phaser Damage: {} | Torpedo Damage: {} |",
            self.ship_systems.phaser.maximum_damage, self.ship_systems.torpedo.maximum_damage
        )
    }

    fn display_defensive_capabilities(&self) {
        println!(
            "| Shield Strength: {} | Hull Integrity: {} |",
            self.ship_systems.shield.maximum, self.ship_systems.hull.maximum
        )
    }

    fn take_damage_from_hostile_ship(&mut self, damage: u8) {
        DamageTaker::take_damage(self, damage)
    }

    fn calculate_damage_from_weapon(&self, seed: u64, weapon_name: String) -> u8 {
        if weapon_name == self.ship_systems.phaser.to_string() {
            self.ship_systems.phaser.calculate_damage(seed)
        } else {
            self.ship_systems.torpedo.calculate_damage(seed)
        }
    }
}

impl DamageTaker for FederationShip {
    fn take_damage(&mut self, damage: u8) {
        let current_shield = self.ship_systems.shield.current;
        self.ship_systems.shield.take_damage(damage);

        if self.ship_systems.shield.current == 0 {
            let damage_remainder = damage - current_shield;
            self.ship_systems.hull.take_damage(damage_remainder);
        }
    }
}

#[cfg(test)]
mod federation_ship_should {
    use crate::components::ship::ship_capabilities::{
        hull::Hull, phaser::Phaser, shield::Shield, torpedo::Torpedo,
    };

    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case("Phaser", 0, 9)]
    #[case("Phaser", 4545, 7)]
    #[case("Phaser", 7001, 5)]
    #[case("Torpedo", 0, 9)]
    #[case("Torpedo", 4545, 7)]
    #[case("Torpedo", 7001, 5)]
    fn calculate_damage_for_weapon(
        #[case] weapon_name: String,
        #[case] seed: u64,
        #[case] expected_damage: u8,
    ) {
        // Given
        let ship = FederationShip::default();

        // When
        let damage = ship.calculate_damage_from_weapon(seed, weapon_name);

        // Then
        assert_eq!(expected_damage, damage);
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
        assert_eq!(current_shields, ship.ship_systems.shield.current);
        assert_eq!(current_hull, ship.ship_systems.hull.current);
    }
}
