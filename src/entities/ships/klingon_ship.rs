use super::ship::Ship;
use crate::{
    components::ship::{
        names::{
            faction_name::FactionName, klingon_ship_class::KlingonShipClass,
            klingon_ship_name::KlingonShipName, ship_identification::ShipIdentification,
        },
        ship_capabilities::{damage::DamageTaker, ship_systems::ShipSystems},
    },
    systems::{
        random_generation::generate_seed, ship_identifier_generation::generate_random_identifier,
    },
};
use rand::random;
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct KlingonShip {
    name: KlingonShipName,
    class: KlingonShipClass,
    ship_identification: ShipIdentification,
    ship_systems: ShipSystems,
}

impl Default for KlingonShip {
    fn default() -> Self {
        Self {
            name: random(),
            class: random(),
            ship_identification: ShipIdentification {
                serial_number: generate_random_identifier(
                    generate_seed(),
                    FactionName::KlingonEmpire,
                ),
                faction: FactionName::KlingonEmpire,
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

impl Ship for KlingonShip {
    fn display_ship_name(&self) {
        println!(
            "| Name: {} {} | Class: {} |",
            self.ship_identification.serial_number, self.name, self.class
        )
    }

    fn display_ship_name_and_faction(&self) {
        println!(
            "| Name: {} {} | Class: {} | Faction {} |",
            self.ship_identification.serial_number,
            self.name,
            self.class,
            self.ship_identification.faction
        )
    }

    fn display_offensive_capabilities(&self) {
        println!(
            "| Phaser Damage: {} | Torpedo Damage {} |",
            self.ship_systems.phaser.maximum_damage, self.ship_systems.torpedo.maximum_damage
        )
    }

    fn display_defensive_capabilities(&self) {
        todo!()
    }

    fn take_damage_from_hostile_ship(&mut self, damage: u8) {
        DamageTaker::take_damage(self, damage)
    }
}

impl DamageTaker for KlingonShip {
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
mod klingon_ship_should {
    use crate::components::ship::ship_capabilities::{
        hull::Hull, phaser::Phaser, shield::Shield, torpedo::Torpedo,
    };

    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case(10, 90, 100)]
    #[case(100, 0, 100)]
    #[case(101, 0, 99)]
    #[case(120, 0, 80)]
    #[case(200, 0, 0)]
    #[case(201, 0, 0)]
    fn take_damage_to_klingon_ship(
        #[case] damage: u8,
        #[case] current_shields: u8,
        #[case] current_hull: u8,
    ) {
        // Given
        let mut ship = KlingonShip::default();

        // When
        ship.take_damage_from_hostile_ship(damage);

        // Then
        assert_eq!(current_shields, ship.ship_systems.shield.current);
        assert_eq!(current_hull, ship.ship_systems.hull.current);
    }
}
