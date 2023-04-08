use crate::{
    components::ship::{
        damage::DamageTaker,
        hull::Hull,
        name::{faction_name::FactionName, klingon_ship_name::KlingonShipName},
        phaser::Phaser,
        shield::Shield,
        torpedo::Torpedo,
    },
    systems::ship_identifier_generation::{generate_random_identifier, generate_seed},
};
use rand::random;
use rand_derive2::RandGen;

use super::ship::Ship;

#[derive(PartialEq, Debug, RandGen)]
pub struct KlingonShip {
    name: KlingonShipName,
    ship_identifier: String,
    faction: FactionName,
    shield: Shield,
    hull: Hull,
    phaser: Phaser,
    torpedo: Torpedo,
}

impl Default for KlingonShip {
    fn default() -> Self {
        Self {
            name: random(),
            ship_identifier: generate_random_identifier(
                generate_seed(),
                FactionName::KlingonEmpire,
            ),
            faction: FactionName::KlingonEmpire,
            shield: Shield::default(),
            hull: Hull::default(),
            phaser: Phaser::default(),
            torpedo: Torpedo::default(),
        }
    }
}

impl Ship for KlingonShip {
    fn display_ship_name(&self) {
        println!(
            "| Name: {} {} | Faction: {} |",
            self.ship_identifier, self.name, self.faction
        )
    }
}

impl DamageTaker for KlingonShip {
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
mod klingon_ship_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_a_default_ship() {
        // Given
        let ship = KlingonShip::default();

        // Then
        assert_ne!(String::default(), ship.ship_identifier);
        assert_eq!(FactionName::KlingonEmpire, ship.faction);
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
    fn take_damage_to_klingon_ship(
        #[case] damage: u8,
        #[case] current_shields: u8,
        #[case] current_hull: u8,
    ) {
        // Given
        let mut ship = KlingonShip::default();

        // When
        ship.take_damage(damage);

        // Then
        assert_eq!(current_shields, ship.shield.current);
        assert_eq!(current_hull, ship.hull.current);
    }
}
