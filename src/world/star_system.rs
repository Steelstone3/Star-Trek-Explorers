use crate::names::random::get_random_number_from_range;
use crate::names::random::get_seeded_random_name;
use crate::names::star_classification::STAR_CLASSIFICATION;
use crate::names::star_system_names::STAR_SYSTEM_NAMES;
use crate::world::planet::Planet;

pub struct StarSystem {
    pub display_symbol: char,
    pub name: String,
    pub classification: String,
    pub planets: Vec<Planet>,
}

impl StarSystem {
    pub fn generate(seed: u64) -> StarSystem {
        return StarSystem {
            display_symbol: '*',
            name: get_seeded_random_name(STAR_SYSTEM_NAMES.to_vec(), seed),
            classification: get_seeded_random_name(STAR_CLASSIFICATION.to_vec(), seed),
            planets: create_distinct_random_planets(get_random_number_from_range(1, 64)),
        };
    }
}

fn create_distinct_random_planets(quantity: u64) -> Vec<Planet> {
    let mut planets: Vec<Planet> = Vec::new();
    for _ in 0..quantity {
        let mut is_unique = true;
        let new_planet = Planet::generate(get_random_number_from_range(1, 1000000));

        for planet in &planets {
            if planet.name == new_planet.name {
                is_unique = false;
            }
        }

        if is_unique {
            planets.push(new_planet);
        }
    }

    return planets;
}

#[cfg(test)]
mod star_system_should {
    use super::*;
    use crate::names::planet_names::PLANET_NAMES;

    #[test]
    fn create_a_star_system() {
        let star_system = StarSystem::generate(45);

        assert_eq!('*', star_system.display_symbol);
        assert_eq!("Sol", star_system.name);
        assert_eq!(
            "Class F: .".to_string(),
            star_system.classification.to_string()
        );
        assert!(1 <= star_system.planets.len());
    }

    #[test]
    fn create_random_planets_with_distinct_names() {
        let planets = create_distinct_random_planets(u64::MAX);

        assert_eq!(PLANET_NAMES.len(), planets.len());
    }
}
