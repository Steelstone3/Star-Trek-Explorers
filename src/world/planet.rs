use crate::world::moon::Moon;
use crate::world::planet_classification::PlanetClassification;
use crate::world::planet_classification::PlanetClassification::ClassM;

pub struct Planet {
    name: String,
    pub(crate) classification: PlanetClassification,
    pub moons: Vec<Moon>,
}

impl Planet {
    pub(crate) fn generate() -> Planet {
        return Planet {
            name: String::from("Earth"),
            classification: ClassM,
            moons: vec![Moon::generate()]
        };
    }
}

#[cfg(test)]
mod planet_should {
    use crate::world::planet_classification::PlanetClassification::ClassM;
    use super::*;

    #[test]
    fn create_a_planet() {
        let planet = Planet::generate();

        assert_eq!("Earth", planet.name);
        assert_eq!(ClassM.to_string(), planet.classification.to_string());
        assert!(1 <= planet.moons.len());
    }
}