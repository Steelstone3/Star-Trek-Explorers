use crate::names::planet_classification::PLANET_CLASSIFICATION;
use crate::names::planet_names::PLANET_NAMES;
use crate::names::random::get_seeded_random_name;

pub struct Planet {
    pub display_symbol: char,
    pub name: String,
    pub classification: String,
}

impl Planet {
    pub fn generate(seed:u64) -> Planet {
        return Planet {
            display_symbol: 'P',
            name: get_seeded_random_name(PLANET_NAMES.to_vec(), seed),
            classification: get_seeded_random_name(PLANET_CLASSIFICATION.to_vec(), seed),
        };
    }
}

#[cfg(test)]
mod planet_should {
    use super::*;

    #[test]
    fn create_a_planet() {
        let planet = Planet::generate(45);

        assert_eq!('P', planet.display_symbol);
        assert_eq!("Earth", planet.name);
        assert_eq!("Class K: Adaptable for Humans by use of artificial biospheres.", planet.classification.to_string());
    }
}