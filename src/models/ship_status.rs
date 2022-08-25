use rand_derive2::RandGen;

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
            shield_strength: 100,
            hull_integrity: 100,
            phaser_max_damage: 10,
            phaser_min_damage: 1,
            torpedo_max_damage: 10,
            torpedo_min_damage: 1,
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
        assert_eq!(10, ShipSystems::default().phaser_max_damage);
    }

    #[test]
    fn have_a_minimum_default_phaser_damage() {
        assert_eq!(1, ShipSystems::default().phaser_min_damage);
    }

    #[test]
    fn have_a_maximum_default_torpedo_damage() {
        assert_eq!(10, ShipSystems::default().torpedo_max_damage);
    }

    #[test]
    fn have_a_minimum_default_torpedo_damage() {
        assert_eq!(1, ShipSystems::default().torpedo_min_damage);
    }
}
