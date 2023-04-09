use super::{
    names::{star_class::StarClass, star_name::StarName},
    planet::Planet,
};
use crate::systems::{planet_generation::generate_planets, random_generation::generate_seed};
use rand::random;
use std::fmt::Display;

pub struct Star {
    name: StarName,
    class: StarClass,
    planets: Vec<Planet>,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            planets: generate_planets(generate_seed()),
            name: random(),
            class: random(),
        }
    }
}

impl Display for Star {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n| Star: {} {} |", self.name, self.class).unwrap();

        for planet in &self.planets {
            writeln!(f, "{}", planet).expect("planets collection empty");
        }

        Ok(())
    }
}

#[cfg(test)]
mod star_should {
    use super::*;

    #[test]
    fn create_a_default_star() {
        // Given
        let star = Star::default();

        // Then
        assert_ne!(0, star.planets.len());
        assert_ne!(String::default(), star.name.to_string());
        assert_ne!(String::default(), star.class.to_string());
    }

    #[test]
    fn display_a_star() {
        // Given
        let star = Star::default();

        // When
        let result = star.to_string();

        // Then
        assert!(result.contains("Star: "));
        assert!(result.contains("Planet: "));
    }
}
