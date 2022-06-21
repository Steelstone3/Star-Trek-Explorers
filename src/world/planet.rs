use crate::names::planet_classification::PLANET_CLASSIFICATION;
use crate::names::planet_names::PLANET_NAMES;
use crate::names::random::get_unique_name;
use crate::names::random::get_random_name;

pub struct Planet {
    pub(crate) display_symbol: char,
    pub(crate) name: String,
    pub(crate) classification: String,
}

impl Planet {
    pub(crate) fn generate(used_names: Vec<&str>, seed:u64) -> Planet {
        return Planet {
            display_symbol: 'P',
            name: get_unique_name(PLANET_NAMES.to_vec(), used_names, seed),
            classification: get_random_name(PLANET_CLASSIFICATION.to_vec(), seed),
        };
    }
}

#[cfg(test)]
mod planet_should {
    use super::*;

    #[test]
    fn create_a_planet() {
        let planet = Planet::generate(vec!("Earth"), 45);

        assert_eq!('P', planet.display_symbol);
        assert_eq!("Mercury", planet.name);
        assert_eq!("Class K: Adaptable for Humans by use of artificial biospheres.", planet.classification.to_string());
    }
}