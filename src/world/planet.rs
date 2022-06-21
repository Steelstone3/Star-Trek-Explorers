use crate::names::planet_classification::PLANET_CLASSIFICATION;
use crate::names::planet_names::PLANET_NAMES;
use crate::world::moon::Moon;
use crate::names::random::get_unique_name;
use crate::names::random::get_random_name;

pub struct Planet {
    name: String,
    pub(crate) classification: String,
    pub(crate) moons: Vec<Moon>,
}

impl Planet {
    pub(crate) fn generate(used_names: &[&str], seed:u64) -> Planet {
        return Planet {
            name: get_unique_name(&PLANET_NAMES, &used_names, seed),
            classification: get_random_name(&PLANET_CLASSIFICATION, seed),
            moons: vec![]
        };
    }
}

#[cfg(test)]
mod planet_should {
    use super::*;

    #[test]
    fn create_a_planet() {
        let planet = Planet::generate(&["Earth"], 45);

        assert_eq!("Mercury", planet.name);
        assert_eq!("Class K: Adaptable for Humans by use of artificial biospheres.", planet.classification.to_string());
        assert_eq!(0, planet.moons.len());
    }
}