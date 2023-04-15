use super::star::Star;
use crate::systems::star_generation::generate_stars;
use std::fmt::Display;

pub struct Universe {
    stars: [Star; 100],
}

impl Default for Universe {
    fn default() -> Self {
        Self {
            stars: generate_stars(),
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for star in &self.stars {
            if star.is_visible {
                write!(f, "{}", star).expect("stars collection empty");
            }
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
