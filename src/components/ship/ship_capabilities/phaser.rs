use std::fmt::Display;

use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_derive2::RandGen;

use super::damage::DamageDealer;

#[derive(PartialEq, Debug, Copy, Clone,RandGen)]
pub struct Phaser {
    minimum_damage: u8,
    pub maximum_damage: u8,
}

impl Default for Phaser {
    fn default() -> Self {
        Self {
            minimum_damage: 5,
            maximum_damage: 10,
        }
    }
}

impl Display for Phaser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Phaser")
    }
}

impl DamageDealer for Phaser {
    fn calculate_damage(&self, seed: u64) -> u8 {
        let mut rng = StdRng::seed_from_u64(seed);
        rng.gen_range(self.minimum_damage..self.maximum_damage)
    }
}

#[cfg(test)]
mod phaser_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_a_default_phaser() {
        // Given
        let phaser = Phaser::default();

        // Then
        assert_eq!(5, phaser.minimum_damage);
        assert_eq!(10, phaser.maximum_damage);
    }

    #[test]
    fn display_a_phaser() {
        // Given
        let phaser = Phaser::default();

        // When
        let result = phaser.to_string();

        // Then
        assert_eq!("Phaser", result);
    }

    #[rstest]
    #[case(0, 9)]
    #[case(4545, 7)]
    fn calculate_damage_between_minimum_and_maximum_damage(
        #[case] seed: u64,
        #[case] expected_damage: u8,
    ) {
        // Given
        let phaser = Phaser::default();

        // When
        let damage = phaser.calculate_damage(seed);

        // Then
        assert_eq!(expected_damage, damage);
    }
}
