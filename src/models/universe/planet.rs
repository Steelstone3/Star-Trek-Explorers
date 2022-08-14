use crate::{
    assests::names::universe::planet_classification::{PLANET_CLASSIFICATION, PLANET_NAMES},
    controllers::game::game_randomiser::{generate_seed, get_seeded_random_name},
};

pub struct Planet {
    pub display_symbol: char,
    pub name: String,
    pub classification: String,
}

impl Planet {
    pub fn create_random_planets(quantity: usize) -> Vec<Planet> {
        let mut planets: Vec<Planet> = Vec::new();

        for _ in 0..quantity {
            planets.push(Planet::create_random_planet(generate_seed()))
        }

        planets
    }

    fn create_random_planet(seed: u64) -> Planet {
        Planet {
            display_symbol: 'P',
            name: get_seeded_random_name(&PLANET_NAMES, seed),
            classification: get_seeded_random_name(&PLANET_CLASSIFICATION, seed),
        }
    }
}

#[cfg(test)]
mod planet_should {
    use super::*;

    #[test]
    fn create_random_planets() {
        const QUANTITY: usize = 100;

        let planets = Planet::create_random_planets(QUANTITY);

        assert_eq!(QUANTITY, planets.len());
    }

    #[test]
    fn create_random_planet() {
        let planet = Planet::create_random_planet(101);

        assert_eq!('P', planet.display_symbol);
        assert_eq!("Saturn", planet.name);
        assert_eq!("Class N: Classified.", planet.classification);
    }
}
