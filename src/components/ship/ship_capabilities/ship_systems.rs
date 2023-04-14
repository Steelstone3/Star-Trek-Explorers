use crate::{
    presenters::ship_presenter::select_weapon,
    systems::random_generation::{generate_random_value_from_range_u8, generate_seed},
};

use super::{
    damage::{DamageDealer, DamageTaker},
    hull::Hull,
    phaser::Phaser,
    shield::Shield,
    torpedo::Torpedo,
};
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen, Copy, Clone, Default)]
pub struct ShipSystems {
    pub shield: Shield,
    pub hull: Hull,
    pub phaser: Phaser,
    pub torpedo: Torpedo,
}

impl ShipSystems {
    pub fn select_ship_weapon_type(&self) -> String {
        let weapon_types = vec![self.phaser.to_string(), self.torpedo.to_string()];
        select_weapon(weapon_types)
    }

    pub fn select_ship_weapon_type_ai(&self) -> String {
        let value = generate_random_value_from_range_u8(generate_seed(), 0, 1);

        if value == 1 {
            self.phaser.to_string()
        } else {
            self.torpedo.to_string()
        }
    }

    pub fn calculate_damage_from_weapon(&self, seed: u64, weapon_name: String) -> u8 {
        if weapon_name == self.phaser.to_string() {
            self.phaser.calculate_damage(seed)
        } else {
            self.torpedo.calculate_damage(seed)
        }
    }

    pub fn take_damage_from_hostile_ship(&mut self, damage: u8) {
        DamageTaker::take_damage(self, damage)
    }

    pub fn display_offensive_capabilities(&self) {
        println!(
            "| Phaser Damage: {} | Torpedo Damage {} |",
            self.phaser.maximum_damage, self.torpedo.maximum_damage
        )
    }

    pub fn display_defensive_capabilities(&self) {
        println!(
            "| Shield Strength: {} | Hull Integrity {} |",
            self.shield.maximum, self.hull.maximum
        )
    }
}

impl DamageTaker for ShipSystems {
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
mod ship_systems_should {
    use super::*;
    use rstest::rstest;

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
        let ship_systems = ShipSystems::default();

        // When
        let damage = ship_systems.calculate_damage_from_weapon(seed, weapon_name);

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
    fn take_damage_to_klingon_ship(
        #[case] damage: u8,
        #[case] current_shields: u8,
        #[case] current_hull: u8,
    ) {
        // Given
        let mut ship_systems = ShipSystems::default();

        // When
        ship_systems.take_damage_from_hostile_ship(damage);

        // Then
        assert_eq!(current_shields, ship_systems.shield.current);
        assert_eq!(current_hull, ship_systems.hull.current);
    }
}
