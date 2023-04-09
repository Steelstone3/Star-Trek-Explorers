use rand::random;

use super::names::{planet_class::PlanetClass, planet_name::PlanetName};

// TODO Add display planet
pub struct Planet {
    name: PlanetName,
    class: PlanetClass,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            name: random(),
            class: random(),
        }
    }
}

#[cfg(test)]
mod planet_should {
    use super::*;

    #[test]
    fn display() {
        // Given
        let planet = Planet::default();

        // Then
    }
}
