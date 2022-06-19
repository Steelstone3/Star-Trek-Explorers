use crate::world::planet::Planet;
use crate::world::star_classification::StarClassification;
use crate::world::star_classification::StarClassification::ClassM;

pub struct StarSystem {
    name: String,
    pub(crate) classification: StarClassification,
    planets: Vec<Planet>
}

impl StarSystem {
    pub(crate) fn generate() -> StarSystem {
        return StarSystem {
            name: String::from("Romulus"),
            classification: ClassM,
            planets: vec![Planet::generate()]
        };
    }
}

#[cfg(test)]
mod star_system_should {
    use super::*;

    #[test]
    fn create_a_star_system() {
        let star_system = StarSystem::generate();

        assert_eq!("Romulus", star_system.name);
        assert_eq!(ClassM.to_string(), star_system.classification.to_string());
        assert!(1 <= star_system.planets.len());
    }
}