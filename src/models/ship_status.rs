use rand_derive2::RandGen;

const SHIELD_STRENGTH: u32 = 100;
const HULL_INTEGRITY: u32 = 100;
const PHASER_MAX_DAMAGE: u32 = 5;
const PHASER_MIN_DAMAGE: u32 = 1;
const TORPEDO_MAX_DAMAGE: u32 = 5;
const TORPEDO_MIN_DAMAGE: u32 = 1;

#[derive(RandGen)]
pub struct ShipSystems {
    pub shield_strength: u32,
    pub hull_integrity: u32,
    pub phaser_max_damage: u32,
    pub phaser_min_damage: u32,
    pub torpedo_max_damage: u32,
    pub torpedo_min_damage: u32,
}

impl ShipSystems {
    pub fn new() -> Self {
        ShipSystems {
            shield_strength: SHIELD_STRENGTH,
            hull_integrity: HULL_INTEGRITY,
            phaser_max_damage: PHASER_MAX_DAMAGE,
            phaser_min_damage: PHASER_MIN_DAMAGE,
            torpedo_max_damage: TORPEDO_MAX_DAMAGE,
            torpedo_min_damage: TORPEDO_MIN_DAMAGE,
        }
    }
}

impl Default for ShipSystems {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod ship_systems_should {
    use super::*;

    #[test]
    fn have_a_maximum_default_shield_strength() {
        assert_eq!(100, ShipSystems::default().shield_strength);
    }

    #[test]
    fn have_a_maximum_default_hull_integrity() {
        assert_eq!(100, ShipSystems::default().hull_integrity);
    }

    #[test]
    fn have_a_maximum_default_phaser_damage() {
        assert_eq!(5, ShipSystems::default().phaser_max_damage);
    }

    #[test]
    fn have_a_minimum_default_phaser_damage() {
        assert_eq!(1, ShipSystems::default().phaser_min_damage);
    }

    #[test]
    fn have_a_maximum_default_torpedo_damage() {
        assert_eq!(5, ShipSystems::default().torpedo_max_damage);
    }

    #[test]
    fn have_a_minimum_default_torpedo_damage() {
        assert_eq!(1, ShipSystems::default().torpedo_min_damage);
    }
}
