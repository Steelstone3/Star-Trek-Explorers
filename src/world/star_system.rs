use crate::names::star_system_names::STAR_SYSTEM_NAMES;
use crate::names::star_classification::STAR_CLASSIFICATION;
use crate::world::planet::Planet;
use crate::names::random::get_unique_name;
use crate::names::random::get_random_name;

pub struct StarSystem {
    display_symbol: char,
    name: String,
    pub(crate) classification: String,
    planets: Vec<Planet>,
}

impl StarSystem {
    pub(crate) fn generate(used_star_names:&[&str], used_planet_names: &[&str], seed:u64) -> StarSystem {
        return StarSystem {
            display_symbol: '*',
            name: get_unique_name(&STAR_SYSTEM_NAMES, used_star_names, seed),
            classification: get_random_name(&STAR_CLASSIFICATION, seed),
            planets: vec![Planet::generate(&used_planet_names,seed)],
        };
    }
}

#[cfg(test)]
mod star_system_should {
    use super::*;

    #[test]
    fn create_a_star_system() {
        let star_system = StarSystem::generate(&["Romulus"], &["Earth"], 45);

        assert_eq!('*', star_system.display_symbol);
        assert_eq!("Sol", star_system.name);
        assert_eq!(
            "Class F: .".to_string(),
            star_system.classification.to_string()
        );
        assert!(1 <= star_system.planets.len());
    }
}
