use super::star::Star;
use crate::systems::{random_generation::generate_seed, star_generation::generate_stars};
use std::fmt::Display;

pub struct Universe {
    stars: Vec<Star>,
}

impl Default for Universe {
    fn default() -> Self {
        Self {
            stars: generate_stars(generate_seed()),
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for star in &self.stars {
            write!(f, "{}", star).expect("stars collection empty");
        }

        Ok(())
    }
}

#[cfg(test)]
mod universe_should {
    use super::*;

    #[test]
    fn create_a_default_universe() {
        // Given
        let universe = Universe::default();

        // Then
        assert_ne!(0, universe.stars.len())
    }

    #[test]
    fn display_a_universe() {
        // Given
        let universe = Universe::default();

        // When
        let result = universe.to_string();

        // Then
        assert!(result.contains("Star: "));
        assert!(result.contains("Planet: "));
    }
}
