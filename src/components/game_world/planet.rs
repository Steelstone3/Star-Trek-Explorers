use super::names::{planet_class::PlanetClass, planet_name::PlanetName};
use rand::random;
use std::fmt::Display;

pub struct Planet {
    name: PlanetName,
    class: PlanetClass,
    pub is_visible: bool,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            name: random(),
            class: random(),
            is_visible: random(),
        }
    }
}

impl Planet {
    pub fn default_visible_planet() -> Self {
        Self {
            name: random(),
            class: random(),
            is_visible: true,
        }
    }
}

impl Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_visible {
            write!(f, "| Planet: {} | {} |", self.name, self.class).unwrap()
        }

        Ok(())
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
    fn create_a_visible_default_planet() {
        // Given
        let planet = Planet::default_visible_planet();

        // Then
        assert_ne!(String::default(), planet.name.to_string());
        assert_ne!(String::default(), planet.class.to_string());
        assert!(planet.is_visible);
    }
}
