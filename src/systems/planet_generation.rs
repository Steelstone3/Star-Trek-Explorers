use crate::{
    components::game_world::planet::Planet,
};

pub fn generate_planets() -> [Planet; 10] {
    let planets: [Planet; 10] = [
        Planet::default_visible_planet(),
        Planet::default(),
        Planet::default(),
        Planet::default(),
        Planet::default(),
        Planet::default(),
        Planet::default(),
        Planet::default(),
        Planet::default(),
        Planet::default(),
    ];

    planets
}

#[cfg(test)]
mod planet_generation_should {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn be_able_to_generate_planets() {
        // Given
        let stop_watch = Instant::now();

        // When
        let planets = generate_planets();

        // Then
        assert!(Duration::from_millis(10) > stop_watch.elapsed());
        assert_eq!(10, planets.len())
    }

    #[test]
    fn are_planets_visible() {
        // Given
        let mut visible_planets: Vec<bool> = Vec::new();
        let stars = generate_planets();

        // When
        for star in stars {
            if star.is_visible {
                visible_planets.push(true);
            }
        }

        // Then
        assert!(0 < visible_planets.len())
    }
}
