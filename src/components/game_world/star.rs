use crate::systems::{planet_generation::generate_planets, random_generation::generate_seed};

use super::planet::Planet;

pub struct Star {
    planets: Vec<Planet>,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            planets: generate_planets(generate_seed()),
        }
    }
}

#[cfg(test)]
mod star_should {
    use super::*;

    #[test]
    fn create_a_default_universe() {
        // Given
        let star = Star::default();

        // Then
        assert_ne!(0, star.planets.len())
    }
}
