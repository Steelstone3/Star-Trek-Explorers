use rand_derive2::RandGen;

use super::damage::DamageTaker;

#[derive(PartialEq, Debug, RandGen)]
pub struct Hull {
    pub current: u8,
    pub maximum: u8,
    repair_rate: u8,
}

impl Default for Hull {
    fn default() -> Self {
        Self {
            current: 100,
            maximum: 100,
            repair_rate: 5,
        }
    }
}

impl DamageTaker for Hull {
    fn take_damage(&mut self, damage: u8) {
        if damage >= self.current {
            self.current = 0;
        } else {
            self.current -= damage;
        }
    }
}

#[cfg(test)]
mod hull_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_a_default_hull() {
        // Given
        let hull = Hull::default();

        // Then
        assert_eq!(100, hull.current);
        assert_eq!(100, hull.maximum);
        assert_eq!(5, hull.repair_rate);
    }

    #[rstest]
    #[case(10, 90)]
    #[case(100, 0)]
    #[case(101, 0)]
    fn take_damage_to_hull(#[case] damage: u8, #[case] current_hull: u8) {
        // Given
        let mut hull = Hull::default();

        // When
        hull.take_damage(damage);

        // Then
        assert_eq!(current_hull, hull.current)
    }
}
