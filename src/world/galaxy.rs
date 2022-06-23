use crate::names::random::get_random_number_from_range;
use crate::world::star_system::StarSystem;

pub struct Galaxy {
    pub star_systems: Vec<StarSystem>,
}

impl Galaxy {
    pub fn generate(size: u64) -> Galaxy {
        return Galaxy {
            star_systems: create_distinct_random_star_systems(size),
        };
    }
}

fn create_distinct_random_star_systems(quantity: u64) -> Vec<StarSystem> {
    let mut star_systems: Vec<StarSystem> = Vec::new();
    for _ in 0..quantity {
        let mut is_unique = true;
        let new_star_system = StarSystem::generate(get_random_number_from_range(1, 1000000));

        for star_system in &star_systems {
            if star_system.name == new_star_system.name {
                is_unique = false;
            }
        }

        if is_unique {
            star_systems.push(new_star_system);
        }
    }

    return star_systems;
}

#[cfg(test)]
mod galaxy_should {
    use super::*;
    use crate::names::star_system_names::STAR_SYSTEM_NAMES;

    #[test]
    fn create_a_galaxy() {
        let galaxy = Galaxy::generate(2500);
        assert_eq!(STAR_SYSTEM_NAMES.len(), galaxy.star_systems.len());
    }

    #[test]
    fn create_random_star_systems_with_distinct_names() {
        let star_systems = create_distinct_random_star_systems(2500);

        assert_eq!(STAR_SYSTEM_NAMES.len(), star_systems.len());
    }
}
