use super::star::Star;
use crate::systems::{random_generation::generate_seed, star_generation::generate_stars};

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
}
