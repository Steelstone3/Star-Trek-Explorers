use rand_derive2::RandGen;

use super::damage::DamageTaker;

#[derive(PartialEq, Debug, RandGen)]
pub struct Shield {
    maximum: u8,
    regeneration: u8,
    current: u8,
}

impl Default for Shield {
    fn default() -> Self {
        Self {
            maximum: 100,
            current: 100,
            regeneration: 5,
        }
    }
}

impl DamageTaker for Shield {
    fn take_damage(&mut self, damage: u8) {
        if damage >= self.current {
            self.current = 0;
        } else {
            self.current -= damage;
        }
    }
}

#[cfg(test)]
mod shield_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_a_default_shield() {
        // Given
        let shield = Shield::default();

        // Then
        assert_eq!(100, shield.current);
        assert_eq!(100, shield.maximum);
        assert_eq!(5, shield.regeneration);
    }

    #[rstest]
    #[case(10, 90)]
    #[case(100, 0)]
    #[case(101, 0)]
    fn take_damage_to_shields(#[case] damage: u8, #[case] current_shield: u8) {
        // Given
        let mut shield = Shield::default();

        // When
        shield.take_damage(damage);

        // Then
        assert_eq!(current_shield, shield.current)
    }
}
