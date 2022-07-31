use crate::assests::names::universe::star_classification::STAR_CLASSIFICATION;
use crate::assests::names::universe::star_classification::STAR_SYSTEM_NAMES;
use crate::controllers::game_randomiser::get_random_number_from_range;
use crate::controllers::game_randomiser::get_seeded_random_name;
use crate::controllers::game_randomiser::RANDOM_LOWER_RANGE;
use crate::controllers::game_randomiser::RANDOM_UPPER_RANGE;
use crate::models::universe::planet::Planet;

pub struct StarSystem {
    pub display_symbol: char,
    pub name: String,
    pub classification: String,
    pub planets: Vec<Planet>,
}

impl StarSystem {
    pub fn create_random_star_systems(quantity: usize) -> Vec<StarSystem> {
        let mut star_systems: Vec<StarSystem> = Vec::new();

        for _ in 0..quantity {
            star_systems.push(StarSystem::create_random_star_system(
                get_random_number_from_range(RANDOM_LOWER_RANGE, RANDOM_UPPER_RANGE),
            ))
        }

        star_systems
    }

    fn create_random_star_system(seed: u64) -> StarSystem {
        StarSystem {
            display_symbol: '*',
            name: get_seeded_random_name(&STAR_SYSTEM_NAMES, seed),
            classification: get_seeded_random_name(&STAR_CLASSIFICATION, seed),
            planets: Planet::create_random_planets(get_random_number_from_range(1, 64) as usize),
        }
    }
}

#[cfg(test)]
mod star_system_should {
    use super::*;

    #[test]
    fn create_random_planets() {
        const QUANTITY: usize = 100;

        let star_systems = StarSystem::create_random_star_systems(QUANTITY);

        assert_eq!(QUANTITY, star_systems.len());
    }

    #[test]
    fn create_random_star_system() {
        let star_system = StarSystem::create_random_star_system(45);

        assert_eq!('*', star_system.display_symbol);
        assert_eq!("Beta Renner system", star_system.name);
        assert_eq!("Class F: .", star_system.classification);
        assert!(!star_system.planets.is_empty());
    }
}
