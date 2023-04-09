use super::names::{planet_class::PlanetClass, planet_name::PlanetName};
use rand::random;
use std::fmt::Display;

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

impl Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "| Planet: {} | {} |", self.name, self.class)
    }
}

#[cfg(test)]
mod planet_should {
    use super::*;

    #[test]
    fn create_a_default_planet() {
        // Given
        let planet = Planet::default();

        // Then
        assert_ne!(String::default(), planet.name.to_string());
        assert_ne!(String::default(), planet.class.to_string());
    }

    #[test]
    fn display_a_planet() {
        // Given
        let planet = Planet::default();

        // When
        let result = planet.to_string();

        // Then
        assert!(result.contains("Planet: "))
    }
}
